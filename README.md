# Audio Visualizer

A real-time audio visualizer for Linux, built with **Rust** and **Quickshell/QML**.

The project uses Rust for audio processing and Quickshell/QML to render the visualization on the desktop. Audio data is captured through `parec`, using the PulseAudio-compatible audio interface provided by PulseAudio or PipeWire.

## Features

- **Real-time visualization** of audio input.
- **Rust-based audio processing**.
- **Quickshell/QML interface** for desktop rendering.
- **PulseAudio / PipeWire compatibility** through `parec`.
- **Native Linux integration** using Quickshell.
- **Cool coloration :)**

## Requirements

The following tools are required to run the project:

- Linux on x86_64
- [Quickshell](https://quickshell.org/) (`qs`)
- `parec` and `pactl`, provided by PulseAudio-compatible audio utilities
- A working PulseAudio or PipeWire audio setup

The project also includes an installation script that can compile the Rust executable if the expected binary is not already available. If compilation is needed, Rust and Cargo must be installed.

## Installation

Make sure the prebuilt audio-visualizer executable and the quickshell/ directory are present in the project root.
Run the installation script:

```bash
./install.sh
```
The script checks the system architecture and required commands, then installs the executable to ~/.local/bin/ and the Quickshell configuration to:

${XDG_CONFIG_HOME:-$HOME/.config}/quickshell/audio-visualizer/

No Rust compilation is performed by the installation script. The prebuilt executable must be provided separately.

## Uninstallation

To remove the installed executable and Quickshell configuration, run the following command from the project directory:

./install.sh --uninstall

This removes the installed audio-visualizer executable and the configuration directory created for this application.

## Usage

After installation, launch the visualizer with:

```bash
qs -c audio-visualizer
```

The visualizer relies on your existing Linux audio setup. Make sure PulseAudio or PipeWire is running and that the required audio commands are available.



## Project Structure

```text
audio_visualizer/
├── src/          # Rust source code
├── quickshell/   # Quickshell/QML interface
├── install.sh    # Installation script
├── Cargo.toml    # Rust package manifest
└── Cargo.lock    # Locked dependency versions
```

## Technologies

- [Rust](https://www.rust-lang.org/) — audio processing
- [Quickshell](https://quickshell.org/) — desktop integration and QML runtime
- [QML](https://doc.qt.io/qt-6/qmlapplications.html) — visualization interface
- [PulseAudio](https://www.freedesktop.org/wiki/Software/PulseAudio/) / [PipeWire](https://pipewire.org/) — audio infrastructure

## License

This project is intended to be released under the **MIT License**.

A `LICENSE` file containing the complete MIT License text should be included in the repository before distributing the project under that license.

## Status

This is a personal project under development. Features, installation steps, and implementation details may evolve over time.
