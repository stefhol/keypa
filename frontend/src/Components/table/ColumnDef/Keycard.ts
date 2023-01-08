import { ColumnDef } from "@tanstack/react-table";
import { Keycard } from "../../../util/intefaces/Keycard";
export const createKeycardDefColumn = (): ColumnDef<Keycard, unknown>[] => {
    return [

        {
            accessorFn: (row) => row.given_out,
            header: 'Activated',
            id: 'given_out',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.is_deactivated,
            header: 'Is Deactivated',
            id: 'is_deactivated',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.is_given_back,
            header: 'Is Given Back',
            id: 'is_given_back',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.is_locked,
            header: 'Is Locked',
            id: 'is_locked',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.is_lost,
            header: 'Is Lost',
            id: 'is_lost',
            footer: info => info.column.id,
        },
    ]

}
export const createKeycardDefColumnExtended = (): ColumnDef<Keycard, unknown>[] => {
    return [

        {
            accessorFn: (row) => row.given_out,
            header: 'Activated',
            id: 'given_out',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.is_deactivated,
            header: 'Is Deactivated',
            id: 'is_deactivated',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.is_given_back,
            header: 'Is Given Back',
            id: 'is_given_back',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.is_locked,
            header: 'Is Locked',
            id: 'is_locked',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.is_lost,
            header: 'Is Lost',
            id: 'is_lost',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => {
                if (row.request?.accept) return "Accept"
                if (row.request?.pending) return "Pending"
                if (row.request?.reject) return "Reject"
            },
            header: 'Status',
            id: 'status',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.request?.active,
            header: 'Active',
            id: 'active',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.request?.active_until,
            header: 'Active Until',
            id: 'active_until',
            footer: info => info.column.id,
        },


    ]

}
