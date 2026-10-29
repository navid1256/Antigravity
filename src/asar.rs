use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

/// Parsed asar header: the file tree plus the offset where the data section starts.
struct AsarHeader {
    tree: serde_json::Value,
    base_offset: u64,
}

fn read_header(file: &mut File) -> Option<AsarHeader> {
    let mut header_size_bytes = [0u8; 16];
    file.read_exact(&mut header_size_bytes).ok()?;

    let header_size = u32::from_le_bytes([
        header_size_bytes[4],
        header_size_bytes[5],
        header_size_bytes[6],
        header_size_bytes[7],
    ]);
    let header_string_size = u32::from_le_bytes([
        header_size_bytes[12],
        header_size_bytes[13],
        header_size_bytes[14],
        header_size_bytes[15],
    ]);

    let mut header_bytes = vec![0u8; header_string_size as usize];
    file.read_exact(&mut header_bytes).ok()?;

    let header_json = String::from_utf8(header_bytes).ok()?;
    let tree: serde_json::Value = serde_json::from_str(&header_json).ok()?;

    Some(AsarHeader {
        tree,
        base_offset: (header_size + 8) as u64,
    })
}

/// Reads a single file out of an asar archive without unpacking it.
/// `rel_path` uses forward slashes, e.g. "dist/main.js".
pub fn read_asar_entry(app_asar: &Path, rel_path: &str) -> Option<Vec<u8>> {
    let mut file = File::open(app_asar).ok()?;
    let header = read_header(&mut file)?;

    let mut node = &header.tree;
    for part in rel_path.split('/') {
        node = node.get("files")?.get(part)?;
    }

    // Directories have no payload, and unpacked entries live on disk instead.
    if node.get("files").is_some() || node.get("unpacked").and_then(|u| u.as_bool()) == Some(true) {
        return None;
    }

    let size = node.get("size").and_then(|s| s.as_u64())?;
    let offset = node
        .get("offset")
        .and_then(|o| o.as_str())
        .and_then(|o| o.parse::<u64>().ok())?;

    file.seek(SeekFrom::Start(header.base_offset + offset))
        .ok()?;
    let mut buf = vec![0u8; size as usize];
    file.read_exact(&mut buf).ok()?;
    Some(buf)
}

fn extract_entry(
    file: &mut File,
    base_offset: u64,
    entry: &serde_json::Value,
    current_path: &Path,
) -> Result<(), std::io::Error> {
    if let Some(files) = entry.get("files") {
        if let Some(obj) = files.as_object() {
            for (name, child) in obj {
                let next_path = current_path.join(name);
                extract_entry(file, base_offset, child, &next_path)?;
            }
        }
        return Ok(());
    }

    let size = entry.get("size").and_then(|s| s.as_u64()).unwrap_or(0);
    let offset_str = entry.get("offset").and_then(|o| o.as_str()).unwrap_or("0");
    let offset = offset_str.parse::<u64>().unwrap_or(0);
    let unpacked = entry
        .get("unpacked")
        .and_then(|u| u.as_bool())
        .unwrap_or(false);

    if let Some(parent) = current_path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !unpacked {
        file.seek(SeekFrom::Start(base_offset + offset))?;
        let mut out_file = File::create(current_path)?;
        let mut remaining = size;
        let mut buffer = [0u8; 65536];
        while remaining > 0 {
            let to_read = std::cmp::min(remaining, buffer.len() as u64) as usize;
            file.read_exact(&mut buffer[..to_read])?;
            out_file.write_all(&buffer[..to_read])?;
            remaining -= to_read as u64;
        }
    }
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(&entry.path(), &dest_path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    /// Locates a real Antigravity Desktop install, if one is present. These
    /// tests assert against the shipping archive rather than a fixture, so they
    /// skip cleanly on machines without the app.
    fn installed_asar() -> Option<std::path::PathBuf> {
        let local = env::var("LOCALAPPDATA").ok()?;
        let p = Path::new(&local)
            .join("Programs")
            .join("Antigravity")
            .join("resources")
            .join("app.asar");
        p.exists().then_some(p)
    }

    #[test]
    fn reads_a_file_out_of_the_archive_without_unpacking() {
        let Some(asar) = installed_asar() else { return };
        let pkg = read_asar_entry(&asar, "package.json").expect("package.json in app.asar");
        let text = String::from_utf8(pkg).expect("utf-8");
        assert!(text.contains("\"name\": \"antigravity\""), "got: {}", text);

        let main = read_asar_entry(&asar, "dist/main.js").expect("dist/main.js in app.asar");
        assert!(!main.is_empty());
    }

    #[test]
    fn missing_entries_return_none() {
        let Some(asar) = installed_asar() else { return };
        assert!(read_asar_entry(&asar, "dist/does-not-exist.js").is_none());
        // A directory node carries no payload.
        assert!(read_asar_entry(&asar, "dist").is_none());
    }
}

/// Unpacks app.asar into app_dir and renames the archive to app.asar.bak.
/// Electron prefers resources/app over resources/app.asar, so a half-finished
/// extraction would leave a broken install - on any error the partial output is
/// removed and the archive is left untouched.
pub fn extract_asar(app_asar: &Path, app_dir: &Path) -> bool {
    if !app_asar.exists() || app_dir.exists() {
        return true;
    }

    let mut file = match File::open(app_asar) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let header = match read_header(&mut file) {
        Some(h) => h,
        None => return false,
    };

    if extract_entry(&mut file, header.base_offset, &header.tree, app_dir).is_err() {
        let _ = fs::remove_dir_all(app_dir);
        return false;
    }

    let asar_unpacked_path = app_asar.with_extension("asar.unpacked");
    if asar_unpacked_path.exists() && asar_unpacked_path.is_dir() {
        if copy_dir_all(&asar_unpacked_path, app_dir).is_err() {
            let _ = fs::remove_dir_all(app_dir);
            return false;
        }
    }

    // Drop the file handle before renaming the archive it points at.
    drop(file);

    let bak = app_asar.with_extension("asar.bak");
    if fs::rename(app_asar, bak).is_err() {
        let _ = fs::remove_dir_all(app_dir);
        return false;
    }

    true
}
