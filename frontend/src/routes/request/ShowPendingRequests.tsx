import { useQuery } from "@tanstack/react-query"
import React from "react"
import { useNavigate } from "react-router-dom"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { useLoading } from "../../hooks/useLoading"
import { Rest } from "../../util/Rest"

export const ShowPendingRequests: React.FC<{}> = (props) => {
    const { data, isLoading, dataUpdatedAt } = useQuery(["request", "pending"], Rest.getPendingRequests)
    const navigate = useNavigate()
    const myData = React.useMemo(() => data?.map(val => ({ ...val, type: "room-request" }) || undefined), [dataUpdatedAt])
    const { } = useLoading(isLoading)
    return (<>
        {myData && <Table data={myData} columns={createBasicColumns(myData[0])}
            filter={
                <>
                    <span className="container">Pending <input checked type={"checkbox"} /> </span>
                    <span className="container">Reject <input type={"checkbox"} /> </span>
                    <span className="container">Accept <input type={"checkbox"} /> </span>
                    <span className="container"><select style={{ width: "initial" }}>
                        <option>room-request</option>
                    </select> </span>
                </>
            }
            rowAction={
                [
                    {
                        element: <button>Ändern</button>,
                        onClick(idx) {
                            navigate(`/request/change-request/${myData[idx].request_id}`)
                        },
                    }
                ]
            }

        />}
    </>)
}