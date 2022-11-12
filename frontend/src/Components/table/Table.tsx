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
                }
                else {
                    return <>{initialValue}</>
                }
        }

    },
}
const createBasicColumns = (obj: {}) => {
    const columns = []
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
    columns: ColumnDef<{}>
    onTableRowClick: (index: number) => void
}
export const Table: React.FC<ITableProps> = (props) => {
    const data = React.useMemo(() => props.data || [{}], [props.data])

    const table = useReactTable({
        data,
        columns: createBasicColumns(data[0]),
        defaultColumn: defaultColumn,
        getCoreRowModel: getCoreRowModel(),
    })

    return (
        <table>
            <thead>
                {table.getHeaderGroups().map(headerGroup => (
                    <tr key={headerGroup.id}>
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
                        onClick={
                            () => props.onTableRowClick(row.index)
                        }
                    >
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

