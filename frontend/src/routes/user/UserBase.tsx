import { Outlet, useNavigate } from "react-router-dom"
import { AuthorizedBuildings, BuildingFC } from "./keys/Key"

export interface UserBaseProps { }
export const UserBase: React.FC<UserBaseProps> = (props) => {
    const navigate = useNavigate()
    return (<main>

        <Outlet />
    </main>)
}