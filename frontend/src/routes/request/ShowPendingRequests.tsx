import { useQuery } from "@tanstack/react-query"
import { useNavigate } from "react-router-dom"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { useLoading } from "../../hooks/useLoading"
import { Rest } from "../../util/Rest"

export const ShowPendingRequests: React.FC<{}> = (props) => {
    const { data, isLoading } = useQuery(["request", "pending"], Rest.getPendingRequests)
    const navigate = useNavigate()

    const { } = useLoading(isLoading)
    return (<>
        {data && <Table data={data} columns={createBasicColumns(data[0])}
            rowAction={
                [
                    {
                        element: <button>Ändern</button>,
                        onClick(idx) {
                            navigate(`/request/change-request/${data[idx].request_id}`)
                        },
                    }
                ]
            }

        />}
    </>)
}