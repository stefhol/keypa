import { useQuery } from "@tanstack/react-query"
import { useNavigate } from "react-router-dom"
import { User } from "../../util/intefaces/Request"
import { Rest } from "../../util/Rest"

export interface UserChangeProps { }
export const UserChange: React.FC<UserChangeProps> = (props) => {
    const { data } = useQuery(["self", "user"], Rest.getSelf)
    const navigate = useNavigate()
    return (<>
        <h1>Account</h1>
        {data &&
            <>
                <UserInfo data={data} />
                <button onClick={(e) => navigate("/")}>Inaktiv schalten</button>
            </>
        }
    </>)
}
export interface UserInfoProps { data: User }
export const UserInfo: React.FC<UserInfoProps> = ({ data }) => {

    return (<>
        <p>
            Name: {data.name}<br />
            Email: {data.email}<br />
            Berufsbezeichnung: {data.role.name} <br />
            Ist momentan Verwaltungsmitarbeiter:
            {data.worker ? "Ja" : "Nein"}
        </p></>)
}