import { useQuery } from "@tanstack/react-query"
import i18next from "i18next"
import React from "react"
import { redirect, useLocation, useNavigate, useSearchParams } from "react-router-dom"
import { createRequestDefColumn } from "../../Components/table/ColumnDef/Request"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { useLoading } from "../../hooks/useLoading"
import { Rest } from "../../util/Rest"
import { convertToStatusString } from "../../util/status"

export const ShowRequests: React.FC<{}> = (props) => {
    const [searchParams, setSearchParams] = useSearchParams();

    const [filter, setFilter] = React.useState({ request_status: "pending", request_type: "", is_sensitive: "" });
    const queryParams = React.useMemo(() => {
        const params = new window.URLSearchParams()
        if (filter.request_status) params.set("request_status", filter.request_status)
        if (filter.request_type) params.set("request_type", filter.request_type)
        if (filter.is_sensitive) params.set("is_sensitive", filter.is_sensitive)
        return params.toString()
    }, [filter.request_status, filter.request_type, filter.is_sensitive])
    React.useEffect(() => {
        setSearchParams()
        const params = new window.URLSearchParams()
        params.set('status', filter.request_status)
        setSearchParams(params)
    }, [filter.request_status]);
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
                        <option value={"accept"}>{i18next.t("status_accepted")}</option>
                        <option value={"pending"}>{i18next.t("status_pending")}</option>
                        <option value={"reject"}>{i18next.t("status_reject")}</option>
                    </select>
                    <select style={{ width: "initial" }}
                        value={filter.request_type}
                        onChange={e => setFilter(prev => ({ ...prev, request_type: e.target.value }))}
                    >
                        <option value={""}></option>
                        <option value={"room"}>{i18next.t("room")}</option>
                        <option value={"temp"}>{i18next.t("temp")}</option>
                        <option value={"keycard"}>{i18next.t("keycard")}</option>
                    </select>
                    <select style={{ width: "initial" }}
                        value={filter.is_sensitive}
                        onChange={e => setFilter(prev => ({ ...prev, is_sensitive: e.target.value }))}
                    >
                        <option value={""}></option>
                        <option value={"true"}>{`${i18next.t("is_sensitive")} ${i18next.t("true")}`}</option>
                        <option value={"false"}>{`${i18next.t("is_sensitive")} ${i18next.t("false")}`}</option>
                    </select>
                </>
            }
            rowAction={
                [
                    {
                        element: <button>
                            {i18next.t("change")}
                        </button>,
                        onClick(idx) {
                            navigate(`/request/change-request/${data[idx].request_id}?status=${convertToStatusString(data[idx])}`)
                        },
                    }
                ]
            }

        />}
    </>)
}
