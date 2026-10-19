#!/usr/bin/env bash
# install.sh: install audio-visualizer for the user
set -eu
# where am i
CURRENT="$(cd "$(dirname "$0")" && pwd)"
cd "$CURRENT"
BIN_DIR="$HOME/.local/bin"
CONF_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/quickshell/audio-visualizer"

if [ "${1:-}" = "--uninstall" ]; then
    rm -f "$BIN_DIR/audio-visualizer"
    rm -rf "$CONF_DIR"
    echo "audio-visualizer a été désinstallé."
    exit 0
fi

if [ "$(uname -s)" != "Linux" ] || [ "$(uname -m)" != "x86_64" ]; then
    echo "Error this is only available on Linux Currently">&2
    exit 1
fi

# --- Check if all dependencies are here
MANQUANTS=""
for OUTIL in parec pactl qs; do
    if ! command -v "$OUTIL" >/dev/null 2>&1; then
        MANQUANTS="$MANQUANTS $OUTIL"
    fi
done
if [ -n "$MANQUANTS" ]; then
    echo "You need:$MANQUANTS" >&2
    exit 1
fi


if [ ! -f "$CURRENT/audio-visualizer" ] || [ ! -d "$CURRENT/quickshell" ]; then
    echo "Error : You should run the script in the folder" >&2
    exit 1
fi

# --- Install ---
mkdir -p "$BIN_DIR" "$CONF_DIR"
install -m755 audio-visualizer "$BIN_DIR/audio-visualizer"
cp -r quickshell/. "$CONF_DIR/"

sed -i "s|__BIN__|$BIN_DIR/audio-visualizer|g" "$CONF_DIR/shell.qml"

if grep -q "__BIN__" "$CONF_DIR/shell.qml"; then
    echo "Erreur : le chemin du binaire n'a pas pu être écrit dans shell.qml." >&2
    exit 1
fi


echo "Install finished"
echo "  with your music : qs -c audio-visualizer"
echo "  uninstall : ./install.sh --uninstall"