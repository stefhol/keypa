import { useQuery } from "@tanstack/react-query"
import { ColumnFiltersState } from "@tanstack/react-table"
import { addDays, isValid, isWithinInterval } from "date-fns"
import i18next from "i18next"
import React, { useMemo } from "react"
import { useNavigate } from "react-router-dom"
import { createKeycardDefColumnExtended } from "../../Components/table/ColumnDef/Keycard"
import { Table } from "../../Components/table/Table"
import { Rest } from "../../util/Rest"
export interface GlobalKeycardListProps { }
export const GlobalKeycardList: React.FC<GlobalKeycardListProps> = (props) => {
    const { data } = useQuery(['keycard'], Rest.getKeycard)
    const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>(
        []
    )
    const [filterActiveUntil, setFilterActiveUntil] = React.useState(false);
    const filterdData = useMemo(() => data?.filter(val => {
        if (!filterActiveUntil) {
            return true
        }
        if (!val.request || !val.request?.active_until) {
            return false
        }
        const date = new Date(val.request?.active_until)
        if (isValid(date) && val.request?.active_until) {
            return date <= addDays(new Date(), 7) 
        } else {
            return false;
        }


    }), [filterActiveUntil, data?.length])
    console.log(filterdData)
    const navigate = useNavigate()
    return (<>
        {data && 
            <Table
            outerClassName="absolute"
            columnFilter={columnFilters}
            filter={
                <span className="my-container">
                    {i18next.t("show_only_expiraing_keycard")}
                    <input type={"checkbox"} checked={filterActiveUntil}
                        onChange={() => setFilterActiveUntil(prev => !prev)}
                    />
                </span>
            }
            rowAction={[
                {
                    element: <button>{i18next.t("contact_per_email")}</button>,
                    onClick(rowIndex) {
                        if (data[rowIndex].request?.requester.email)
                            window.open(`mailto:${data[rowIndex].request?.requester.email}`);
                    },
                },
                {
                    element: <button>{i18next.t("open_user")}</button>,
                    onClick(rowIndex) {
                        if (data[rowIndex].request?.requester_id)
                            navigate(`/user/${data[rowIndex].request?.requester_id}`)
                    },
                },
            ]}
            data={filterdData}
                columns={createKeycardDefColumnExtended()}
            />}

    </>)
}
