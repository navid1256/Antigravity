#!/bin/sh
# Antigravity Unlocker — terminal mode in one command (Linux x86-64):
#
#   curl -fsSL https://raw.githubusercontent.com/confeden/Antigravity/main/tui.sh | sh
#
# Fetches the latest release from GitHub, saves the program to
# ~/.local/share/agunlocker/ and runs it in this terminal. Running
# the same command again downloads only if a newer version exists. No root.
set -eu

REPO="confeden/Antigravity"
DIR="${XDG_DATA_HOME:-$HOME/.local/share}/agunlocker"
BIN="$DIR/ag_unlocker"

case "$(uname -m)" in
    x86_64 | amd64) ;;
    *)
        echo "Antigravity Unlocker is built only for x86-64; this system is $(uname -m)." >&2
        exit 1
        ;;
esac

# The newest tag from where /releases/latest redirects to: no API call, so no
# rate limit on a server sharing its address with a hundred others.
TAG=""
if URL=$(curl -fsSLo /dev/null -w '%{url_effective}' "https://github.com/$REPO/releases/latest"); then
    TAG="${URL##*/}"
fi
VER="${TAG#v}"

case "$VER" in
    "" | *[!0-9._]*)
        if [ -x "$BIN" ]; then
            echo "GitHub did not respond — starting the previously downloaded version." >&2
        else
            echo "Could not find the latest version on github.com/$REPO." >&2
            exit 1
        fi
        ;;
    *)
        if [ ! -x "$BIN" ] || [ "$(cat "$BIN.version" 2>/dev/null)" != "$VER" ]; then
            echo "Downloading Antigravity Unlocker $VER…" >&2
            mkdir -p "$DIR"
            TMP=$(mktemp -d)
            trap 'rm -rf "$TMP"' EXIT
            # The tag spells a fourth number with a dot (v2.24.0.1), the asset with
            # an underscore (AG_2.24.0_1_linux.tar.gz): try both.
            ALT=$(echo "$VER" | sed -E 's/\.([0-9]+)$/_\1/')
            NAME=""
            for cand in "$VER" "$ALT"; do
                if curl -fsIL "https://github.com/$REPO/releases/download/$TAG/AG_${cand}_linux.tar.gz" >/dev/null 2>&1; then
                    NAME="$cand"
                    break
                fi
            done
            if [ -z "$NAME" ]; then
                echo "Release $TAG has no build for Linux." >&2
                exit 1
            fi
            curl -fsSL "https://github.com/$REPO/releases/download/$TAG/AG_${NAME}_linux.tar.gz" |
                tar xz -C "$TMP"
            mv -f "$TMP/AG_${NAME}_linux/ag_unlocker" "$BIN"
            chmod +x "$BIN"
            echo "$VER" >"$BIN.version"
            rm -rf "$TMP"
            trap - EXIT
        fi
        ;;
esac

# `curl … | sh` hands this script to sh on stdin; the program's keys come from
# the terminal itself.
exec "$BIN" --tui </dev/tty
