import React from "react";
import { Rest } from "../../util/Rest";
import '../../css/login.css'
import { useNavigate } from "react-router-dom";
import { useLoading } from "../../hooks/useLoading";
import UserContext from "../../context/UserContext";
import { decodeToken } from "../../util/token";
import i18next from "i18next";
export interface LoginRequest {
    email: string,
    password: string
}
export interface LoginProps { }
export const Login: React.FC<LoginProps> = (props) => {
    const [name, setName] = React.useState("");
    const [password, setPassword] = React.useState("1234");
    const { startLoading, stopLoading } = useLoading()
    const { set } = React.useContext(UserContext);

    const [error, setError] = React.useState("");
    const navigate = useNavigate()
    return (<main className="login"
    >
        <p>{i18next.t("login")}</p>
        <form
            className="login"
            onSubmit={e => {
                e.preventDefault()
                startLoading()
                Rest.sendLogin({
                    email: name,
                    password
                }).then(_ => {
                    setError("")
                    const params = new window.URLSearchParams(document.cookie)
                    if (set) set(decodeToken(params))
                    navigate("/home")
                    location.reload()
                })
                    .catch(err => {
                        console.log(err);

                        setError(i18next.t("failed_login") as string)
                    })
                    .finally(() => {

                        stopLoading()
                    })
            }}>
            <input
                autoComplete="username"
                value={name}
                onChange={e => setName(e.target.value)}
            />
            <input
                autoComplete="current-password"
                value={password}
                type="password"
                onChange={e => setPassword(e.target.value)}
            />
            <label>
                {i18next.t("stay_logged_in")}:
                <input type={
                    "checkbox"
                } />
            </label>
            <button>
                {i18next.t("login")}
            </button>
            <p>{error}</p>
        </form>
    </main>)
}