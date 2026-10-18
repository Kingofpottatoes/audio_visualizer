import Quickshell
import QtQuick

FloatingWindow{
    implicitWidth:400
    implicitHeight:200

    Visualizer{
        path: "/home/king/Code/Rust/src/audio_visualizer/target/release/audio_visualizer"
        anchors.fill:parent
        barColor:"orange" //plutot ici la couleur caelestia
    }
}