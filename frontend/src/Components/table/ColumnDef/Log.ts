import { ColumnDef } from "@tanstack/react-table";
import i18next from "i18next";
import { Log } from "../../../util/intefaces/Log";

export const createLogDefColumn = (): ColumnDef<Log, unknown>[] => {
    return [
        {
            accessorFn: (log) => log.changed_at,
            header: i18next.t("changed_at") as string,
            id: 'changed_at',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (log) => log.changed_by?.name,
            header: i18next.t("changed_by") as string,
            id: 'changed_by',
            footer: info => info.column.columnDef.header,

        },
        {
            accessorFn: (log) => log.message,
            header: i18next.t("message") as string,
            id: 'message',
            footer: info => info.column.columnDef.header,
        },
    ]

}
