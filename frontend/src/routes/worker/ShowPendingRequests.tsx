import { useQuery } from "@tanstack/react-query"
import { useNavigate } from "react-router-dom"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { Rest } from "../../util/Rest"

export interface ShowPendingRequestsProps { }
export const ShowPendingRequests: React.FC<ShowPendingRequestsProps> = (props) => {
    const { data } = useQuery(["request", "pending"], Rest.getPendingRequests)
    const navigate = useNavigate()
    return (<>
        {data && <Table data={data} columns={createBasicColumns(data[0])} onTableRowClick={(idx) => {
            navigate(`change-request/${data[idx].request_id}`)
        }} />}
    </>)
}