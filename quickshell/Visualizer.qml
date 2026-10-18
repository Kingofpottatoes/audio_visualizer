import QtQuick
import Quickshell.Io

Item{
    id:root
    property string path:""
    property int n_barres:32
    property var barres_heights:[]
    property color barColor:"teal" //a changer pour caelestia
    property color accent :"white"
    property real force:2
    property int gap:6


    property int compteur: 0
    Process{
        command: [root.path]
        running:true
        stdout: SplitParser{
            onRead: data => {root.barres_heights = data.split(";").map(Number)
            }

        }
    }
    Row{
        //affichage des barres
        anchors.fill:parent
        spacing:root.gap

        Repeater{
            model:root.n_barres

            Item{
                property real attaque: (root.barres_heights[index +root.n_barres]||0)/100
                width: (root.width-root.gap * (root.n_barres-1))/root.n_barres
                height:root.height
                
                Rectangle {
                    anchors.bottom: parent.bottom
                    anchors.horizontalCenter: parent.horizontalCenter
                    width: parent.width * (1 + 2 * parent.attaque)
                    height: parent.height * (root.barres_heights[index] || 0) / 100
                    color: Qt.tint(root.barColor, Qt.rgba(root.accent.r, root.accent.g, root.accent.b, Math.min(1, parent.attaque * root.force)))

                    Behavior on height {
                        NumberAnimation {
                            duration: 60
                            easing.type: Easing.OutQuad
                        }
                    }
                    Behavior on width {
                        NumberAnimation {
                            duration: 60
                            easing.type: Easing.OutQuad
                        }
                    }
                }
            }
        }
    }
}