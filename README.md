# Audio Visualizer

cool audio visualizer for Linux, built with **Rust** and **Quickshell/QML**.

The project uses Rust for audio processing and Quickshell/QML to render the visualization on the desktop as a floating window. I will integrate it to my desktop shell later : https://github.com/Kingofpottatoes/potato-shell. Audio data is captured through `parec`, using the PulseAudio-compatible audio interface provided by PulseAudio or PipeWire.

## Features
- 32 band from 50 Hz to 12 kHz, log spaced, because that's how we hear the sound 
- Uses an FFT with a Hann window to analyse the incoming audio.
- animate a smooth drop of the bar when they drop, but instant upgrade if its an increase
- Calculates a separate attack value for each band. Used to make an effect when the sound increase quickly (on a beat for exemple)
- Draws the result in a Quickshell window, with the visual part handled by QML, because i want to integrate it in my shell, and its built in qml.

## Requirements

This project is made on, and for **Linux x86_64**. You should have:

- [Quickshell](https://quickshell.org/). or you dont have any window
- `parec` and `pactl`, which are provided by PulseAudio-compatible audio utilities, or the bar will never show
- A running PulseAudio server, or PipeWire with its PulseAudio compatibility layer enabled, to get the sound
- A default output device with a monitor source

The installer checks if you have all the requirements before copying the files.

## Installation

Download the release, put the binary next to install.sh and run the script with :

```bash
chmod +x install.sh
./install.sh
```

The script moves the binary to `~/.local/bin/` and the quickshell files to `${XDG_CONFIG_HOME:-$HOME/.config}/quickshell/audio-visualizer/`. and link the binary path to the qml Process component that needs it

## Uninstallation

To uninstall, run from your project root:

```bash
./install.sh --uninstall
```

## Usage

Launch it with:

```bash
qs -c audio-visualizer
```

Play some music, if nothing moves, check that pactl get-default-sink gives you something.

## what i used to build it

- [Rust](https://www.rust-lang.org/) for audio processing
- [Quickshell](https://quickshell.org/) for desktop integration and QML runtime
- [QML](https://doc.qt.io/qt-6/qmlapplications.html) for visualization interface
- [PulseAudio](https://www.freedesktop.org/wiki/Software/PulseAudio/) / [PipeWire](https://pipewire.org/) for audio infrastructure


## AI Notice:
AI has been used to know how to process a sound signal using fft and hans, as well as how to send data to the qml, and learn the syntax of these two language, but all the code and logic are human.

I gave up to have a professional looking README.

The installation script, is ai

i will probably edit this later to add it to my desktop : https://github.com/Kingofpottatoes/potato-shell but im currently to lazy to do it/working on other things
