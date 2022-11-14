import { useQuery } from "@tanstack/react-query"
import { Outlet, useNavigate } from "react-router-dom"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { Rest } from "../../util/Rest"
import React from 'react'
import { useLoading } from "../../hooks/useLoading"

export interface ShowAllUsersProps { }
export const ShowAllUsers: React.FC<ShowAllUsersProps> = (props) => {
    const navigate = useNavigate()
    const { data: userData, isLoading } = useQuery(["users"], Rest.getUsers)
    const columns = React.useMemo(() => createBasicColumns(userData?.[0] || {}), [userData])
    useLoading(isLoading)
    return (<>
        <h1>Alle Nutzer</h1>
        {
            userData &&
            <Table data={userData}
                rowAction={
                    [
                        {
                            element: <button>Ändern</button>,
                            onClick(idx) {
                                navigate(`change-worker/${userData[idx].user_id}`)
                            },
                        },
                        {
                            element: <button>Keycards</button>,
                            onClick(idx) {
                                navigate(`/user/${userData[idx].user_id}/keycard`)
                            },
                        },
                        {
                            element: <button>User</button>,
                            onClick(idx) {
                                navigate(`/user/${userData[idx].user_id}`)
                            },
                        },
                        {
                            element: <button>Antraege</button>,
                            onClick(idx) {
                                navigate(`/user/${userData[idx].user_id}/request`)
                            },
                        }
                    ]
                }
                columns={columns} />
        }</>)
}