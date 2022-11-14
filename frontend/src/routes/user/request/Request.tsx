import { useQuery } from "@tanstack/react-query";
import React, { MutableRefObject, Ref } from "react";
import { SelectionRef, TreeData, TreeView } from "../../../Components/tree-view/TreeView";
import { Building, Room } from "../../../util/intefaces/Buildings";
import { Rest } from "../../../util/Rest";
const getData = async () => {
    return await Rest.getBuildings()
}
export interface RequestProps { }
export const Request: React.FC<RequestProps> = (props) => {
    const [description, setDescription] = React.useState("");
    const [until, setUntil] = React.useState("");
    const selection = React.useRef({ getCurrentSelection: () => Selection }) as unknown as SelectionRef;
    const { data } = useQuery(["buildings"], getData)
    const [keygroup, setKeygroup] = React.useState("");
    // accept: boolean;
    // changed_at: string;
    // comments ?: Comment[];
    // created_at: string;
    // description: string;
    // key_group_id: string;
    // pending: boolean;
    // reject: boolean;
    // request_id: string;
    // requester_id: string;
    return (<>

        <h1>Neue Anfrage</h1>
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
                    Waehle die Raeume fuer den du Zugang benoetigst
                <br />
                    {data &&
                        <TreeView selectionRef={selection} data={prepareData(data)} />
                    }
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
        name: `Gebaeude ${val.name}`,
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