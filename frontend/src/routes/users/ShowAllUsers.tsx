import { useQuery } from "@tanstack/react-query"
import { useNavigate } from "react-router-dom"
import { Table } from "../../Components/table/Table"
import { Rest } from "../../util/Rest"
import React, { useMemo } from 'react'
import { useLoading } from "../../hooks/useLoading"
import { createUserDefColumn } from "../../Components/table/ColumnDef/User"
import i18next from "i18next"

export interface ShowAllUsersProps { }
export const ShowAllUsers: React.FC<ShowAllUsersProps> = (props) => {
    const navigate = useNavigate()
    const { data: userData, isLoading } = useQuery(["users"], Rest.getUsers)
    const [filter, setFilter] = React.useState({ role: "" } as { role: string });
    const filteredUserData = useMemo(() => {
        if (filter.role !== "") {
            let role = Number(filter.role);
            return userData?.filter(val => val.role_id == role)
        }
        return userData
    }, [filter.role, userData])
    useLoading(isLoading)
    return (<>
        <h1>Alle Nutzer</h1>
        {
            userData &&
            <Table

                outerClassName="absolute"
                data={filteredUserData}
                filter={
                    <>
                        <select onChange={e => setFilter({ role: e.target.value })}>
                            <option value={""}>
                            </option>
                            <option value={"0"}>
                                {i18next.t("admin")}
                            </option>
                            <option value={"1"}>
                                {i18next.t("leader_managment")}
                            </option>
                            <option value={"2"}>
                                {i18next.t("worker_managment")}
                            </option>
                        </select>
                    </>

                }
                rowAction={
                    [

                        {
                            element: <button className="outline contrast">Nutzer anpassen</button>,
                            onClick(idx) {
                                navigate(`/user/${userData[idx].user_id}`)
                            },
                        },
                        // {
                        //     element: <button>Anträge</button>,
                        //     onClick(idx) {
                        //         navigate(`/user/${userData[idx].user_id}/request`)
                        //     },
                        // }
                    ]
                }
                columns={createUserDefColumn()} />
        }</>)
}