import React from "react";
import { SelectionRef, TreeView } from "../../Components/tree-view/TreeView";
import { Building } from "../../util/intefaces/Buildings";
import { prepareData } from "../user/request/Request";

export const ConvertProposition: React.FC<{}> = (props) => {
    const selection = React.useRef({ getCurrentSelection: () => Selection }) as unknown as SelectionRef;




    return (<>

        <h1>Neue Raumanfrage von Bernd Habeck</h1>
        <form>

            <div className="container">
                <h3>Daten aus der Anfrage</h3>
                Beschreibung:<br />
                <p>
                    Ich bin ein Mitarbeiter im Lehrstuhl IT-Sec bis Ende 2024, ausserdem ist mein Buero noch im alten Fim Gebaeude
                </p>
                Bis wann:
                <p>31.12.2024</p>
            </div>

            <label>
            </label>


            <br />
            Wähle die Räume die Bernd Habeck zusätzlich Angefragt hat.
            <div className="container">
                <p> 1. Gebäude Fim: Raum 204</p>
            </div>
            <div className="container">

                <TreeView selectionRef={selection} data={prepareData(demoData)} />
            </div>
            <br />
            <button onClick={(e) => {
                e.preventDefault()
                console.log(selection.current.getCurrentSelection());
            }}>
                Absenden
            </button>
        </form>
    </>)
}
const demoData: Building[] = [

    {
        building_id: "FIM",
        name: "Fim",
        rooms: [
            {
                building_id: "FIM",
                floor: 1,
                is_sensitive: false,
                name: "104",
                room_id: "1",
                doors: [
                    {
                        door_id: "1",
                        name: "",
                        owner: false,
                        room_id: "1"
                    }
                ]
            },
            {
                building_id: "FIM",
                floor: 2,
                is_sensitive: false,
                name: "204",
                room_id: "1",
                doors: [
                    {
                        door_id: "1",
                        name: "",
                        owner: false,
                        room_id: "1"
                    }
                ]
            },
            {
                building_id: "FIM",
                floor: 1,
                is_sensitive: true,
                name: "105",
                room_id: "1",
                doors: [
                    {
                        door_id: "1",
                        name: "",
                        owner: false,
                        room_id: "1"
                    }
                ]
            }
        ]
    }
]