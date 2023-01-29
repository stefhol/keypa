import i18next from "i18next";
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
        <main className="container">
            <ul>
            <h1>
                {i18next.t("home")}
            </h1>
                <ul>
                    <li>
                        <a href="user">
                            {i18next.t("myarea")}
                        </a>
                    </li>
                </ul>
                <ul>
                    <h3>
                        {i18next.t("managment")}
                    </h3>
                    <li>
                        <a href="request">
                            {i18next.t("requests")}
                        </a>
                    </li>
                    <li>
                        <a href="/keycard">
                            {i18next.t("keycard_overview")}
                        </a>
                    </li>
                    <li>
                        <a href="users">
                            {i18next.t("user_overview")}
                        </a>
                    </li>
                </ul>
                <ul>
                    <h3>
                        {i18next.t("tools")}
                    </h3>
                    <li>
                        <a href="logs">
                            {i18next.t("log_files")}
                        </a>
                    </li>
                </ul>
                <ul>
                    <h3>
                        {i18next.t("util")}
                    </h3>
                    <li>
                        <a href="use-keycard">
                            {i18next.t("demo_use_keycard")}
                        </a>
                    </li>
                </ul>
            </ul>

        </main>
    </>)
}
