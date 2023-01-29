import { useQuery, useQueryClient } from "@tanstack/react-query"
import { format } from "date-fns"
import i18next from "i18next"
import React, { useMemo } from "react"
import { useLocation, useNavigate, useParams, useSearchParams } from "react-router-dom"
import CommentView from "../../Components/comment/Comment"
import { createTreeDatafromBuiding, TreeView } from "../../Components/tree-view/TreeView"
import UserContext from "../../context/UserContext"
import { useLoading } from "../../hooks/useLoading"
import { Building, BuildingWithOwner } from "../../util/intefaces/Buildings"
import { Request } from "../../util/intefaces/Request"
import { Rest } from "../../util/Rest"
import { IndividualRoomWrapper } from "../user/User"
export interface ChangeRequestProps { }
const getRequest = async ({ queryKey }: { queryKey: string[] }) => {
    const requestId = queryKey[1]
    const status = queryKey[2]
    return await Rest.getSingleRequest(requestId, status)
}

export const ChangeRequest: React.FC<ChangeRequestProps> = (props) => {
    const { requestId } = useParams()
    const [searchParams] = useSearchParams();



    const { data: request, isLoading, dataUpdatedAt, remove } = useQuery(["request", requestId || "", searchParams.toString() || ""], getRequest, { refetchOnMount: 'always' })
    useLoading(isLoading)
    const { data: building } = useQuery(["building"], () => {
        return Rest.getBuildings()
    })
    const key = useMemo(() => dataUpdatedAt, [isLoading])

    React.useEffect(() => {
        return () => {
            remove()
        }
    }, []);
    return (<>
        {(request && building && !isLoading) &&
            <ChangeRequestForm data={request} building={building} key={key} />
        }
    </>)
}

export interface ChangeRequestFormProps { data: Request, building: Building[] }
export const ChangeRequestForm: React.FC<ChangeRequestFormProps> = (props) => {

    const { data: departmentsData } = useQuery(["departments", props.data.request_id], Rest.getDepartments)
    const [departments, setDepartments] = React.useState(props.data.departments);

    // date input is here managed is a bit complicated because of using the native date picker with preset value
    const [activeUntil, setActiveUntil] = React.useState(props.data?.active_until ? new Date(props.data?.active_until) : null);
    const dateElRef = React.useRef(undefined as unknown as HTMLInputElement);
    React.useEffect(() => {
        if (dateElRef?.current) dateElRef.current.defaultValue = activeUntil ? format(activeUntil, "yyyy-MM-dd") : ""
    }, [dateElRef.current]);
    //
    const [searchParams] = useSearchParams();
    const status = searchParams.get('status');
    const disabled = React.useMemo(() => status === 'reject', [status])
    const { is_worker, is_leader } = React.useContext(UserContext);
    const navigate = useNavigate()
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
    const treeData = React.useRef(createTreeDatafromBuiding(props.building, props.data.doors));


    return (<>
        <h1>{i18next.t("change_request")}</h1>
        <form
            onSubmit={e => {
                e.preventDefault()
                send(props.data.request_id, {
                    accept: accept || undefined,
                    reject: reject || undefined,
                    pending: pending || undefined,
                    active_until: activeUntil?.toISOString() ?? null,
                    departments: departments,
                    rooms: rooms
                }).then(res => {
                    if (res === "FurtherActionsRequired" && is_worker) {
                        alert("Antrag wurde gespeichert muss aber von Verwaltungsvorgestzten genehmigt werden, da dieser Antrag nun sensitiv ist")
                    }
                    navigate("../")

                })
            }}>
            <div className="my-container">
                <h2>{i18next.t("contact_info")}</h2>
                <p>
                    {i18next.t("username")}: {props.data.requester.name}
                </p>
                <p>
                    {i18next.t("email")}: {props.data.requester.email}
                </p>
                <p>
                    {i18next.t("role")}: {props.data.requester.role_id}
                </p>
                <p>
                    {i18next.t("tel")}: {props.data.requester.tel}
                </p>
            </div>

            <div className="my-container">
                <h2>{i18next.t("description")}</h2>
                <p>
                    {props.data.description}
                </p>
            </div>
            <div className="grid">
                <label>
                    {i18next.t("active_until")}:
                    <input
                        type={"date"} ref={dateElRef} onChange={e => setActiveUntil(e.target.valueAsDate)} disabled={!(is_leader || is_worker || !disabled)} />
                </label>
                {(is_leader || is_worker) &&
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
                            <option value="1">{i18next.t("status_accepted")}
                            </option>
                            <option value="2" >{i18next.t("status_reject")}</option>
                            <option value="3" >{i18next.t("status_pending")}</option>
                        </select>
                    </label>
                }
            </div>
            {!disabled && <> {(props.data.request_type !== "keycard") && <div className="">
                <details className="" open>
                    <summary>
                        <h2>{i18next.t("requested_individual_rooms")}</h2>
                        {props.data.additional_rooms}
                    </summary>

                    {(is_worker || is_leader) ? <>
                        <h2>
                            {i18next.t("individual_rooms")}
                        </h2>

                        <TreeView displayFilter selectionRef={{ current: {} } as any} data={treeData.current}
                            onChange={e => {
                                setRooms(e.map(val =>
                                    val.children?.map(floor => floor?.children?.filter(room => room.value).map(room => room?.id || "") || []) || []).flat().flat())
                            }}
                        />
                    </> : <>
                        <h2>
                                {i18next.t("individual_rooms")}

                        </h2>
                            <IndividualRoomWrapper buildings={fillBuildingWithOwner(props.building, props.data.doors)} />
                    </>}
                </details>
                <details open className="my-container">
                    <summary>
                        <h2>{i18next.t("requested_department")}</h2>
                    </summary>
                    <div className="wrap">
                        {(departments && departmentsData) && departments.map((val, idx) => {
                            const currentDepartment = departmentsData.find(dep => dep.department_id === val)
                            return <div className="my-container" key={idx}>
                                <label>
                                    <b>{currentDepartment?.name}</b>
                                    {(is_leader || is_worker) && <button className="outline contrast red"
                                        style={{ "width": "3rem" }}
                                        onClick={e => {
                                            e.preventDefault()
                                            setDepartments(prev => {
                                                return prev?.filter(f => f !== val)
                                            })
                                        }}>X</button>}
                                </label>
                                {currentDepartment?.buildings.map(building => (
                                    <div key={building.building_id}>
                                        <b>{building.name}:</b> {building.rooms.map(room => room.name).join(", ")}
                                    </div>
                                ))}

                            </div>
                        })}
                    </div>
                    {
                        (departmentsData && (is_leader || is_worker)) &&
                        <>
                            <select value={addDepartmentOption}

                                onChange={e => setAddDepartmentOption(e.target.value)}>
                                <option value={""}></option>
                                {departmentsData.map((val, idx) => <option value={val.department_id} key={idx}>
                                    {val.name} {val.is_sensitive ? "Sensitiv" : ""}
                                </option>)}
                            </select>
                            <button
                                className="outline contrast"
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
                            >{i18next.t("add_requested_department")}
                            </button>
                        </>
                    }
                </details>



            </div>}
                {(is_leader || is_worker) &&
                    <button className="outline contrast" >
                        {i18next.t("send")}
                    </button>}
            </>}
        </form>

        {!disabled &&
            <CommentView
                requesterId={props.data.requester_id}
                requestId={props.data.request_id}

            />
        }
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
    if (data.departments) {
        data.departments = new Array(...new Set(data.departments))
    }
    if (data.rooms) {
        data.rooms = new Array(...new Set(data.rooms))
    }
    return await (await Rest.quickAdd(`request/${requestId}`, "POST", data)).json();
}

const fillBuildingWithOwner = (buildings: Building[], doors: string[]): BuildingWithOwner[] => {
    let tempBuilding = buildings as BuildingWithOwner[]
    tempBuilding.map(building => {
        building.rooms = building.rooms.map(room => {
            room.doors = room.doors.map(door => {
                door.owner = !!doors.find(doorId => doorId == door.door_id)
                return door
            })
            return room
        })
        return building
    })
    return tempBuilding
}