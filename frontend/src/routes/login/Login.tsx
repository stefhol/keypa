import React from "react";
import { Rest } from "../../util/Rest";
import '../../css/login.css'
import { useNavigate } from "react-router-dom";
import { useLoading } from "../../hooks/useLoading";
import UserContext from "../../context/UserContext";
import { decodeToken } from "../../util/token";
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
        <p>Login</p>
        <form
            className="login"
            onSubmit={e => {
                e.preventDefault()
                startLoading()
                Rest.sendLogin({
                    email: name,
                    password
                }).then(res => {
                    setError("")
                    const params = new window.URLSearchParams(document.cookie)
                    if (set) set(decodeToken(params))
                    navigate("/home")
                    location.reload()
                })
                    .catch(err => {
                        console.log(err);

                        setError("Login nicht erfolgreich")
                    })
                    .finally(() => {

                        stopLoading()
                    })
            }}>
            <input
                autoComplete="current-password"
                value={name}
                onChange={e => setName(e.target.value)}
            />
            <input
                autoComplete="username"
                value={password}
                type="password"
                onChange={e => setPassword(e.target.value)}
            />
            <label>
                Angemeldet bleiben:
                <input type={
                    "checkbox"
                } />
            </label>
            <button>
                Login
            </button>
            <p>{error}</p>
        </form>
    </main>)
}