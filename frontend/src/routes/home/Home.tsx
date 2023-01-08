import React from "react";
import { useNavigate } from "react-router-dom";
import UserContext from "../../context/UserContext";

export interface HomeProps { }
export const Home: React.FC<HomeProps> = (props) => {
    const { is_admin, is_leader, is_worker } = React.useContext(UserContext);
    const navigate = useNavigate()
    React.useEffect(() => {
        if (!(is_admin || is_leader || is_worker)) {
            navigate("/user")
        }
    }, [is_admin, is_leader, is_worker]);
    return (<>
        <main>
            <h1>Home</h1>
            <a href="user">
                Mein Bereich
            </a><br />
            <h3>Verwaltung</h3>
            <a href="request">
                Antragsformulare
            </a><br />
            <a href="/keycard">
                Keycard Übersicht
            </a><br />
            <a href="users">
                Nutzer Übersicht
            </a><br />
            <h3>Tools</h3>
            <a href="logs">
                Logdateien
            </a><br />
            <h3>Util</h3>
            <a href="use-keycard">
                Demo Keycard Nutzen
            </a><br />
            <a href="email">
                Demo Email
            </a><br />
        </main>
    </>)
}
