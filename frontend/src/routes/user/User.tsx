import { useQuery } from "@tanstack/react-query"
import { useNavigate, useParams } from "react-router-dom"
import { Building } from "../../util/intefaces/Buildings"
import { User } from "../../util/intefaces/Request"
import { Rest } from "../../util/Rest"
import { KeycardSelf, KeycardsFromUser } from "../keycard/Keycard"
import { AuthorizedBuildings } from "./keys/Key"


export interface SelfUserProps { }
export const SelfUser: React.FC<SelfUserProps> = (props) => {
    const { data: buildings } = useQuery(["self", "doors"], Rest.getSelfDoors)
    const { data: user } = useQuery(["self", "users"], Rest.getSelf)

    return (<>
        {(buildings && user) && <UserFc buildings={buildings || []} user={user} />}
        <h2>Meine Schluesselkarten</h2>
        <KeycardSelf />
    </>)
}
export const UserByUserId: React.FC<SelfUserProps> = (props) => {
    const { userId } = useParams()
    const { data: buildings } = useQuery(["user", userId || "", "doors"], getDoorsByUser)
    const { data: user } = useQuery(["user", userId || ""], getUserById)
    return (<>
        {
            (buildings && user) && <UserFc buildings={buildings} user={user} />
        }
        <h2>Meine Schluesselkarten</h2>
        <KeycardsFromUser />
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
}

const UserFc: React.FC<UserProps> = (props) => {
    const navigate = useNavigate()

    return (<>
        <h1>
            Nutzerbereich 
        </h1>
        <h2>{props.user.name}</h2>
        <p>
            {props.user.email}

        </p>
        <button onClick={(e) => {
            e.preventDefault()
            navigate("/request/add-request")
        }}>Neue Zugaenge anfragen</button>
        <button onClick={(e) => {
            e.preventDefault()
            navigate("account")
        }}>Account</button>
        <AuthorizedBuildings data={props.buildings} />



    </>)
}