import QtQuick
import Quickshell.Io

Item{
    id:root
    property string path:""
    property int n_barres:32
    property var barres_heights:[]
    property color barColor:"teal" //a changer pour caelestia
    property int gap:6

    Process{
        command: [root.path]
        running:true
        stdout: SplitParser{
            onRead: data => root.barres_heights = data.split(";").map(Number)
        }
    }
    Row{
        //affichage des barres
        anchors.fill:parent
        spacing:root.gap

        Repeater{
            model:root.n_barres

            Item{
                width: (root.width-root.gap * (root.n_barres-1))/root.n_barres
                height:root.height

                Rectangle {
                    anchors.bottom: parent.bottom
                    width: parent.width
                    height: parent.height * (root.barres_heights[index] || 0) / 100
                    color: root.barColor
                }
            }
        }
    }
}