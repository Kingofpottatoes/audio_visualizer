# Audio Visualizer

cool audio visualizer for Linux, built with **Rust** and **Quickshell/QML**.

The project uses Rust for audio processing and Quickshell/QML to render the visualization on the desktop as a floating window. I will integrate it to my desktop shell later : https://github.com/Kingofpottatoes/potato-shell. Audio data is captured through `parec`, using the PulseAudio-compatible audio interface provided by PulseAudio or PipeWire.

## Features

- Shows the audio playing through your current default output device.
- Splits the spectrum into **32 frequency bands**, spaced logarithmically from 50 Hz to 12 kHz.
- Uses an FFT with a Hann window to analyse the incoming audio.
- Smooths the bar levels so they do not jump around, while still reacting quickly when the sound gets louder.
- Calculates a separate attack value for each band. These values make the bars react to sudden changes and drive the accent effect in the QML visualizer.
- Draws the result in a Quickshell window, with the visual part handled by QML.

## Requirements

This release is currently built for **Linux x86_64**. You also need:

- [Quickshell](https://quickshell.org/) and its `qs` command.
- `parec` and `pactl`, which are provided by PulseAudio-compatible audio utilities.
- A running PulseAudio server, or PipeWire with its PulseAudio compatibility layer enabled.
- A default output device with a monitor source available, so the visualizer can read the audio being played.

The installer checks the architecture and looks for `qs`, `parec` and `pactl` before copying the files.

## Installation

1. Open the [releases page](https://github.com/Kingofpottatoes/audio_visualizer/releases) and download the source-code archive and the `audio-visualizer` binary from the latest release.
2. Extract the source archive.
3. Put the downloaded `audio-visualizer` binary in the extracted project folder. It must sit next to `install.sh` and the `quickshell/` directory.
4. Open a terminal in that folder and run:

```bash
chmod +x install.sh
./install.sh
```

The script installs the executable to `~/.local/bin/` and copies the Quickshell files to `${XDG_CONFIG_HOME:-$HOME/.config}/quickshell/audio-visualizer/`. It also writes the installed binary's path into the Quickshell configuration.

You do not need Rust or Cargo to install the release binary. The installer does not compile the project.

## Uninstallation

To remove the installed executable and Quickshell configuration, run the following command from the project directory:

```bash
./install.sh --uninstall
```

This removes the installed audio-visualizer executable and the configuration directory created for this app.

## Usage

Once the installation has finished, start it from a terminal with:

```bash
qs -c audio-visualizer
```

The visualizer follows the monitor source of your current default audio output. Play something through that output and the bars should start moving. If there is no activity, check that PulseAudio or PipeWire is running and that `pactl get-default-sink` returns a device with a working monitor source.

## what i used to build it

- [Rust](https://www.rust-lang.org/) — audio processing
- [Quickshell](https://quickshell.org/) — desktop integration and QML runtime
- [QML](https://doc.qt.io/qt-6/qmlapplications.html) — visualization interface
- [PulseAudio](https://www.freedesktop.org/wiki/Software/PulseAudio/) / [PipeWire](https://pipewire.org/) — audio infrastructure


## AI Notice:
AI have been used to know how to process a sound signal using fft and hans, as well as how to send data to the qml, and learn the syntax of these two language, but all the code and logic are human.

part of this readme is AI generated, because it looked more profesional.
The installation script, is also ai

i will probably edit this later to add it to my desktop : https://github.com/Kingofpottatoes/potato-shell but im currently to lazy to do it/working on other things
