import i18next from "i18next"
import React from "react"
import { Outlet, useNavigate } from "react-router-dom"
import { Rest } from "../../util/Rest"
import { LogoSmall } from "../images/LogoSmall"

export interface HeaderProps { }
export const Header: React.FC<HeaderProps> = (props) => {
    const navigate = useNavigate()
    const [selectedLanguage, setSelectedLanguage] = React.useState(i18next.language);
    return (
        <>
            <header>
                <LogoSmall width={40} onClick={() => { navigate("/") }} />
                {
                    window.document.cookie !== "" &&
                    <button onClick={(e) => {
                    e.preventDefault()
                            navigate("/home")
                }}>
                            {i18next.t("home")}
                    </button>}
                <span className="spacer"> </span>
                <select value={selectedLanguage} onChange={e => {
                    i18next.changeLanguage(e.target.value);
                    setSelectedLanguage(e.target.value)
                    localStorage.setItem("language", e.target.value)
                    window.location.reload()

                }}>
                    <option value="en">English</option>
                    <option value="de">German</option>
                </select>
                {
                    window.document.cookie.match("token") &&
                    <button onClick={(e) => {
                            e.preventDefault()
                            Rest.sendLogout().then(() => {
                                navigate("/")
                            })
                    }}>
                            {i18next.t("logout")}
                    </button>
                }

                {
                    (!window.document.cookie.match("token") && !window.location.pathname.match("login")) &&
                    <button onClick={(e) => {
                        navigate("/login")
                    }}>
                            {i18next.t("login")}
                    </button>
                }
            </header>
            <Outlet />
        </>)
}