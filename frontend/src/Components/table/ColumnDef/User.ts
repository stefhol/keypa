import { ColumnDef } from "@tanstack/react-table";
import i18next from "i18next";
import { User } from "../../../util/intefaces/Request";

export const createUserDefColumn = (): ColumnDef<User, unknown>[] => {
    return [

        {
            accessorFn: (row) => row.name,
            header: i18next.t("username") as string,
            id: 'name',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.email,
            header: i18next.t("email") as string,
            id: 'email',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.address,
            header: i18next.t("address") as string,
            id: 'address',
            footer: info => info.column.columnDef.header,
        },
        {
            accessorFn: (row) => row.tel,
            header: i18next.t("tel") as string,
            id: 'tel',
            footer: info => info.column.columnDef.header,
        },

    ]

}
