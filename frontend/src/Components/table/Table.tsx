import * as React from 'react'
import ReactDOM from 'react-dom/client'
import { compareAsc, format, isValid } from 'date-fns'
import '../../css/table.css'
import {
    ColumnDef,
    createColumnHelper,
    flexRender,
    getCoreRowModel,
    useReactTable,
} from '@tanstack/react-table'


const columnHelper = createColumnHelper<{}>()
// Give our default column cell renderer editing superpowers!
const defaultColumn: Partial<ColumnDef<{}>> = {
    cell: ({ getValue, row: { index }, column: { id }, table }) => {
        const initialValue = getValue()
        // We need to keep and update the state of the cell normally
        switch (typeof initialValue) {
            case "bigint":
            case "number":
                return <>{initialValue}</>
            case "boolean":
                return <>{initialValue ? "x" : ""}</>
            default:
                if (!initialValue) {
                    return <></>
                } else if (Array.isArray(initialValue)) {
                    return <>{initialValue.length}</>
                }
                else if (isValid(new Date(initialValue as string))) {
                    return <>{format(new Date(initialValue as string), 'dd.MM.yyyy hh:mm')} </>
                } else if (typeof initialValue === "object") {
                    //@ts-ignore
                    return <>{initialValue?.name || "x"}</>
                }
                else {
                    return <>{initialValue}</>
                }
        }

    },
}
export const createBasicColumns = (obj: {}) => {
    const columns = [] as ColumnDef<{}>[]
    for (const key in obj) {
        if (Object.prototype.hasOwnProperty.call(obj, key)) {
            //@ts-ignore
            const element = obj[key] as unknown;
            if (!key.includes("id"))
                columns.push(columnHelper.accessor(key, {
                    // cell: info => info.getValue(),
                    footer: info => info.column.id,
                }))
        }
    }
    return columns
}


interface ITableProps {
    data?: {}[]
    columns: ColumnDef<{}>[]
    onTableRowClick: (index: number) => void
}
export const Table: React.FC<ITableProps> = (props) => {
    const data = React.useMemo(() => props.data || [], [props.data])
    const columns = React.useMemo(() => props.columns || [], [props.columns])
    const table = useReactTable({
        data,
        columns: columns,
        defaultColumn: defaultColumn,
        getCoreRowModel: getCoreRowModel(),
    })

    return (
        <table>
            <thead>
                <tr><td colSpan={100}>
                    Suche: <input></input>
                </td></tr>
                {table.getHeaderGroups().map(headerGroup => (
                    <tr key={headerGroup.id}>
                        <th key="edit"></th>
                        {headerGroup.headers.map(header => (

                            <th key={header.id}>
                                {header.isPlaceholder
                                    ? null
                                    : flexRender(
                                        header.column.columnDef.header,
                                        header.getContext()
                                    )}
                            </th>
                        ))}
                    </tr>
                ))}
            </thead>
            <tbody>
                {table.getRowModel().rows.map(row => (
                    <tr key={row.id}
                    >
                        <td>
                            <button
                                onClick={(e) => {
                                    e.preventDefault()
                                    props.onTableRowClick(row.index)

                                }}
                            >Ändern</button>
                        </td>
                        {row.getVisibleCells().map(cell => (
                            <td key={cell.id}>
                                {flexRender(cell.column.columnDef.cell, cell.getContext())}
                            </td>
                        ))}
                    </tr>
                ))}
            </tbody>
            <tfoot>
                {table.getFooterGroups().map(footerGroup => (
                    <tr key={footerGroup.id}>
                        <th key="edit"></th>
                        {footerGroup.headers.map(header => (
                            <th key={header.id}>
                                {header.isPlaceholder
                                    ? null
                                    : flexRender(
                                        header.column.columnDef.footer,
                                        header.getContext()
                                    )}
                            </th>
                        ))}
                    </tr>
                ))}
            </tfoot>
        </table>
    )
}

