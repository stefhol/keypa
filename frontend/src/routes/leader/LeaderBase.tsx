import { useQuery } from "@tanstack/react-query"
import { Outlet, useNavigate } from "react-router-dom"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { Rest } from "../../util/Rest"
import React from 'react'
export interface LeaderBaseProps { }
export const LeaderBase: React.FC<LeaderBaseProps> = (props) => {
    const navigate = useNavigate()
    const { data: userData } = useQuery(["users"], Rest.getUsers)
    const columns = React.useMemo(() => createBasicColumns(userData?.[0] || {}), [userData])
    return (<main>

        <Outlet />
    </main>)
}