import Quickshell
import QtQuick

FloatingWindow{
    implicitWidth:400
    implicitHeight:200
    color:'#00000000'

    Visualizer{
        path: "/home/king/Code/Rust/src/audio_visualizer/target/release/audio_visualizer"
        anchors.fill:parent
        barColor1:"orange" //plutot ici la couleur caelestia
        barColor2:"yellow"
        accent:"red"
    }
}