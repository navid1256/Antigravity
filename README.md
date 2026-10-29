# Antigravity Unlocker · English Edition

[![GitHub stars](https://img.shields.io/github/stars/navid1256/Antigravity-English?style=flat)](https://github.com/navid1256/Antigravity-English/stargazers)
[![Windows x64](https://img.shields.io/badge/download-Windows_x64-0078D4)](https://github.com/navid1256/Antigravity-English/releases/tag/v2.28.0-en.1)
[![Interface: English](https://img.shields.io/badge/interface-English-2ea44f)](#why-this-edition)

**Antigravity Unlocker, with an English interface and a ready-to-run Windows download.**

A community edition of [Antigravity Unlocker](https://github.com/confeden/Antigravity) for Antigravity 2.0, Antigravity IDE, and Antigravity CLI. It patches installed clients and configures targeted DNS/proxy routing to address regional sign-in errors, including error 400, “User location is not supported.” Availability still depends on the account, service, and working routes.

## Download for Windows

### [Download the English EXE — Windows 10/11 x64](https://github.com/navid1256/Antigravity-English/releases/download/v2.28.0-en.1/Antigravity-English-2.28.0-en.1-windows-x64.exe)

**About 10.3 MiB · Preview `v2.28.0-en.1` · No Rust or Cargo installation needed to run.**

[Release notes](https://github.com/navid1256/Antigravity-English/releases/tag/v2.28.0-en.1) · [SHA-256 checksums](https://github.com/navid1256/Antigravity-English/releases/download/v2.28.0-en.1/SHA256SUMS.txt) · [Report an issue](https://github.com/navid1256/Antigravity-English/issues)

This preview includes the English translation and the IDE shared-CLI discovery fix. The Windows build and focused tests pass; successful IDE sign-in and a model response after this fix have **not yet been verified**. The executable is unsigned.

## Why this edition?

- **English throughout:** GUI, terminal interface, diagnostic messages, reports, and documentation.
- **Better Windows discovery:** finds the shared `agy.exe` that an IDE extension can launch from `%USERPROFILE%/.gemini/bin`.
- **Download and run:** a Windows executable is attached to this repository's release.
- **Visible controls:** inspect detected installations, enable the setup, and undo individual changes using the switches.
- **Automatic patching:** can reapply client patches after Antigravity updates.

The translation changes the unlocker's language. Antigravity's own interface language is configured separately.

## Get started

1. Install Antigravity from the [official website](https://antigravity.google/download).
2. Download the English EXE above, then close Antigravity IDE and any older unlocker window.
3. Right-click the EXE and choose **Run as administrator**. Administrator rights are required for the Windows service and system routing setup.
4. Enter the upstream **2.28.0** key from the pinned messages in [@nova_txt](https://t.me/nova_txt/69864). The English release suffix does not change the application's underlying version.
5. Check the detected installations and click **Enable all**. Antigravity may close briefly while its files are patched.
6. Reopen Antigravity, sign in, and send a chat message. A successful model response is the useful end-to-end check; detection or a successful patch alone does not prove connectivity.

Use the switches to undo the corresponding changes. Under **Advanced settings**, you can also add a custom IPv4 DNS server.

## Antigravity IDE and the shared CLI

Antigravity CLI is the command-line application whose executable is `agy.exe`. An IDE extension can also run it in the background:

```text
%USERPROFILE%/.gemini/bin/agy.exe --hub
```

Earlier builds missed this folder during Windows discovery. This edition checks it automatically and lists the executable as an **Antigravity CLI** installation. Patching the IDE's bundled Language Server alone leaves this separate executable unchanged.

If the CLI is missing from the list, use the installation's pencil button to add your profile's `.gemini/bin` folder. Then apply the setup and retry the IDE. Finding an unpatched CLI does not by itself establish the cause of an account eligibility error.

**Gemini CLI is a separate application.** This project patches Antigravity clients; it makes no claim that Gemini CLI is disabled or unavailable. The `.gemini` folder name alone does not identify an executable as Gemini CLI.

## Terminal interface

The same English executable includes a terminal interface. From PowerShell in the download folder:

```powershell
./Antigravity-English-2.28.0-en.1-windows-x64.exe --tui
```

Enter the key, use the arrow keys to select an option, press Space to toggle it, and press `q` to quit. Run PowerShell as administrator when applying the Windows service or routing changes.

The repository's `tui.ps1` and `tui.sh` download upstream binaries. Use the English EXE above or a local source build for this edition's interface.

## Build from source

Rust and Cargo are required **only if you want to compile the project yourself**. Windows builds also need the MSVC C++ build tools and a Windows SDK.

```sh
cargo build --locked --release
```

Output: `target/release/ag_unlocker.exe` on Windows, or `target/release/ag_unlocker` on Linux. For the terminal interface, add `--tui` when launching it.

Linux source is included; this release provides a Windows x64 binary only. See the [Linux notes](linux/readme.md) for the existing Linux packaging workflow.

### Cargo is not recognized

After installing Rust, reopen PowerShell and restart the editor if using its integrated terminal. To refresh the default Cargo path for the current session:

```powershell
$env:Path = "$env:USERPROFILE/.cargo/bin;$env:Path"
cargo --version
```

### Build warnings and public-source limitations

- `release canary` is a build/provenance identifier.
- `private routes: relay=false exits=false group=false` means the corresponding upstream private route assets/configuration are absent. They are not included in this release.
- `unused import` and `function ... is never used` warnings did not prevent the Windows executable from building.

A successful build does not verify sign-in, route availability, or chat. The public source does not include the maintainer's private exit list or Group proxy server configuration; available alternatives depend on the local setup.

## What changes on your system?

| Component | Change |
| --- | --- |
| Installed Language Server / CLI | Equal-length replacements of `ineligible` → `inexigible` and `https_proxy` → `AG_LS_PROXY`; file size stays the same. Includes the discovered shared Antigravity CLI. |
| Windows local proxy/service | Uses `%ProgramData%/AGUnlocker` and `127.0.0.1:53129`. The custom `AG_LS_PROXY` variable points to it. |
| DNS rules (NRPT) | Target `cloudcode-pa.googleapis.com` and `daily-cloudcode-pa.googleapis.com`. Other domains retain their usual DNS path. |
| IPv4 priority | Applied when there is no global IPv6 connectivity; removed when the corresponding setup is undone. |

The implementation does not set the system-wide `HTTPS_PROXY`, decrypt TLS, or install certificates. Use the application's switches to undo its setup.

## Need help?

[Open an issue](https://github.com/navid1256/Antigravity-English/issues) with your Windows and Antigravity versions, the unlocker version, the exact error, and steps to reproduce it. **Save report** provides diagnostics; review the contents before sharing them publicly.

Verification for this preview (2026-10-29): the discovery regression and install-root tests pass, read-only inspection finds the shared CLI, and the executable starts with `--version`. Full IDE sign-in and chat verification is still pending. Reports of successful or failed setups are welcome.

If this English edition helped you, **[give the repository a star](https://github.com/navid1256/Antigravity-English)** so you can find it again and help others discover it.

## Credits and license

Based on [confeden/Antigravity](https://github.com/confeden/Antigravity). Original package author: Brent, [@nova_txt](https://t.me/nova_txt). English edition maintained at [navid1256/Antigravity-English](https://github.com/navid1256/Antigravity-English).

[Upstream video guide](https://youtu.be/aybWT_7wtqA) · [Upstream community](https://t.me/nova_txt) · [Donate to the upstream project](https://nova-app.eu/donate)

The original **Antigravity Non-Commercial & Restricted Use License** is retained. See [LICENSE](LICENSE) for its terms.
