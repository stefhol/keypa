import { ColumnDef } from "@tanstack/react-table";
import { KeycardUsageHistory } from "../../../util/intefaces/KeycardUsageHistory";
export const createKeycardUsageHistoryDefColumn = (): ColumnDef<KeycardUsageHistory, unknown>[] => {
    return [

        {
            accessorFn: (row) => row.building_id,
            header: 'Building Id',
            id: 'building_id',
            footer: info => info.column.id,

        },
        {
            accessorFn: (row) => row.building_name,
            header: 'Building Name',
            id: 'building_name',
            footer: info => info.column.id,

        },
        {
            accessorFn: (row) => row.door_id,
            header: 'Door Id',
            id: 'door_id',
            footer: info => info.column.id,

        },
        {
            accessorFn: (row) => row.is_sensitive,
            header: 'Is senstive',
            id: 'is_sensitive',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.keycard_history_id,
            header: 'Id',
            id: 'keycard_history_id',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.keycard_id,
            header: 'Keycard Id',
            id: 'keycard_id',
            footer: info => info.column.id,
        },

        {
            accessorFn: (row) => row.role_id,
            header: 'Role Id',
            id: 'role_id',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.room_floor,
            header: 'Room Floor',
            id: 'room_floor',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.room_id,
            header: 'Room Id',
            id: 'room_id',
            footer: info => info.column.id,
        },

        {
            accessorFn: (row) => row.room_name,
            header: 'Room Name',
            id: 'room_name',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.success,
            header: 'Succesfull used',
            id: 'success',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.used_at,
            header: 'Used at',
            id: 'used_at',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.user_id,
            header: 'User Id',
            id: 'user_id',
            footer: info => info.column.id,
        },
        {
            accessorFn: (row) => row.username,
            header: 'Username',
            id: 'username',
            footer: info => info.column.id,
        },

    ]

}

