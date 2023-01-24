import { ColumnDef } from "@tanstack/react-table";
import i18next from "i18next";
import { Keycard } from "../../../util/intefaces/Keycard";
export const createKeycardDefColumn = (): ColumnDef<Keycard, unknown>[] => {
    return [

        {
            accessorFn: (row) => row.given_out,
            header: i18next.t("activated") as string,
            id: 'given_out',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.is_deactivated,
            header: i18next.t("is_deactivated") as string,
            id: 'is_deactivated',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.is_given_back,
            header: i18next.t("is_given_back") as string,
            id: 'is_given_back',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.is_locked,
            header: i18next.t("is_locked") as string,
            id: 'is_locked',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.is_lost,
            header: i18next.t("is_lost") as string, 
            id: 'is_lost',
            footer: info => info.column.columnDef.header,
        },
    ]

}
export const createKeycardDefColumnExtended = (): ColumnDef<Keycard, unknown>[] => {
    return [

        {
            accessorFn: (row) => row.request?.requester.name,
            header: i18next.t("username") as string,
            id: 'requester',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.given_out,
            header: i18next.t("activated") as string,
            id: 'given_out',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.is_deactivated,
            header: i18next.t("is_deactivated") as string,
            id: 'is_deactivated',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.is_given_back,
            header: i18next.t("is_given_back") as string,
            id: 'is_given_back',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.is_locked,
            header: i18next.t("is_locked") as string,
            id: 'is_locked',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.is_lost,
            header: i18next.t("is_lost") as string,
            id: 'is_lost',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => {
                if (row.request?.accept) return i18next.t("status_accepted") as string
                if (row.request?.pending) return i18next.t("status_pending") as string
                if (row.request?.reject) return i18next.t("status_reject") as string
            },
            header: i18next.t("status") as string,
            id: 'status',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.request?.active,
            header: i18next.t("active") as string,
            id: 'active',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.request?.active_until,
            header: i18next.t("active_until") as string,
            id: 'active_until',
            footer: info => info.column.columnDef.header,
        },


    ]

}
