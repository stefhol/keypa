import * as React from 'react'
import ReactDOM from 'react-dom/client'
import { compareAsc, format, isValid } from 'date-fns'
import '../../css/table.css'
import {
    ColumnDef,
    ColumnFiltersState,
    createColumnHelper,
    FilterFn,
    flexRender,
    getCoreRowModel,
    getFacetedMinMaxValues,
    getFacetedRowModel,
    getFacetedUniqueValues,
    getFilteredRowModel,
    getPaginationRowModel,
    getSortedRowModel,
    useReactTable,
} from '@tanstack/react-table'

import {
    RankingInfo,
    rankItem,
} from '@tanstack/match-sorter-utils'


declare module '@tanstack/table-core' {
    interface FilterFns {
        fuzzy: FilterFn<unknown>
    }
    interface FilterMeta {
        itemRank: RankingInfo
    }
}


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
                    return <>{format(new Date(initialValue as string), 'dd.MM.yyyy HH:mm')} </>
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
    outerClassName?: string
    data?: {}[]
    columns: ColumnDef<any, unknown>[]
    rowAction: IAction[]
    filter?: JSX.Element,
    columnFilter?: ColumnFiltersState
}
const fuzzyFilter: FilterFn<any> = (row, columnId, value, addMeta) => {
    // Rank the item
    const itemRank = rankItem(row.getValue(columnId), value)

    // Store the itemRank info
    addMeta({
        itemRank,
    })

    // Return if the item should be filtered in/out
    return itemRank.passed
}

export const Table: React.FC<ITableProps> = (props) => {
    const data = React.useMemo(() => props.data || [], [props.data])
    const columns = React.useMemo(() => props.columns, [props.columns])

    const [globalFilter, setGlobalFilter] = React.useState('')
    const table = useReactTable({
        data,
        columns: columns as any,
        defaultColumn: defaultColumn,
        getCoreRowModel: getCoreRowModel(),

        filterFns: {
            fuzzy: fuzzyFilter,
        },
        state: {
            columnFilters: props.columnFilter,
            globalFilter,
        },
        // onColumnFiltersChange: setColumnFilters,
        onGlobalFilterChange: setGlobalFilter,
        globalFilterFn: fuzzyFilter,
        getFilteredRowModel: getFilteredRowModel(),
        getSortedRowModel: getSortedRowModel(),
        getPaginationRowModel: getPaginationRowModel(),
        getFacetedRowModel: getFacetedRowModel(),
        getFacetedUniqueValues: getFacetedUniqueValues(),
        getFacetedMinMaxValues: getFacetedMinMaxValues(),

    })

    return (
        <div className={`outer-table ${props.outerClassName || ""}`}>
            <table>
                <thead>
                    <tr key={-1}><td colSpan={100}>
                        <span>
                            Suche: <DebouncedInput
                                value={globalFilter ?? ''}
                                onChange={value => setGlobalFilter(String(value))}
                                placeholder="Search all columns..."
                            />
                        </span>
                        {props.filter}
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
                        <tr key={row.index}
                        >
                            <td>
                                {props.rowAction.map((val, idx) => <ButtonTable {...val} rowIndex={row.index} key={`edit${idx}`} />)}
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
        </div>
    )
}
export interface IAction {
    element: JSX.Element,
    onClick: (rowIndex: number) => void
}
export interface ButtonTableProps extends IAction {
    rowIndex: number
}
export const ButtonTable: React.FC<ButtonTableProps> = ({
    element,
    onClick,
    rowIndex
}) => {

    return (React.cloneElement(element, {
        onClick: (e: Event) => {
            e.preventDefault();
            onClick(rowIndex)
        }
    }))
}
// A debounced input react component
function DebouncedInput({
    value: initialValue,
    onChange,
    debounce = 500,
    ...props
}: {
    value: string | number
    onChange: (value: string | number) => void
    debounce?: number
} & Omit<React.InputHTMLAttributes<HTMLInputElement>, 'onChange'>) {
    const [value, setValue] = React.useState(initialValue)

    React.useEffect(() => {
        setValue(initialValue)
    }, [initialValue])

    React.useEffect(() => {
        const timeout = setTimeout(() => {
            onChange(value)
        }, debounce)

        return () => clearTimeout(timeout)
    }, [value])

    return (
        <input {...props} value={value} onChange={e => setValue(e.target.value)} />
    )
}