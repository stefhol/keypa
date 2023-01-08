import { useQuery } from "@tanstack/react-query"
import { format } from "date-fns"
import React from "react"
import { useNavigate, useParams } from "react-router-dom"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { Building } from "../../util/intefaces/Buildings"
import { Department } from "../../util/intefaces/Departments"
import { User } from "../../util/intefaces/Request"
import { Rest } from "../../util/Rest"
import { Request } from '../../util/intefaces/Request'
import { Keycard } from "../../util/intefaces/Keycard"
import UserContext from "../../context/UserContext"
import { createKeycardDefColumn } from "../../Components/table/ColumnDef/Keycard"
import { createRequestDefColumn } from "../../Components/table/ColumnDef/Request"

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
    buildings: Building[]
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
            Nutzerbereich von {props.user.name}
        </h1>
        <div className="container">
            <h2>Kontaktinformationen</h2>
            <p>
                {props.user.email}
            </p>
            <p>
                {props.user.tel}
            </p>
        </div>
        {(!is_leader) ? <div className="container">
            {props.isSelf && <button onClick={(e) => {
                e.preventDefault()
                navigate("/request/add-request")
            }}>Neuen Antrag stellen</button>
            }
        </div> : <div className="container">
            <button onClick={(e) => {
                e.preventDefault()
                navigate("/request/add-request/keycard")
            }}>Neuen Keycard erstellen</button>
        </div>}
        <div className="container">
            <h2>Individuelle Räume</h2>
            <IndividualRoomWrapper buildings={props.buildings} />
        </div>
        <DepartmentWrapper departments={props.department} />
        <div className="container">
            <h2>Keycards</h2>
            <>

                <Table data={props.keycard}
                    rowAction={
                        [
                            {
                                element: <button>Ändern</button>,
                                onClick(idx) {
                                    navigate(`/keycard/change-request/${props.keycard?.[idx].keycard_id}`)
                                },
                            }
                        ]
                    }

                    columns={createKeycardDefColumn()} />

            </>
        </div>
        <div className="container">
            <h2>Genehmigte Anträge</h2>
            <Table
                columns={createRequestDefColumn()}
                data={props.acceptedRequests}
                rowAction={[{ element: <button>Ansehen</button>, onClick: (rowIndex) => { navigate(`/request/change-request/${props.acceptedRequests?.[rowIndex].request_id}`) } }]}
            // rowAction={ }
            />
        </div>
        <div className="container">
            <h2>Ausstehende Anträge</h2>
            <Table
                columns={createRequestDefColumn()}
                data={props.pendingRequests}
                rowAction={[{ element: <button>Ansehen</button>, onClick: (rowIndex) => { navigate(`/request/change-request/${props.pendingRequests?.[rowIndex].request_id}`) } }]}
            // rowAction={ }
            />
        </div>
    </>)
}
export interface DepartmentWrapperProps { departments: Department[] }
export const DepartmentWrapper: React.FC<DepartmentWrapperProps> = (props) => {

    return (<>
        <div className="container">
            {props.departments.map((val, idx) =>
                <div className="container" key={idx}>
                    <h2>{val.name}</h2>

                    Beinahltet: {val.buildings.map((val, idx) => <div key={idx}>
                        <b>{val.name}:</b>{` ${val.rooms.map((val) => val.name).join(", ")} `}
                    </div>)}
                </div>
            )}
        </div>

    </>)
}
export interface IndividualRoomWrapperProps { buildings: Building[] }
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