import { ColumnDef } from "@tanstack/react-table";
import { User } from "../../../util/intefaces/Request";

export const createUserDefColumn = (): ColumnDef<User, unknown>[] => {
    return [

        {
            accessorFn: (row) => row.name,
            header: 'Name',
            id: 'name',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.email,
            header: 'Email',
            id: 'email',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.address,
            header: 'Address',
            id: 'address',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.tel,
            header: 'Telephone Number',
            id: 'tel',
            footer: info => info.column.id,
        },

    ]

}
