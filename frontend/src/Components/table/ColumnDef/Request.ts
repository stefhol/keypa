import { ColumnDef } from "@tanstack/react-table";
import i18next from "i18next";
import { Request } from '../../../util/intefaces/Request'
export const createRequestDefColumn = (): ColumnDef<Request, unknown>[] => {
    return [

        {
            accessorFn: (row) => row.requester.name,
            header: i18next.t("requester") as string,

            id: "name",
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.description,
            header: i18next.t("description") as string,
            id: 'description',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.active_until,
            header: i18next.t("active_until") as string,
            id: 'active_until',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.request_type,
            header: i18next.t("type") as string,
            id: 'request_type',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => {
                if (row?.accept) return i18next.t("status_accepted") as string
                if (row?.pending) return i18next.t("status_pending") as string
                if (row?.reject) return i18next.t("status_reject") as string
            },
            header: i18next.t("status") as string,
            id: 'status',
            footer: info => info.column.columnDef.header,
        },

        {
            accessorFn: (row) => row.changed_at,
            header: i18next.t("changed_at") as string,
            id: 'changed_at',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.created_at,
            header: i18next.t("created_at") as string,
            id: 'created_at',
            footer: info => info.column.columnDef.header,
        },



    ]

}
