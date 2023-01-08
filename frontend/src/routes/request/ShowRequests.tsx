import { useQuery } from "@tanstack/react-query"
import React from "react"
import { useNavigate } from "react-router-dom"
import { createRequestDefColumn } from "../../Components/table/ColumnDef/Request"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { useLoading } from "../../hooks/useLoading"
import { Rest } from "../../util/Rest"

export const ShowRequests: React.FC<{}> = (props) => {
    const [filter, setFilter] = React.useState({ request_status: "pending", request_type: "", is_sensitive: "" });
    const queryParams = React.useMemo(() => {
        const params = new window.URLSearchParams()
        if (filter.request_status) params.set("request_status", filter.request_status)
        if (filter.request_type) params.set("request_type", filter.request_type)
        if (filter.is_sensitive) params.set("is_sensitive", filter.is_sensitive)
        return params.toString()
    }, [filter.request_status, filter.request_type, filter.is_sensitive])
    const { data, isLoading } = useQuery(["request", "pending", queryParams],
        ({ queryKey }) => Rest.getRequests(queryKey[2])
    )
    const navigate = useNavigate()
    const { } = useLoading(isLoading)
    return (<>
        {data && <Table
            outerClassName="absolute"
            data={data} columns={createRequestDefColumn()}
            filter={
                <>
                    <select style={{ width: "initial" }}
                        value={filter.request_status}
                        onChange={e => setFilter(prev => ({ ...prev, request_status: e.target.value }))}
                    >
                        <option value={""}></option>
                        <option value={"accept"}>accept</option>
                        <option value={"pending"}>pending</option>
                        <option value={"reject"}>reject</option>
                    </select>
                    <select style={{ width: "initial" }}
                        value={filter.request_type}
                        onChange={e => setFilter(prev => ({ ...prev, request_type: e.target.value }))}
                    >
                        <option value={""}></option>
                        <option value={"room"}>room</option>
                        <option value={"temp"}>temp</option>
                        <option value={"keycard"}>keycard</option>
                    </select>
                    <select style={{ width: "initial" }}
                        value={filter.is_sensitive}
                        onChange={e => setFilter(prev => ({ ...prev, is_sensitive: e.target.value }))}
                    >
                        <option value={""}></option>
                        <option value={"true"}>Ja</option>
                        <option value={"false"}>Nein</option>
                    </select>
                </>
            }
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