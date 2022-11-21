import { useQuery } from "@tanstack/react-query";
import React, { MutableRefObject, Ref } from "react";
import { useNavigate } from "react-router-dom";
import { SelectionRef, TreeData, TreeView } from "../../../Components/tree-view/TreeView";
import { Building, Room } from "../../../util/intefaces/Buildings";
import { Rest } from "../../../util/Rest";
const getData = async () => {
    return await Rest.getBuildings()
}
export interface RequestProps { }
export const RoomRequest: React.FC<RequestProps> = (props) => {
    const [description, setDescription] = React.useState("");
    const [until, setUntil] = React.useState("");
    const selection = React.useRef({ getCurrentSelection: () => Selection }) as unknown as SelectionRef;
    const { data } = useQuery(["buildings"], getData)
    const [keygroup, setKeygroup] = React.useState("");

    return (<>

        <h1>Neue Raumanfrage</h1>
        <form>

            <label>
                Warum benoetigst du den Zugang
                <br />

                <textarea
                    style={{ minWidth: "15rem", minHeight: 100 }}
                    value={description}
                    onChange={e => setDescription(e.target.value)}
                />
            </label><br />
            <label>
                Bis wann
                <br />

                <input
                    type={'date'}
                    value={until}
                    onChange={(e) => {
                        setUntil(e.target.value)
                    }}
                />
            </label>
            <br />
            <label>
                Wähle die Räume fuer den du Zugang benoetigst
                <br />
                <div className="container">

                    {data &&
                        <TreeView selectionRef={selection} data={prepareData(data)} />
                    }
                </div>
            </label>
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

export const TempRequest: React.FC<RequestProps> = (props) => {
    const [description, setDescription] = React.useState("");
    const [until, setUntil] = React.useState("");
    const selection = React.useRef({ getCurrentSelection: () => Selection }) as unknown as SelectionRef;
    const { data } = useQuery(["buildings"], getData)
    const [keygroup, setKeygroup] = React.useState("");

    return (<>

        <h1>Neue Keycard mit Pfand Anfrage</h1>
        <h2>Hinweis: Pfand muss vorher bezahtl werden.</h2>
        <form>

            <label>
                Der Grund für neuen Zugang
                <br />

                <textarea
                    style={{ minWidth: "15rem", minHeight: 100 }}
                    value={description}
                    onChange={e => setDescription(e.target.value)}
                />
            </label><br />
            <label>
                Bis wann
                <br />

                <input
                    type={'date'}
                    value={until}
                    onChange={(e) => {
                        setUntil(e.target.value)
                    }}
                />
            </label>
            <br />
            <label>
                Wähle die Räume fuer den du Zugang benötigst
                <br />

                <div className="container">
                    {data &&
                        <TreeView selectionRef={selection} data={prepareData(data)} />
                    }
                </div>
            </label>
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

export const prepareData = (data: Building[], filter?: boolean) => {
    return data.map(val => ({
        name: `Gebäude ${val.name}`,
        children: prepareStockwerke(val.rooms)
    }))
}
const prepareStockwerke = (data: Room[]): TreeData[] => {
    let ret = [] as TreeData[]
    let floors = new Set(data.map(val => val.floor) as number[])
    floors.forEach(floor => {
        ret.push({
            name: `Stockwerk: ${floor}`,
            children: data.filter(val => val.floor == floor).map((val, idx) => ({
                name: `Raum: ${val.name}`,
                value: !!val.doors.find(val => val.owner === true),
                children: []
            }))
        })
    })
    return ret
}
export interface RequestPickerProps { }
export const RequestPicker: React.FC<RequestPickerProps> = (props) => {
    const navigate = useNavigate()
    return (<>
        <h2>
            Typ des Antrags
        </h2>
        <button onClick={e => {
            e.preventDefault()
            navigate("/request/add-request/room")
        }}>Zugangsantrag</button>
        <button onClick={e => {
            e.preventDefault()
            navigate("/request/add-request/keycard")
        }} >Keycardantrag</button>
        <button onClick={e => {
            e.preventDefault()
            navigate("/request/add-request/temp")
        }} >Temporärer Zugang</button>
    </>)
}