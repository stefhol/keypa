import { ColumnDef } from "@tanstack/react-table";
import { Log } from "../../../util/intefaces/Log";

export const createLogDefColumn = (): ColumnDef<Log, unknown>[] => {
    return [
        {
            accessorFn: (log) => log.changed_at,
            header: 'Changed at',
            id: 'changed_at',
            footer: info => info.column.id,
        },
        {
            accessorFn: (log) => log.changed_by?.name,
            header: 'Changed by',
            id: 'changed_by',
            footer: info => info.column.id,

        },
        // {
        //     accessorFn: (log) => !!log.door_to_request_history,
        //     header: 'Has Door Request History',
        //     id: 'door_to_request_history',
        //     footer: info => info.column.id,
        // },
        // {
        //     accessorFn: (log) => !!log.keycard_history,
        //     header: 'Has Keycard History',
        //     id: 'keycard_history',
        //     footer: info => info.column.id,
        // },
        {
            accessorFn: (log) => log.message,
            header: 'Message',
            id: 'message',
            footer: info => info.column.id,
        },
    ]

}
