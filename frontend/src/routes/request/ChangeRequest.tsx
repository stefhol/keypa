import { useQuery } from "@tanstack/react-query"
import { format } from "date-fns"
import React from "react"
import { useParams } from "react-router-dom"
import CommentView from "../../Components/comment/Comment"
import { prepareData, TreeView } from "../../Components/tree-view/TreeView"
import { useLoading } from "../../hooks/useLoading"
import { Request } from "../../util/intefaces/Request"
import { Rest } from "../../util/Rest"
export interface ChangeRequestProps { }
const getUser = async ({ queryKey }: { queryKey: string[] }) => {
    const userId = queryKey[1]
    return await Rest.getSingleUser(userId)
}
const getRequest = async ({ queryKey }: { queryKey: string[] }) => {
    const requestId = queryKey[1]
    return await Rest.getSingleRequest(requestId)
}

export const ChangeRequest: React.FC<ChangeRequestProps> = (props) => {
    const { requestId } = useParams()

    const { data: request, isLoading } = useQuery(["request", requestId || ""], getRequest)
    useLoading(isLoading)

    return (<>
        {request &&
            <ChangeRequestForm data={request} />
        }
    </>)
}

export interface ChangeRequestFormProps { data: Request, }
export const ChangeRequestForm: React.FC<ChangeRequestFormProps> = (props) => {
    const { data: building } = useQuery(["building", props.data.request_id], ({ queryKey }) => {
        const requestId = queryKey[1]
        return Rest.getDoorsWithRequestId(requestId)
    })
    const { data: departmentsData } = useQuery(["departments", props.data.request_id], Rest.getDepartments)
    const [departments, setDepartments] = React.useState(props.data.departments);

    // date input is here managed is a bit complicated because of using the native date picker with preset value
    const [activeUntil, setActiveUntil] = React.useState(props.data?.active_until ? new Date(props.data?.active_until) : null);
    const dateElRef = React.useRef(undefined as unknown as HTMLInputElement);
    React.useEffect(() => {
        if (dateElRef?.current) dateElRef.current.defaultValue = activeUntil ? format(activeUntil, "yyyy-MM-dd") : ""
    }, [dateElRef.current]);
    //


    const [accept, setAccept] = React.useState(props.data.accept);
    const [reject, setReject] = React.useState(props.data.reject);
    const [pending, setPending] = React.useState(props.data.pending);
    const [rooms, setRooms] = React.useState(undefined as unknown as string[]);
    const statusValue = React.useMemo(() => {
        if (accept) {
            return "1"
        }
        if (reject) {
            return "2"
        }
        if (pending) {
            return "3"
        }
    }, [accept, reject, pending])
    const [addDepartmentOption, setAddDepartmentOption] = React.useState("");
    const treeData = React.useMemo(() => building?.length ? prepareData(building) : [], [building?.length])
    return (<>
        <h1>Antrag</h1>
        <form onSubmit={e => {
            e.preventDefault()
            send(props.data.request_id, {
                accept: accept || undefined,
                reject: reject || undefined,
                pending: pending || undefined,
                active_until: activeUntil?.toISOString() ?? null,
                departments: departments,
                rooms: rooms
            })
        }}>
            <div className="container">
                <h2>Kontaktinformationen</h2>
                <p>
                    Name: {props.data.requester.name}
                </p>
                <p>
                    Email: {props.data.requester.email}
                </p>
                <p>
                    Rolle: {props.data.requester.role_id}
                </p>
                <p>
                    Tel: {props.data.requester.tel}
                </p>
            </div>

            <div className="container">
                <h2>Beschreibung</h2>
                <p>
                    {props.data.description}
                </p>
            </div>
            <div className="container"><label>
                Aktiv bis
                <input type={"date"} ref={dateElRef} onChange={e => setActiveUntil(e.target.valueAsDate)} />
            </label></div>
            {(props.data.additional_rooms && building) && <div className="container">
                <h2>Angefragte Räume</h2>
                <div className="container">
                    <h2>Angefragte Individuelle Räume</h2>
                    <p>
                        {props.data.additional_rooms}
                    </p>

                    <div>
                        <b>Nachtragen</b>
                    </div>
                    <TreeView displayFilter selectionRef={{ current: {} } as any} data={treeData}
                        onChange={e => {
                            setRooms(e.map(val =>
                                val.children?.map(floor => floor?.children?.filter(room => room.value).map(room => room?.id || "") || []) || []).flat().flat())
                        }}
                    />
                </div>
                <div className="container">
                    <h2>Angefragte Raumgruppen</h2>
                    {(departments && departmentsData) && departments.map((val, idx) => {
                        const currentDepartment = departmentsData.find(dep => dep.department_id === val)
                        return <div className="container" key={idx}>
                            <label>
                                <b>{currentDepartment?.name}</b>
                                <button onClick={e => {
                                    e.preventDefault()
                                    setDepartments(prev => {
                                        return prev?.filter(f => f !== val)
                                    })
                                }}>X</button>
                            </label>
                            {currentDepartment?.buildings.map(building => (
                                <div key={building.building_id}>
                                    <b>{building.name}:</b> {building.rooms.map(room => room.name).join(", ")}
                                </div>
                            ))}

                        </div>
                    })}
                    {
                        departmentsData &&
                        <>
                            <select value={addDepartmentOption} onChange={e => setAddDepartmentOption(e.target.value)}>
                                <option value={""}></option>
                                {departmentsData.map((val, idx) => <option value={val.department_id} key={idx}>
                                    {val.name} {val.is_sensitive ? "Sensitiv" : ""}
                                </option>)}
                            </select>
                            <button
                                disabled={!addDepartmentOption}
                                onClick={e => {
                                    e.preventDefault()

                                    setDepartments(prev => {
                                        let newState = []
                                        if (prev) {
                                            newState = [...prev, addDepartmentOption]
                                        } else {
                                            newState = [addDepartmentOption]
                                        }
                                        return newState;
                                    })
                                    setAddDepartmentOption("")
                                }}
                            >Raumgruppe hinzufügen</button>
                        </>
                    }
                </div>



            </div>}
            <div className="container">
                <label>
                    Status:
                    <select name="status"
                        value={statusValue}
                        onChange={(e) => {
                        let value = e.target.value
                        if (value === "1") {
                            setAccept(true)
                            setReject(false)
                            setPending(false)
                        }
                        if (value === "2") {
                            setAccept(false)
                            setReject(true)
                            setPending(false)
                        }
                        if (value === "3") {
                            setAccept(false)
                            setReject(false)
                            setPending(true)
                        }
                    }}>
                        <option value="1">Akzeptieren</option>
                        <option value="2" >Ablehnen</option>
                        <option value="3" >Ausstehend</option>
                    </select>
                </label>

                <button>
                    Änderung Speichern
                </button>
            </div>
        </form>

        <div className="container">
            <CommentView
                requesterId={props.data.requester_id}
                requestId={props.data.request_id}

            />
        </div>
    </>)
}
interface ChangeRequest {
    active_until?: string | null,
    departments?: string[]
    rooms?: string[]
    accept?: boolean,
    reject?: boolean,
    pending?: boolean,
}
const send = async (requestId: string, data: ChangeRequest) => {
    await Rest.quickAdd(`request/${requestId}`, "POST", data);
}