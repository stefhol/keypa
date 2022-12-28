import { useQuery } from "@tanstack/react-query";
import React, { Dispatch, SetStateAction, useState } from "react";
import { TreeView } from "../../Components/tree-view/TreeView";
import { Building } from "../../util/intefaces/Buildings";
import { Department } from "../../util/intefaces/Departments";
import { Rest } from "../../util/Rest";
import { prepareData } from "../user/request/Request";
interface ILocalObjectType<T> {
    [key: number]: T
}
function convertToArray<T>(obj: ILocalObjectType<T>) {
    let arr = []
    for (const key in obj) {
        if (Object.prototype.hasOwnProperty.call(obj, key)) {
            const element = obj[key];
            arr.push(element)
        }
    }
    return arr
}
interface ILocalContext {
    departments: {
        value: ILocalObjectType<string>,
    };
    individualRooms: {
        value: ILocalObjectType<IndivdualRoom>,

    }
}

const send = async (data: CreateRequest) => {
    await Rest.quickAdd("request", "PUT", data);
}

export const LocalContext = React.createContext<ILocalContext>({} as ILocalContext)
export const CreatePropostion: React.FC<{}> = (props) => {

    const [activeUntil, setActiveUntil] = React.useState(null as Date | null);
    const [createKeycard, setCreateKeycard] = React.useState(false);
    const departments = React.useRef({} as ILocalObjectType<string>);
    const [description, setDescription] = React.useState("");
    const [doors, setDoors] = React.useState(null);
    const indivdualRooms = React.useRef({} as ILocalObjectType<IndivdualRoom>);

    return (<>
        <LocalContext.Provider value={{ departments: { value: departments.current }, individualRooms: { value: indivdualRooms.current } }}>

            <form>
                <h1>Neuer Antrag</h1>
                <label> Beschreibung:
                    <textarea
                        value={description}
                        onChange={e => setDescription(e.target.value)}
                    />
                </label>
                <label> Bis wann:
                    <input type={"date"} onChange={e => setActiveUntil(e.target.valueAsDate)}
                        value={activeUntil?.toDateString()} />
                </label>
                <DepartmentGroupWrapper />
                <AdditionalRoomsWrapper />

                <div>
                    <div className="container">
                        <label>
                            Gebäude auswählen
                            <select value={1}><option value={1}>FIM</option></select>
                        </label>
                        <label>
                            Räume auswählen
                            <TreeView filter={false} selectionRef={{ current: {} } as any} data={prepareData(demoData)} />
                        </label>
                        <button>Räume in einen anderen Gebäude hinzufügen</button>
                    </div>
                </div>

                <button onClick={e => {
                    e.preventDefault()
                    send({
                        active_until: activeUntil?.toISOString(),
                        create_keycard: createKeycard,
                        departments: convertToArray(departments.current),
                        description: description ?? undefined,
                        doors: doors ?? undefined,
                        indivdual_rooms: convertToArray(indivdualRooms.current),
                    } as CreateRequest).then(res => {
                        alert("Success")
                    })
                }}>Absenden</button>


            </form>

        </LocalContext.Provider>
    </>)
}


export interface CreateRequest {
    active_until: string;
    create_keycard: boolean;
    departments?: string[];
    description?: string;
    doors?: string[];
    indivdual_rooms?: IndivdualRoom[];
}

export interface IndivdualRoom {
    building_id: string;
    rooms: string;
}
interface SingelItemProp {
    nmbr: number
}
interface DepartmentGroupProp extends SingelItemProp {
    department?: Department[]
}
const DepartmentGroup: React.FC<DepartmentGroupProp> = (props) => {
    const { departments } = React.useContext(LocalContext);
    const [selected, setSelected] = React.useState("");
    React.useEffect(() => {
        departments.value[props.nmbr] = selected;
    }, [selected]);
    const selected_option = React.useMemo(() => props.department?.find(val => val.department_id === selected), [selected, props.department?.length])

    return (<>

        <div className="container" key={props.nmbr}>
            <label > {props.nmbr + 1}.

                <select value={selected} onChange={e => setSelected(e.target.value)} name={`department-select-${props.nmbr}`}>
                    <option value={""}></option>
                    {props.department?.map((val, idx) => <option key={idx} value={val.department_id}>{val.name}</option>)}
                            </select>
                        </label>
            {selected_option &&
                <p>
                    Beinahltet: {selected_option.buildings.map((val, idx) => <div key={idx}>
                        {`${val.name}: ${val.rooms.map((val) => val.name).join(", ")} `}
                    </div>)}
                </p>
            }

                    </div>

    </>)
}
export const DepartmentGroupWrapper: React.FC<{}> = (props) => {
    const [count, setCount] = React.useState([0] as number[]);
    const { data: availableDepartments } = useQuery(["department"], Rest.getDepartments)

    return (
        <div>
            <div className="container">
                <h2>Gruppen</h2>
                <>
                    {
                        count.map((_, idx) =>
                            <DepartmentGroup key={idx} nmbr={idx}
                                department={availableDepartments} />
                        )
                    }</>
            <button onClick={e => {
                e.preventDefault()
                setCount(prev => {
                    let arr = [...prev]
                    arr.push(0);
                    return arr
                })
            }}>Anderen Gruppe hinzufügen</button>
            </div>
        </div>
    )
}
interface AdditionalRoomsProps extends SingelItemProp {
    buildings?: Building[]
}
export const AdditionalRooms: React.FC<AdditionalRoomsProps> = (props) => {
    const { individualRooms } = React.useContext(LocalContext);
    const [selectedBuilding, setSelectedBuilding] = React.useState("");
    const [rooms, setSelectedRooms] = React.useState("");
    // const [selected, setSelected] = React.useState({} as IndivdualRoom);
    React.useEffect(() => {
        individualRooms.value[props.nmbr] = {
            building_id: selectedBuilding,
            rooms: rooms
        };
    }, [selectedBuilding, rooms]);
    return (<>
        <label>
            Gebäude auswählen
            <select value={selectedBuilding} onChange={e => setSelectedBuilding(e.target.value)} name={`department-select-${props.nmbr}`}>
                <option value={""}></option>
                {props.buildings?.map((val, idx) => <option key={idx} value={val.building_id}>{val.name}</option>)}
            </select>
        </label>
        <label>
            Räume auswählen
            <textarea value={rooms} onChange={e => setSelectedRooms(e.target.value)} />
        </label>
    </>)
}
export const AdditionalRoomsWrapper: React.FC<{}> = (props) => {
    const [count, setCount] = React.useState([0] as number[]);
    const { data } = useQuery(["buildings"], Rest.getBuildings)

    return (<div>
        <div className="container">
            <p>
                Sonstige Räume
            </p>
            <>
                <textarea />
            </>
        </div>

    </div>)
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
                        room_id: "1"
                    }
                ]
            }
        ]
    }
]