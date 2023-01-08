import { useQuery } from "@tanstack/react-query"
import { Outlet, useNavigate } from "react-router-dom"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { Rest } from "../../util/Rest"
import React from 'react'
import { useLoading } from "../../hooks/useLoading"
import { createUserDefColumn } from "../../Components/table/ColumnDef/User"

export interface ShowAllUsersProps { }
export const ShowAllUsers: React.FC<ShowAllUsersProps> = (props) => {
    const navigate = useNavigate()
    const { data: userData, isLoading } = useQuery(["users"], Rest.getUsers)
    useLoading(isLoading)
    return (<>
        <h1>Alle Nutzer</h1>
        {
            userData &&
            <Table
                outerClassName="absolute"
                data={userData}
                filter={
                    <>
                        <span>Ist Verwaltung
                        </span>
                        <input type={"checkbox"} />
                        <span>Ist Vorgesetzter <input type={"checkbox"} /></span>
                        <br />
                        <span>Ist Admin <input type={"checkbox"} /></span>
                    </>

                }
                rowAction={
                    [

                        {
                            element: <button>Nutzer anpassen</button>,
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