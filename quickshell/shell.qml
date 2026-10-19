import Quickshell
import QtQuick

FloatingWindow{
    implicitWidth:400
    implicitHeight:200
    color:'#00000000'

    Visualizer{
        path: "__BIN__"
        anchors.fill:parent
        barColor1:"orange" //plutot ici la couleur caelestia
        barColor2:"yellow"
        accent:"red"
    }
}