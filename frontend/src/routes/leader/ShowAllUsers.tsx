import { useQuery } from "@tanstack/react-query"
import { Outlet, useNavigate } from "react-router-dom"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { Rest } from "../../util/Rest"
import React from 'react'

export interface ShowAllUsersProps { }
export const ShowAllUsers: React.FC<ShowAllUsersProps> = (props) => {
    const navigate = useNavigate()
    const { data: userData } = useQuery(["users"], Rest.getUsers)
    const columns = React.useMemo(() => createBasicColumns(userData?.[0] || {}), [userData])
    return (<>
        <h1>Alle Nutzer</h1>
        {
            userData &&
            <Table data={userData} onTableRowClick={(e) => { navigate(`change-worker/${userData[e].user_id}`) }} columns={columns} />
        }</>)
}