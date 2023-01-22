import { useQuery } from "@tanstack/react-query"
import i18next from "i18next"
import React from "react"
import { useNavigate, useParams } from "react-router-dom"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { useLoading } from "../../hooks/useLoading"
import { Rest } from "../../util/Rest"


const getRequestFormUser = async ({ queryKey }: { queryKey: string[] }) => {
    const userId = queryKey[1]
    return await Rest.getRequestsFromUser(userId)
}
export const ShowAllRequestFromUser: React.FC<{}> = (props) => {
    const { userId } = useParams()
    const { data, isLoading } = useQuery(["request", userId || ""], getRequestFormUser)
    const { } = useLoading(isLoading)
    const navigate = useNavigate()
    return (<>
        {data && <Table data={data} columns={createBasicColumns(data[0])}
            rowAction={
                [
                    {
                        element: <button>{i18next.t("change")}</button>,
                        onClick(idx) {
                            navigate(`/request/change-request/${data[idx].request_id}`)
                        },
                    }
                ]
            }

        />}
    </>)
}