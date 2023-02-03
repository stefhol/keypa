import i18next from "i18next"
import React from "react"
import { Outlet, useNavigate } from "react-router-dom"
import UserContext from "../../context/UserContext"
import { SidebarContext } from "../../util/Provider/SidebarProvider"
import { Rest } from "../../util/Rest"
import { LogoSmall } from "../images/LogoSmall"
import '../../css/header.css'
export interface HeaderProps { }
export const Header: React.FC<HeaderProps> = (props) => {
    const navigate = useNavigate()
    const { loggedIn, set } = React.useContext(UserContext);
    const [selectedLanguage, setSelectedLanguage] = React.useState(i18next.language);
    const { set: setSidebar } = React.useContext(SidebarContext);
    const { is_admin, is_leader, is_worker } = React.useContext(UserContext);
    return (
        <>
            <nav>
                {(is_admin || is_leader || is_worker) && <ul>
                    <li>
                        <button className="outline contrast" onClick={e => {
                            e.preventDefault()
                            setSidebar(true)
                        }} aria-label="Close menu">
                            <svg aria-hidden="true" focusable="false" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" height="16px" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <line x1="3" y1="12" x2="21" y2="12"></line>
                                <line x1="3" y1="6" x2="21" y2="6"></line>
                                <line x1="3" y1="18" x2="21" y2="18"></line>
                            </svg>
                        </button>
                    </li>
                </ul>}
                <ul>
                    <li>
                        <LogoSmall width={40} onClick={() => { navigate("/") }} />
                    </li>
                </ul>
                <ul>
                    <li>
                        {
                            loggedIn &&
                            <button className="outline contrast" onClick={(e) => {
                                e.preventDefault()
                                navigate("/home")
                            }}>
                                {i18next.t("home")}
                            </button>}
                    </li>
                </ul>
                <span className="spacer"> </span>
                <ul>
                    <li>

                    </li>
                </ul>
                <ul>
                    <li>
                        <select value={selectedLanguage} onChange={e => {
                            i18next.changeLanguage(e.target.value);
                            setSelectedLanguage(e.target.value)
                            localStorage.setItem("language", e.target.value)
                            window.location.reload()

                        }}>
                            <option value="en">English</option>
                            <option value="de">German</option>
                        </select>
                    </li>
                </ul>
                <ul>
                    <li>
                        {
                            loggedIn &&
                            <button className="outline contrast" onClick={(e) => {
                                e.preventDefault()
                                Rest.sendLogout().then(() => {
                                    navigate("/")
                                    if (set) set({ loggedIn: false })
                                })
                            }}>
                                {i18next.t("logout")}
                            </button>
                        }

                        {
                            (!loggedIn && !window.location.pathname.match("login")) &&
                            <button className="outline contrast" onClick={(e) => {
                                navigate("/login")
                            }}>
                                {i18next.t("login")}
                            </button>
                        }
                    </li>
                </ul>
            </nav>
            <Outlet />
        </>)
}