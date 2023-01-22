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
        <main>
            <h1>
                {i18next.t("home")}
            </h1>
            <a href="user">
                {i18next.t("myarea")}
            </a><br />
            <h3>
                {i18next.t("managment")}
            </h3>
            <a href="request">
                {i18next.t("requests")}
            </a><br />
            <a href="/keycard">
                {i18next.t("keycard_overview")}
            </a><br />
            <a href="users">
                {i18next.t("user_overview")}
            </a><br />
            <h3>
                {i18next.t("tools")}
            </h3>
            <a href="logs">
                {i18next.t("log_files")}
            </a><br />
            <h3>
                {i18next.t("util")}
            </h3>
            <a href="use-keycard">
                {i18next.t("demo_use_keycard")}
            </a><br />
        </main>
    </>)
}
