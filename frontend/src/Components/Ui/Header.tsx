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
                {
                    window.document.cookie !== "" &&
                    <button onClick={(e) => {
                    e.preventDefault()
                    navigate("/dashboard")
                }}>
                    Dashboard
                    </button>}
                <span className="spacer"> </span>
                {
                    window.document.cookie !== "" &&
                    <button onClick={(e) => {
                        e.preventDefault()
                        Rest.sendLogout().then(res => console.log(res))
                    }}>
                        Logout
                    </button>
                }

                {
                    (window.document.cookie === "" && !window.location.pathname.match("login")) &&
                    <button onClick={(e) => {
                        navigate("/login")
                    }}>
                        Login
                    </button>
                }
            </header>
            <Outlet />
        </>)
}