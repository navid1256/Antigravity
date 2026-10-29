# Antigravity Unlocker — Linux (beta port)

Unlock Antigravity 2.0 / IDE / CLI on Linux. The tool does two things:

1. **Patches client binaries and JS** — removes the eligibility check.
2. **Proxy route (bypasses error 400 «User location is not supported»)** —
   starts a local proxy and routes requests to Google servers through an exit in an
   allowed region, bypassing the server-side regional restriction.
   Your own VPN or proxy is **not required** — the built-in exit is used.

Everything runs **as a normal user, without root** (no sudo prompt).

## Getting started

1. Extract the archive anywhere (including a VM shared folder or a Windows drive:
   if programs cannot run from there, the unlocker copies itself to your
   home folder).
2. Double-click **`launch.sh`** → **«Run as a program»**, or use a
   terminal: `bash launch.sh`. **Do not use `sudo`** — the unlocker will refuse to
   start under sudo and explain why. It first asks for a key (free in the group
   t.me/nova_txt), then displays the switches.
3. Click **«Enable all»**.
4. **Close Antigravity completely and reopen it** — the language server
   will pick up the proxy and error 400 should disappear.

To launch from the application menu, run this once: `bash install.sh`.

Without a graphical session (a server over SSH), the unlocker opens directly in the
terminal. Alternatively, run this single command without downloading an archive:

    curl -fsSL https://raw.githubusercontent.com/confeden/Antigravity/main/tui.sh | sh

## Turning it off

Turn off the same switches — "Unlock account sign-in" restores the files
to their original state; "Bypass error 400" removes the local proxy and `AG_LS_PROXY`. To do it manually:
delete `~/.config/environment.d/ag-unlocker.conf` and run
`systemctl --user disable --now ag-unlocker-proxy`.

## If the installation was not found automatically

Use the pencil **✏** on the installation row to set its path manually. Common
locations: `/opt/antigravity`, `~/.local/share/antigravity`, `~/.local/bin` (CLI).
Snap versions cannot be patched (read-only); use a non-snap installation.

## What is not available on Linux yet

The DNS layer (NRPT rules and the local relay on `:53`) is Windows-specific. On Linux
the client patch and proxy route through the built-in exit work; the
DNS layer switches in the window do not install anything.

## Files

| File | Description |
|---|---|
| `ag_unlocker` | patcher, proxy, GUI, and terminal mode (ELF x86-64, glibc 2.17+) |
| `launch.sh` | launcher: checks execution permissions; uses terminal mode without a graphical session |
| `Antigravity-Unlocker.desktop` | shortcut |
| `install.sh` | adds a shortcut to the application menu |
