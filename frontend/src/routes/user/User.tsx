import { useQuery } from "@tanstack/react-query"
import i18next from "i18next"
import React from "react"
import { useNavigate, useParams } from "react-router-dom"
import { createKeycardDefColumn } from "../../Components/table/ColumnDef/Keycard"
import { createRequestDefColumn } from "../../Components/table/ColumnDef/Request"
import { Table } from "../../Components/table/Table"
import UserContext from "../../context/UserContext"
import { Building, BuildingWithOwner } from "../../util/intefaces/Buildings"
import { Department } from "../../util/intefaces/Departments"
import { Keycard } from "../../util/intefaces/Keycard"
import { Request, User } from "../../util/intefaces/Request"
import { Rest } from "../../util/Rest"

export interface SelfUserProps { }
export const SelfUser: React.FC<SelfUserProps> = (props) => {
    const { data: buildings } = useQuery(["self", "doors"], Rest.getSelfDoors)
    const { data: user } = useQuery(["self", "users"], Rest.getSelf)
    const { data: departments } = useQuery(["self", "department"], Rest.getSelfDepartments)
    const { data: keycards } = useQuery(["self", "keycard",], Rest.getSelfKeycard);
    const { data: acceptedRequests } = useQuery(["self", "acceptedRequests"], () => Rest.
        getSelfRequests("request_status=accept"))
    const { data: pendingRequests } = useQuery(["self", "pendingRequests"], () => Rest.
        getSelfRequests("request_status=pending"))
    return (<>
        {(buildings !== undefined && user && departments !== undefined) &&
            <UserFc
                keycard={keycards || []}
                department={departments || []}
                buildings={buildings || []}
                user={user}
                acceptedRequests={acceptedRequests || []}
                pendingRequests={pendingRequests || []}
                isSelf />}
    </>)
}
export const UserByUserId: React.FC<SelfUserProps> = (props) => {
    const { userId } = useParams()
    const { data: buildings } = useQuery(["user", userId || "", "doors"], getDoorsByUser)
    const { data: user } = useQuery(["user", userId || ""], getUserById)
    const { data: departments } = useQuery(["user", userId, "department"], ({ queryKey }) => Rest.getUserDepartments(queryKey[1] || ""))
    const { data: keycards } = useQuery(["user", userId, "keycard",], ({ queryKey }) => Rest.getKeycardsFromUser(queryKey[1] || ""))
    const { data: acceptedRequests } = useQuery(["user", userId, "acceptedRequests"], ({ queryKey }) => Rest.
        getRequestsFromUser(queryKey[1] || "", "request_status=accept"))
    const { data: pendingRequests } = useQuery(["user", userId, "pendingRequests"], ({ queryKey }) => Rest.
        getRequestsFromUser(queryKey[1] || "", "request_status=pending"))
    return (<>
        {
            (buildings !== undefined && user && departments !== undefined) && <UserFc
                keycard={keycards || []}
                department={departments}
                buildings={buildings}
                user={user}
                acceptedRequests={acceptedRequests || []}
                pendingRequests={pendingRequests || []}
                isSelf={false}
            />
        }

    </>)
}


const getDoorsByUser = async ({ queryKey }: { queryKey: string[] }) => {
    const userId = queryKey[1]
    return await Rest.getDoorsByUser(userId)
}
const getUserById = async ({ queryKey }: { queryKey: string[] }) => {
    const userId = queryKey[1]
    return await Rest.getUserByUserId(userId)
}
export interface UserProps {
    buildings: BuildingWithOwner[]
    user: User
    isSelf: boolean
    department: Department[]
    acceptedRequests: Request[]
    pendingRequests: Request[]
    keycard: Keycard[]
}

const UserFc: React.FC<UserProps> = (props) => {
    const navigate = useNavigate()
    const { is_admin, is_leader, is_worker } = React.useContext(UserContext);
    return (<>
        <h1>
            {props.user.name}
        </h1>
        <article className="my-container">
            <h3>{i18next.t("contact_info")}</h3>
            <p>
                {props.user.email}
            </p>
            <p>
                {props.user.tel}
            </p>
        {(!is_leader) ? <>
                {props.isSelf && <button className="outline contrast" onClick={(e) => {
                e.preventDefault()
                navigate("/request/add-request")
            }}>{i18next.t("create_new_request")}</button>
            }
        </> : <>
                    <button className="outline contrast" onClick={(e) => {
                e.preventDefault()
                navigate("/request/add-request/keycard")
                }}>{i18next.t("create_new_keycard")}</button>
        </>}
        </article>


        <details open>
            <summary>
                <h3>{i18next.t("access")}</h3>
            </summary>
            <div className="wrap">
                <article>
                    <header>
                        {i18next.t("individual_rooms")}
                    </header>
                    <IndividualRoomWrapper buildings={props.buildings} />
                </article>
                <DepartmentWrapper departments={props.department} />
            </div>
        </details>
        <TempKeycardAuthorizationView keycards={props.keycard} userId={props.isSelf ? undefined : props.user.user_id} />

        <details open>
            <summary><h3>{i18next.t("keycards")}</h3></summary>
            <>

                <Table data={props.keycard}
                    rowAction={
                        [
                            {
                                element: <button className="outline contrast">{i18next.t("change")}</button>,
                                onClick(idx) {
                                    navigate(`/keycard/change-request/${props.keycard?.[idx].keycard_id}`)
                                },
                            }
                        ]
                    }

                    columns={createKeycardDefColumn()} />

            </>
        </details>
        <details open>
            <summary><h3>{i18next.t("approved_requests")}</h3></summary>
            <Table
                defaultHidden={["name"]}
                columns={createRequestDefColumn()}
                data={props.acceptedRequests}
                rowAction={[{ element: <button className="outline contrast">{i18next.t("open")}</button>, onClick: (rowIndex) => { navigate(`/request/change-request/${props.acceptedRequests?.[rowIndex].request_id}`) } }]}
            // rowAction={ }
            />
        </details>
        <details open>
            <summary>
                <h3>{i18next.t("pending_requests")}</h3>
            </summary>
            <Table
                defaultHidden={["name"]}
                columns={createRequestDefColumn()}
                data={props.pendingRequests}
                rowAction={[{ element: <button className="outline contrast">{i18next.t("open")}</button>, onClick: (rowIndex) => { navigate(`/request/change-request/${props.pendingRequests?.[rowIndex].request_id}`) } }]}
            // rowAction={ }
            />
        </details>
    </>)
}
export interface DepartmentWrapperProps { departments: Department[] }
export const DepartmentWrapper: React.FC<DepartmentWrapperProps> = (props) => {

    return (<>

        {props.departments.map((val, idx) =>
            <article key={idx}>
                <header>{val.name}</header>

                {i18next.t("includes")}: {val.buildings.map((val, idx) => <div key={idx}>
                    <b>{val.name}:</b>{` ${val.rooms.map((val) => val.name).join(", ")} `}
                </div>)}
            </article>
        )}

    </>)
}
export interface IndividualRoomWrapperProps { buildings: BuildingWithOwner[] }
export const IndividualRoomWrapper: React.FC<IndividualRoomWrapperProps> = (props) => {
    const hasSomething = React.useMemo(() => !!props.buildings.find(val => val.rooms.find(val => val.doors.find(val => val.owner))), [props.buildings?.length])
    return (<>
        {hasSomething && <>

            {props.buildings.map((val, idx) => <div key={idx}>
                <b>{val.name}:</b>{` ${val.rooms.filter(val => val.doors.find(val => val.owner)).map((val) => val.name).join(", ")} `}
            </div>)}
        </>}

    </>)
}
export interface TempKeycardAuthorizationViewProps {
    keycards: Keycard[]
    userId?: string
}
export const TempKeycardAuthorizationView: React.FC<TempKeycardAuthorizationViewProps> = (props) => {

    return (<>
        {props.keycards.filter(val => val.keycard_type === 'temp').map((val, idx) => <TempKeycardSingle key={val.keycard_id} keycard={val} userId={props.userId} idx={idx} />)}
    </>)
}
export interface TempKeycardSingleProps {
    keycard: Keycard,
    userId?: string
    idx: number
}
export const TempKeycardSingle: React.FC<TempKeycardSingleProps> = (props) => {
    const { data: keycard } = useQuery(["keycard", props.keycard.keycard_id], ({ queryKey }) => Rest.getSingleKeycard(queryKey[1] || ""))
    const { data: departments } = useQuery(["user", props.userId ?? "self", "department", props.keycard.keycard_id], ({ queryKey }) => {
        if (queryKey[1] === 'self') {
            return Rest.getSelfDepartmentsWithKeycard(queryKey[3])
        }
        return Rest.getUserDepartmentsWithKeycards(queryKey[1], queryKey[3])
    })

    const { data: buildings } = useQuery(["user", props.userId ?? "self", "doors", props.keycard.keycard_id], ({ queryKey }) => {
        if (queryKey[1] === 'self') {
            return Rest.getSelfDoorsKeycard(queryKey[3])
        }
        return Rest.getDoorsByUserAndKeycard(queryKey[1], queryKey[3])
    })

    return (<>
        {
            (buildings && keycard && departments) &&
            <details open>
                <summary>
                    <h3>{i18next.t("temp_keycard")} #{props.idx + 1}</h3>
                    {props.keycard.active_until && <>{i18next.t("active_until")}: {keycard.request?.active_until}</>}
                    </summary>

                    <div className="wrap">
                        <article>
                            <header>
                                {i18next.t("individual_rooms")}
                            </header>
                            <IndividualRoomWrapper
                                buildings={buildings}
                            />
                        </article>
                        <DepartmentWrapper departments={departments} />
                    </div>

                </details>
        }

    </>)
}