import { ColumnDef } from "@tanstack/react-table";
import { Request } from '../../../util/intefaces/Request'
export const createRequestDefColumn = (): ColumnDef<Request, unknown>[] => {
    return [

        {
            accessorFn: (row) => row.requester.name,
            header: 'Requester',
            id: "name",
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.description,
            header: 'Description',
            id: 'description',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.active_until,
            header: 'Active Until',
            id: 'active_until',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.request_type,
            header: 'Type',
            id: 'request_type',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => {
                if (row.accept) return "Accept"
                if (row.pending) return "Pending"
                if (row.reject) return "Reject"
            },
            header: 'Status',
            id: 'status',
            footer: info => info.column.id,
        },

        {
            accessorFn: (row) => row.changed_at,
            header: 'Changed At',
            id: 'changed_at',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.created_at,
            header: 'Created At',
            id: 'created_at',
            footer: info => info.column.id,
        },



    ]

}
