import { useQuery } from "@tanstack/react-query"
import { useNavigate, useParams } from "react-router-dom"
import { User } from "../../util/intefaces/Request"
import { Rest } from "../../util/Rest"

export interface UserChangeProps { }
export const UserChange: React.FC<UserChangeProps> = (props) => {
    const { data } = useQuery(["self", "user"], Rest.getSelf)
    return (<>
        {data &&
            <>
                <UserInfo data={data} />

            </>
        }
    </>)
}
const getUser = async ({ queryKey }: { queryKey: string[] }) => {
    const userId = queryKey[1]
    return await Rest.getUserByUserId(userId)
}
export const UserSelfChange: React.FC<UserChangeProps> = (props) => {
    const { userId } = useParams()
    const { data } = useQuery(["users", userId || ""], getUser)
    return (<>
        {data &&
            <>
                <UserInfo data={data} />

            </>
        }
    </>)
}
export interface UserInfoProps { data: User }
export const UserInfo: React.FC<UserInfoProps> = ({ data }) => {
    const navigate = useNavigate()

    return (<>
        <h1>Account</h1>
        <p>
            Name: {data.name}<br />
            Email: {data.email}<br />
            Berufsbezeichnung: {data.role.name} <br />
            Ist momentan Verwaltungsmitarbeiter:
            {data.worker ? "Ja" : "Nein"}
        </p>
        <button onClick={(e) => navigate("/")}>Inaktiv schalten</button>
    </>)
}