import { Outlet } from "react-router-dom"

export interface UserBaseProps { }
export const UserBase: React.FC<UserBaseProps> = (props) => {
    return (<main className="container">

        <Outlet />
    </main>)
}