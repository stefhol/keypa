import { useQuery } from "@tanstack/react-query";
import i18next from "i18next";
import React, { } from "react";
import { useNavigate } from "react-router-dom";
import { DepartmentGroup } from "../../Components/request/Department";
import { RoomSelection } from "../../Components/request/RoomSelection";
import UserContext from "../../context/UserContext";
import { Rest } from "../../util/Rest";
interface ILocalObjectType<T> {
    [key: number]: T
}
function convertToArray<T>(obj: ILocalObjectType<T>) {
    let arr = []
    for (const key in obj) {
        if (Object.prototype.hasOwnProperty.call(obj, key)) {
            const element = obj[key];
            if (element)
                arr.push(element)
        }
    }
    return arr
}
interface ILocalContext {
    departments: {
        value: ILocalObjectType<string>,
    };
    doors: {
        value: ILocalObjectType<string[]>,

    }
}

const send = async (data: CreateRequest) => {
    await Rest.quickAdd("request", "PUT", data);
}

export const CreateKeycardRequestForm: React.FC<{ title: JSX.Element }> = (props) => {
    const [activeUntil, setActiveUntil] = React.useState(null as Date | null);
    const [description, setDescription] = React.useState("");
    return (<>

        <form>
            {props.title}
            <label> {i18next.t("descritption")}:
                <textarea
                    value={description}
                    onChange={e => setDescription(e.target.value)}
                />
            </label>
            <label> {i18next.t("active_until")}:
                <input type={"date"} onChange={e => setActiveUntil(e.target.valueAsDate)}
                />
            </label>
            <button onClick={e => {
                e.preventDefault()
                send({

                    active_until: activeUntil?.toISOString(),
                    create_keycard: true,
                    departments: undefined,
                    description: description || undefined,
                    rooms: undefined,
                    other_rooms: undefined,
                } as CreateRequest).then(res => {
                    alert("Success")
                })
            }}>{i18next.t("send")}</button>
        </form>
    </>)
}
export const LocalContext = React.createContext<ILocalContext>({} as ILocalContext)
export const CreateRequestForm: React.FC<{
    title: JSX.Element,
    createKeycard: boolean
}> = (props) => {

    const [activeUntil, setActiveUntil] = React.useState(null as Date | null);
    const departments = React.useRef({} as ILocalObjectType<string>);
    const [description, setDescription] = React.useState("");
    const rooms = React.useRef({} as ILocalObjectType<string[]>);
    const [otherRooms, setOtherRooms] = React.useState("");
    const { is_worker, is_admin, is_leader } = React.useContext(UserContext);
    const navigate = useNavigate()
    return (<>
        <LocalContext.Provider value={{ departments: { value: departments.current }, doors: { value: rooms.current } }}>

            <form>
                {props.title}
                <label> {i18next.t("descritption")}:
                    <textarea
                        value={description}
                        onChange={e => setDescription(e.target.value)}
                    />
                </label>
                <label> {i18next.t("active_until")}:
                    <input type={"date"} onChange={e => setActiveUntil(e.target.valueAsDate)}
                    />
                </label>
                <DepartmentGroupWrapper />
                {!is_worker ?
                    <div className="container">
                        <p>
                            {i18next.t("other_rooms")}
                        </p>
                        <textarea
                            value={otherRooms}
                            onChange={(e) => setOtherRooms(e.target.value)}
                        />
                    </div> :
                    <RoomSelectionWrapper />
                }



                <button onClick={e => {
                    e.preventDefault()
                    send({

                        active_until: activeUntil?.toISOString(),
                        create_keycard: props.createKeycard,
                        departments: convertToArray(departments.current),
                        description: description || undefined,
                        rooms: convertToArray(rooms.current).flat() ?? undefined,
                        other_rooms: otherRooms || null,
                    } as CreateRequest).then(res => {
                        alert("Success")
                        navigate("/user")

                    })
                }}>{i18next.t("send")}:</button>


            </form>

        </LocalContext.Provider>
    </>)
}


export interface CreateRequest {
    active_until: string;
    create_keycard: boolean;
    departments?: string[];
    description?: string;
    rooms?: string[];
    other_rooms?: string;
}


const DepartmentGroupWrapper: React.FC<{}> = (props) => {
    const [count, setCount] = React.useState([] as number[]);
    const { data: availableDepartments } = useQuery(["department"], Rest.getDepartments)
    const { departments } = React.useContext(LocalContext);
    const elements = React.useMemo(() => count.map((_, idx) =>
        <DepartmentGroup key={idx} nmbr={idx}
            department={availableDepartments}
            onChange={(e) => { departments.value[idx] = e }}
        />
    ), [count.length])
    return (
        <div>
            <div className="container">
                <h2>{i18next.t("group")}:</h2>
                <>
                    {elements}
                </>
                <button onClick={e => {
                    e.preventDefault()
                    setCount(prev => {
                        let arr = [...prev]
                        arr.push(0);
                        return arr
                    })
                }}>{i18next.t("add_requested_department")}:</button>
            </div>
        </div>
    )
}
const RoomSelectionWrapper: React.FC<{}> = (props) => {
    const { data } = useQuery(["buildings"], Rest.getBuildings)
    const [count, setCount] = React.useState([] as number[]);
    const { doors } = React.useContext(LocalContext);
    return (<>
        <div className="container">
            <h2>{i18next.t("rooms")}</h2>

            {data &&
                count.map((_, idx) =>
                    <RoomSelection key={idx} buildings={data} nmbr={idx} onChange={(e) => {
                        doors.value[idx] = e
                    }} />
                )
            }

            <button
                onClick={e => {
                    e.preventDefault()
                    setCount(prev => {
                        let arr = [...prev]
                        arr.push(0);
                        return arr
                    })
                }}>
                {i18next.t("choose_rooms_different_building")}</button>
        </div>
    </>)
}