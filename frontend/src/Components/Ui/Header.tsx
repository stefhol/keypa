import { Outlet, useNavigate } from "react-router-dom"
import { Rest } from "../../util/Rest"
import { LogoSmall } from "../images/LogoSmall"

export interface HeaderProps { }
export const Header: React.FC<HeaderProps> = (props) => {
    const navigate = useNavigate()
    return (
        <>
            <header>
                <LogoSmall width={40} onClick={() => { navigate("/") }} />
                <button onClick={(e) => {
                    e.preventDefault()
                    navigate("/dashboard")
                }}>
                    Dashboard
                </button>
                <span className="spacer"> </span>
                <button onClick={(e) => {
                    e.preventDefault()
                    Rest.sendLogout().then(res => console.log(res))
                }}>
                    Logout
                </button>

                <button onClick={(e) => {
                    navigate("/login")
                }}>
                    Login
                </button>
            </header>
            <Outlet />
        </>)
}