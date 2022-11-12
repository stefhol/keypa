import React from "react";
import { Rest } from "../../util/Rest";
import '../../css/login.css'
import { useNavigate } from "react-router-dom";
export interface LoginRequest {
    email: string,
    password: string
}
export interface LoginProps { }
export const Login: React.FC<LoginProps> = (props) => {
    const [name, setName] = React.useState("una_ad@yahoo.com");
    const [password, setPassword] = React.useState("1234");
    const [error, setError] = React.useState("");
    const navigate = useNavigate()
    return (<main className="login"
    >
        <p>Login</p>
        <form
            className="login"
            onSubmit={e => {
                e.preventDefault()
                Rest.sendLogin({
                    email: name,
                    password
                }).then(res => {
                    setError("")
                    navigate("/dashboard")
                })
                    .catch(err => {
                        console.log(err);

                        setError("Login nicht erfolgreich")
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
            <button>
                Login
            </button>
            <p>{error}</p>
        </form>
    </main>)
}