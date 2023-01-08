import { useNavigate } from "react-router-dom"
import { User } from "../../util/intefaces/Request"

export interface UserInfoProps { data: User }
export const UserInfo: React.FC<UserInfoProps> = ({ data }) => {
    const navigate = useNavigate()

    return (<div className="container" >
        <h1>Account Informationen</h1>
        <p>
            Name: {data.name}<br />
            Email: {data.email}<br />
            Role: {data.role_id} <br />
        </p>
    </div>)
}