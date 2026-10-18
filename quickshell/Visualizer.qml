import QtQuick
import Quickshell.Io

Item{
    id:root
    property string path:""
    property int n_barres:32
    property var barres_heights:[]
    property color barColor1:"teal" //a changer pour caelestia integration
    property color barColor2:"teal" //a changer pour caelestia integration
    property color accent :"white" //a changer pour caelestia integration
    property real force:2.5
    property real gap:0 //this is a %
    property real gonflement:0.5


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
        Repeater{
            model:root.n_barres
            Item{
                id:cell
                width:parent.width/root.n_barres
                height:parent.height
                property real attaque:(root.barres_heights[index +root.n_barres]||0)/100
                property real niveau: Math.min(1,(1.1/*boost for style*/)*(root.barres_heights[index] || 0) / 100)
                property real intensity:Math.min(1,attaque*root.force)
                property color topColor:Qt.tint(root.barColor1, Qt.rgba(root.accent.r, root.accent.g, root.accent.b, intensity*Math.min(1,attaque * root.force)))
                property color bottomColor: Qt.tint(root.barColor2, Qt.rgba(root.accent.r, root.accent.g, root.accent.b, intensity*0.6*Math.min(1, attaque * root.force)))

                Item{
                    width:parent.width*(1-root.gap)
                    height:parent.height
                    anchors.horizontalCenter:parent.horizontalCenter

                    Rectangle{
                        anchors.bottom: parent.bottom
                        anchors.horizontalCenter: parent.horizontalCenter
                        width:parent.width * (1-(root.gonflement*(1-cell.attaque)))
                        height:parent.height*cell.niveau
                        gradient:Gradient{
                            GradientStop { position: 0.0; color: cell.topColor }
                            GradientStop { position: 1.0; color: cell.bottomColor }
                        }

                    }
                }
                Behavior on niveau { NumberAnimation { duration: 60 } }
                Behavior on attaque { NumberAnimation { duration: 60 } }

            }
        }
    }
}