import i18next from "i18next";
import React from "react";
import UserContext from "../../context/UserContext";
import { SidebarContext } from "../../util/Provider/SidebarProvider";
import './../../css/sidebar.css'
export interface SidebarProps { }
export const Sidebar: React.FC<SidebarProps> = (props) => {
    const { set, value } = React.useContext(SidebarContext);
    const ref = React.useRef(undefined as unknown as HTMLElement);
    const { is_admin, is_leader, is_worker } = React.useContext(UserContext);
    React.useEffect(() => {
        if (value && (is_admin || is_leader || is_worker)) {
            ref.current.style.display = "block"
        } else {
            ref.current.style.display = "none"
        }
        return () => {

        }
    }, [value]);
    return (<>

        <aside ref={ref}>
            <nav>
                <button className="outline contrast" onClick={e => {
                    e.preventDefault()
                    set(false)
                }} aria-label="Open menu">
                    <svg aria-hidden="true" focusable="false" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"
                        height="16px" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"
                        stroke-linejoin="round">
                        <line x1="18" y1="6" x2="6" y2="18">
                        </line>
                        <line x1="6" y1="6" x2="18" y2="18"></line>
                    </svg>
                </button>
                <ul>

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
            </nav>
        </aside>
    </>)
}