import { useQuery } from "@tanstack/react-query"
import { ColumnFiltersState } from "@tanstack/react-table"
import i18next from "i18next"
import React from "react"
import { createKeycardUsageHistoryDefColumn, KeycardUsageHistoryHiddenColumns } from "../../Components/table/ColumnDef/KeycardUsageHistory"
import { createLogDefColumn } from "../../Components/table/ColumnDef/Log"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { useLoading } from "../../hooks/useLoading"
import { User } from "../../util/intefaces/Request"
import { Rest } from "../../util/Rest"

export interface LogsProps { }
export const Logs: React.FC<LogsProps> = (props) => {
    const { data: logData } = useQuery(["logs"], Rest.getLogs)
    const { data: keycardUsageHistory } = useQuery(["keycardUsageHistory"], Rest.getKeycardUsageHistory)
    const [filter, setFilter] = React.useState({
        building_id: "",
        keycard_id: "",
        room_id: "",
        user_id: ""
    });
    const userData = React.useMemo(() => {
        if (Array.isArray(logData)) {
            const userData: { [key: string]: User } = {}
            for (let index = 0; index < logData.length; index++) {
                const element = logData[index];
                userData[element.changed_by_id] = element.changed_by
            }
            let uniqueUserData: User[] = []
            for (const key in userData) {
                if (Object.prototype.hasOwnProperty.call(userData, key)) {
                    uniqueUserData.push(userData[key])
                }
            }
            return uniqueUserData
        }
        return []

    }, [logData?.length])
    const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>(
        []
    )
    const [columnFilters_1, setColumnFilters_1] = React.useState<ColumnFiltersState>(
        []
    )
    React.useEffect(() => {
        setColumnFilters_1(prev => {
            let column = [...prev]
            const building_index = column.findIndex(val => val.id === "building_id")
            if (building_index !== -1) {
                column[building_index].value = filter.building_id
            } else {
                column.push({ value: "", id: "building_id" })
            }
            const room_index = column.findIndex(val => val.id === "room_id")
            if (room_index !== -1) {
                column[building_index].value = filter.room_id
            } else {
                column.push({ value: "", id: "room_id" })
            }
            const keycard_index = column.findIndex(val => val.id === "keycard_id")
            if (room_index !== -1) {
                column[keycard_index].value = filter.keycard_id
            } else {
                column.push({ value: "", id: "keycard_id" })
            }
            const user_index = column.findIndex(val => val.id === "user_id")
            if (room_index !== -1) {
                column[user_index].value = filter.user_id
            } else {
                column.push({ value: "", id: "user_id" })
            }
            return column
        })

    }, [filter.building_id, filter.keycard_id, filter.room_id, filter.user_id]);

    return (<>
        {logData && 
            <div className="my-container">
                <h1>{i18next.t("keycard_usage_history")}</h1>
                <Table
                    data={keycardUsageHistory}
                    columnFilter={{ value: columnFilters_1, set: setColumnFilters_1 }}
                    filter={
                        <div>
                            {i18next.t("building")}:
                            <select value={filter.building_id} onChange={e => setFilter(prev => (
                                { ...prev, building_id: e.target.value }
                            ))}>
                                <option value={""}>
                                </option>
                                {[...new Set(keycardUsageHistory?.map(val => val.building_id))].map((val, idx) => <option key={idx} value={val}>{keycardUsageHistory?.find(f => f.building_id == val)?.building_name}</option>)}
                            </select>
                            {i18next.t("room")}:
                            <select value={filter.room_id} onChange={e => setFilter(prev => (
                                { ...prev, room_id: e.target.value }
                            ))}>
                                <option value={""}>
                                </option>
                                {[...new Set(keycardUsageHistory?.map(val => val.room_id))].map((val, idx) => <option key={idx} value={val}>{keycardUsageHistory?.find(f => f.room_id == val)?.room_name}</option>)}
                            </select>
                            {i18next.t("keycard")}:
                            <select value={filter.keycard_id} onChange={e => setFilter(prev => (
                                { ...prev, keycard_id: e.target.value }
                            ))}>
                                <option value={""}>
                                </option>
                                {[...new Set(keycardUsageHistory?.map(val => val.keycard_id))].map((val, idx) => <option key={idx} value={val}>{val}</option>)}
                            </select>
                            {i18next.t("username")}:
                            <select value={filter.user_id} onChange={e => setFilter(prev => (
                                { ...prev, user_id: e.target.value }
                            ))}>
                                <option value={""}></option>
                                {[...new Set(keycardUsageHistory?.map(val => val.user_id))].map((val, idx) => <option key={idx} value={val}>{keycardUsageHistory?.find(f => f.user_id == val)?.username}</option>)}
                            </select>
                        </div>
                    }
                    defaultHidden={KeycardUsageHistoryHiddenColumns}
                    columns={createKeycardUsageHistoryDefColumn()} rowAction={[]} />
                <button onClick={e => {
                    e.preventDefault()
                    Rest.quickFetch("csv/keycard-usage-history", "GET").then(res => {
                        if (!res.ok) {
                            throw new Error()
                        }
                        return res.text()
                    }).then(res => {
                        const blob = new Blob([res], { type: 'application/text' });
                        const url = URL.createObjectURL(blob);
                        download(url, new Date().toISOString() + '_keycard_usage_history.csv');
                    })
                }}>{i18next.t("download_log")}</button>
            </div>
        }
        <div className="my-container">
            <h1>{i18next.t("request_log")}</h1>
            <Table
                data={logData}
                columnFilter={{ value: columnFilters, set: setColumnFilters }}
                filter={
                    <div className="my-container">
                        {i18next.t("changed_by")}:
                        <select
                            value={columnFilters.find((val) => val.id === "changed_by")?.value as any} onChange={(e) => {
                                const index = columnFilters.findIndex(val => val.id === "changed_by")
                                if (index !== -1) {
                                    setColumnFilters(val => {
                                        let columnFilters = [...val]
                                        columnFilters[index].value = e.target.value
                                        return columnFilters;
                                    })
                                } else {
                                    setColumnFilters(val => {
                                        let columnFilters = [...val]
                                        columnFilters.push({
                                            id: "changed_by",
                                            value: e.target.value
                                        })
                                        return columnFilters;
                                    })
                                }
                            }
                            }>
                            <option key={-1} value={undefined}>
                            </option>
                            {userData?.map((val, idx) => <option key={idx} value={val.name}>
                                {val.name}
                            </option>)}
                        </select>
                    </div>
                }
                columns={createLogDefColumn()} rowAction={[]} />

            <button onClick={e => {
                e.preventDefault()
                Rest.quickFetch("csv/logs", "GET").then(res => {
                    if (!res.ok) {
                        throw new Error()
                    }
                    return res.text()
                }).then(res => {
                    const blob = new Blob([res], { type: 'application/text' });
                    const url = URL.createObjectURL(blob);
                    download(url, new Date().toISOString() + '_logs.csv');
                })
            }}>{i18next.t("download_log")}</button>
        </div>
    </>)
}
const download = (path: string, filename: string) => {
    // Create a new link
    const anchor = document.createElement('a');
    anchor.href = path;
    anchor.download = filename;

    // Append to the DOM
    document.body.appendChild(anchor);

    // Trigger `click` event
    anchor.click();

    // Remove element from DOM
    document.body.removeChild(anchor);
}; 
