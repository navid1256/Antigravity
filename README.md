# Antigravity Unlocker — English edition

A Windows and Linux patcher for installed copies of Antigravity 2.0, Antigravity IDE, and Antigravity CLI. It applies client patches and configures targeted DNS/proxy routing to address regional sign-in restrictions and error 400, "User location is not supported". Automatic patching can reapply the patches after Antigravity updates.

This edition translates the unlocker's GUI, terminal interface, diagnostic messages, reports, and documentation into English. It does not change the language of Antigravity itself.

Video guide: https://youtu.be/aybWT_7wtqA

## Build the English version

Building from source requires Rust and Cargo. On Windows, the MSVC C++ build tools and a Windows SDK are also required. These tools are needed to build the executable; running an already-built unlocker does not require Rust or Cargo.

From the project folder, run:

```sh
cargo build --locked --release
```

The output is `target/release/ag_unlocker.exe` on Windows or `target/release/ag_unlocker` on Linux.

An alternate Windows executable already built in this working copy is `target/release/ag_unlocker_ide_fix.exe`. It includes the IDE CLI-discovery fix described below. The normal build command above also includes this fix, but outputs the standard `ag_unlocker.exe` filename.

The release downloads and one-command launchers below fetch upstream binaries; those binaries do not include this checkout's translations or local fixes until built and published from it.

## Installation

1. Install Antigravity from the [official website](https://antigravity.google/download). For this English edition, use the executable built above. The original unlocker's binaries are available in upstream [Releases](https://github.com/confeden/Antigravity/releases).
2. Run the unlocker **as administrator** (right-click the file). Without administrator rights, only the client patch is installed; the service that bypasses error 400, "User location is not supported", is not installed.
3. Enter the free key from the pinned messages in the [t.me/nova_txt](https://t.me/nova_txt/69864) group. Each version has its own key: after updating the unlocker, you need a new one.
4. Click **«Enable all»** (Antigravity will close for a few seconds).
5. Launch Antigravity, sign in, and send a chat message. The card at the top of the window will turn green: **«Working. The model responded …»**.

You can turn off any switch to undo that part of the setup.

Custom DNS: enter your own DNS server (IPv4) under "Advanced settings". It joins the same pool as the other servers, and every answer is checked against a reference resolver, so a server that does not substitute addresses does not disrupt the setup.

## Antigravity IDE and the shared CLI

CLI means the command-line version of Antigravity, whose executable is `agy.exe`. An IDE extension can also run this executable in the background with `--hub` to provide a local service. In the inspected Windows installation, the IDE extension launches:

```text
%USERPROFILE%\.gemini\bin\agy.exe --hub
```

Patching the IDE's bundled Language Server does not patch this separate executable. Earlier builds of this checkout missed its folder during automatic discovery. Windows discovery now also checks `%USERPROFILE%\.gemini\bin`; if `agy.exe` is present there, it is included as an **Antigravity CLI** installation in patching and inspection.

To apply the updated build:

1. Close Antigravity IDE and any older unlocker window.
2. Run the updated English executable as administrator. In this working copy, the separately built executable is `target/release/ag_unlocker_ide_fix.exe`.
3. Check that the **Antigravity CLI** installation is detected. If necessary, use its pencil button to add the full path to your user profile's `.gemini\bin` folder manually.
4. Click **Enable all** if offered, then reopen the IDE and retry sign-in.

Verification status for this fix (2026-10-29): the discovery regression test and install-root tests pass, read-only inspection detects the shared CLI, and the updated executable builds and runs with `--version`. Successful account sign-in and a model response after applying this fix have **not yet been verified**. Finding an unpatched CLI does not by itself prove that it caused a particular sign-in error.

## With or without a VPN

No configuration is needed, with a VPN or a third-party DNS server. Antigravity's connections to the two Google servers go through a service on your computer, outside the VPN. Routes are chosen based on **actual model responses**, rather than speed; a route that returns an error is temporarily set aside.

## Terminal version (TUI)

For servers over SSH or when the window does not open. To use your local English build, pass `--tui` to its executable. The following download commands fetch the latest upstream release, which may still have the original interface language.

```bash
curl -fsSL https://raw.githubusercontent.com/confeden/Antigravity/main/tui.sh | sh
```

In PowerShell, run as administrator:

```powershell
irm https://raw.githubusercontent.com/confeden/Antigravity/main/tui.ps1 | iex
```

Paste the key and press Enter. Arrow keys: select; Space: turn on or off; `q`: quit.

## Linux

Extract the archive and run `launch.sh` (double-click it or run `bash launch.sh`). You do not need `sudo`: if Antigravity is in a root-owned folder (`/opt`), the unlocker requests a password through pkexec only to write the patch.

## Troubleshooting

Check the card at the top of the window: it explains what is blocking the setup (antivirus, firewall, or another program) and what to do. If that does not help, click "Save report" and send the report to the group.

### Cargo is not recognized in PowerShell

If Rust is installed but `cargo` is not recognized, close and reopen the terminal. For an integrated terminal, restart its editor or host application too. To make the default installation available immediately in the current PowerShell session:

```powershell
$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
cargo --version
```

If that still fails, check that Rust installation completed and that `%USERPROFILE%\.cargo\bin\cargo.exe` exists.

### Build warnings

- `release canary`: an informational build/provenance identifier printed by the build script.
- `private routes: relay=false exits=false group=false`: the build does not include the corresponding private route assets/configuration. It can still build, but does not include all of the maintainer's preconfigured connection routes.
- `unused import` and `function ... is never used`: code is unused in the current build. These warnings did not prevent the verified Windows executable from building; a successful build alone does not confirm working sign-in or chat.

## System changes

All changes are targeted and can be undone using the switches in the window.

1. **Language Server / CLI** (`language_server.exe`, `agy.exe`): two equal-length string replacements: `ineligible` → `inexigible` (removes the regional restriction) and `https_proxy` → `AG_LS_PROXY`. The file size stays the same.
   On Windows, this also includes the shared CLI at `%USERPROFILE%\.gemini\bin\agy.exe` launched by the IDE extension; patching only the IDE's bundled Language Server leaves that CLI unchanged.
2. **Local proxy and service** (`%ProgramData%\AGUnlocker`, address `127.0.0.1:53129`): only `AG_LS_PROXY` points to it. The system-wide `HTTPS_PROXY` stays unchanged; `git` and browsers continue to work as before. TLS is not decrypted, and no certificates are installed.
3. **DNS rules (NRPT)** apply only to `cloudcode-pa.googleapis.com` and `daily-cloudcode-pa.googleapis.com`. Other DNS queries, including account sign-in, follow their usual path.
4. **IPv4 priority** over IPv6, only when you have no global IPv6 connectivity; removed when you undo the changes.

**Gemini CLI:** this unlocker does not provide patches for Gemini CLI. This support limitation does not mean that Gemini CLI is disabled or unavailable. The `agy.exe` executable mentioned above is Antigravity CLI, even when it is located under `.gemini\bin`.

The public source does not include the built-in exit list or the server side of the "Group proxy" route (to avoid overloading third-party servers). Without these, the program still builds and works using the other routes.

## VirusTotal

Antivirus software may flag unsigned programs with complex contents. The source code is public; if you do not trust it, run it in a virtual machine.

---

# Made with ❤️

🙋 **Telegram group:** [@nova_txt](https://t.me/nova_txt) — questions, news, and
support.

☕ **[Donate](https://nova-app.eu/donate)** — if Antigravity Unlocker has been useful to you.
An optional way to say "thank you".
