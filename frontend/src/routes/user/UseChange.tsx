import { useQuery } from "@tanstack/react-query"
import { useNavigate } from "react-router-dom"
import { Rest } from "../../util/Rest"

export interface UserChangeProps { }
export const UserChange: React.FC<UserChangeProps> = (props) => {
    const { data } = useQuery(["self", "user"], Rest.getSelf)
    const navigate = useNavigate()
    return (<>
        <h1>Account</h1>
        {data &&
            <>
                <p>
                    Name: {data.name}<br />
                    Email: {data.email}<br />
                    Berufsbezeichnung: {data.role.name} <br />
                    Ist momentan Verwaltungsmitarbeiter:
                    {data.worker ? "Ja" : "Nein"}
                </p>
                <button onClick={(e) => navigate("/")}>Inaktiv schalten</button>
            </>
        }
    </>)
}