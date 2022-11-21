import { useQuery } from "@tanstack/react-query"
import { format } from "date-fns"
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
        {(buildings && user) && <UserFc buildings={buildings || []} user={user} isSelf />}
    </>)
}
export const UserByUserId: React.FC<SelfUserProps> = (props) => {
    const { userId } = useParams()
    const { data: buildings } = useQuery(["user", userId || "", "doors"], getDoorsByUser)
    const { data: user } = useQuery(["user", userId || ""], getUserById)
    return (<>
        {
            (buildings && user) && <UserFc buildings={buildings} user={user} isSelf={false} />
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
}

const UserFc: React.FC<UserProps> = (props) => {
    const navigate = useNavigate()

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
                +49 151 25894930
            </p>
        </div>
        <div className="container">
            {props.isSelf && <button onClick={(e) => {
                e.preventDefault()
                navigate("/request/add-request")
            }}>Neuen Antrag stellen</button>
            }
            <button onClick={(e) => {
                e.preventDefault()
                navigate("account")
            }}>Account</button>
        </div>
        <div className="container">
            <AuthorizedBuildings data={props.buildings} />

        </div>
        <div className="container">
            <h2>Schlüsselkarten</h2>
            <KeycardSelf />
        </div>
        <div className="container">
            <h2>Anträge</h2>
            <table>
                <thead>
                    <tr>
                        <th>

                        </th>
                        <th>
                            created_at
                        </th>
                        <th>
                            changed_at
                        </th>
                        <th>
                            comments
                        </th>
                        <th>
                            accept
                        </th>
                        <th>
                            reject
                        </th>
                        <th>
                            pending
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <button>Schliessen</button>
                        </td>
                        <td>
                            {format(new Date(), "dd.MM.yyyy hh:mm")}
                        </td>
                        <td>
                            {format(new Date(), "dd.MM.yyyy hh:mm")}
                        </td>
                        <td>
                            3
                        </td>
                        <td>
                            x
                        </td>
                        <td>

                        </td>
                        <td>

                        </td>
                    </tr>
                    <tr>
                        <td>
                            <button>Ansehen</button>
                            <button>Schliessen</button>
                        </td>
                        <td>
                            {format(new Date(), "dd.MM.yyyy hh:mm")}
                        </td>
                        <td>
                            {format(new Date(), "dd.MM.yyyy hh:mm")}
                        </td>
                        <td>
                            1
                        </td>
                        <td>

                        </td>
                        <td>

                        </td>
                        <td>
                            x
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    </>)
}