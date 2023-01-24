import { ColumnDef } from "@tanstack/react-table";
import i18next from "i18next";
import { KeycardUsageHistory } from "../../../util/intefaces/KeycardUsageHistory";
export const createKeycardUsageHistoryDefColumn = (): ColumnDef<KeycardUsageHistory, unknown>[] => {
    return [
        {
            accessorFn: (row) => row.user_id,
            header: 'User Id',
            id: 'user_id',
            footer: info => info.column.id,

        },
        {
            accessorFn: (row) => row.username,
            header: i18next.t("username") as string,
            id: 'username',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.role_id,
            header: i18next.t("role") as string,
            id: 'role_id',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.is_sensitive,
            header: i18next.t("is_sensitive") as string,
            id: 'is_sensitive',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.success,
            header: i18next.t("success_on_opening_door") as string,
            id: 'success',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.used_at,
            header: i18next.t("used_at") as string,
            id: 'used_at',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.building_name,
            header: i18next.t("building_name") as string,
            id: 'building_name',
            footer: info => info.column.columnDef.header,

        },
        {
            accessorFn: (row) => row.building_id,
            header: 'Building Id',
            id: 'building_id',
            footer: info => info.column.id,

        },
        {
            accessorFn: (row) => row.room_floor,
            header: i18next.t("room_floor") as string,
            id: 'room_floor',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.room_id,
            header: 'Room Id',
            id: 'room_id',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.room_name,
            header: i18next.t("room_name") as string,
            id: 'room_name',
            cell(props) {
                return props.cell.getValue()
            },
            footer: info => info.column.columnDef.header,
        },

        {
            accessorFn: (row) => row.door_id,
            header: i18next.t("door_id") as string,
            id: 'door_id',
            footer: info => info.column.columnDef.header,
            enableHiding: true
        },

        {
            accessorFn: (row) => row.keycard_id,
            header: i18next.t("keycard_id") as string,
            id: 'keycard_id',
            footer: info => info.column.columnDef.header,

        },






    ]

}
export const KeycardUsageHistoryHiddenColumns = ["user_id", "room_id", "building_id", "door_id"]

