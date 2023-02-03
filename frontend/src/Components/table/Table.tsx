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
import i18next from 'i18next'


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
                return <>{initialValue ? i18next.t("true") : i18next.t("false")}</>
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
    columnFilter?: { value: ColumnFiltersState, set: React.Dispatch<React.SetStateAction<ColumnFiltersState>> }
    defaultHidden?: string[]
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
const makeHidden = (defaultHidden: string[] = []): {} => {
    let obj = {} as { [key: string]: boolean }
    for (let index = 0; index < defaultHidden.length; index++) {
        const element = defaultHidden[index];
        obj[element] = false
    }
    return obj
}
export const Table: React.FC<ITableProps> = (props) => {
    const data = React.useMemo(() => props.data || [], [props.data])
    const columns = React.useMemo(() => props.columns, [props.columns])
    const [columnVisibility, setColumnVisibility] = React.useState(makeHidden(props.defaultHidden))
    const [globalFilter, setGlobalFilter] = React.useState('')
    const [columnFilter, setColumnFilter] = React.useState([] as ColumnFiltersState);
    const table = useReactTable({
        data,
        columns: columns as any,
        defaultColumn: defaultColumn,
        getCoreRowModel: getCoreRowModel(),

        filterFns: {
            fuzzy: fuzzyFilter,
        },
        state: {
            columnFilters: props.columnFilter?.value ?? columnFilter,
            globalFilter,
            columnVisibility,
        },

        onColumnVisibilityChange: setColumnVisibility,
        onColumnFiltersChange: props.columnFilter?.set ?? setColumnFilter,
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
        <div className={`outer-table ${props.outerClassName || ""} grid`}>
            <figure>
                <table>
                    <thead>
                        <tr key={-1}><td colSpan={100} >
                            <div className='table-filter'>
                                <span >
                                    Suche: <DebouncedInput
                                        value={globalFilter ?? ''}
                                        onChange={value => setGlobalFilter(String(value))}
                                        placeholder={i18next.t("search") || ""}
                                    />
                                </span>
                                <span>
                                    {props.filter}
                                </span>
                            </div>
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
                <div className="grid table-nav">
                    <button
                        className="outline contrast"
                        onClick={() => table.setPageIndex(0)}
                        disabled={!table.getCanPreviousPage()}
                    >
                        {'<<'}
                    </button>
                    <button
                        className="outline contrast"
                        onClick={() => table.previousPage()}
                        disabled={!table.getCanPreviousPage()}
                    >
                        {'<'}
                    </button>
                    <button
                        className="outline contrast"
                        onClick={() => table.nextPage()}
                        disabled={!table.getCanNextPage()}
                    >
                        {'>'}
                    </button>
                    <button
                        className="outline contrast"
                        onClick={() => table.setPageIndex(table.getPageCount() - 1)}
                        disabled={!table.getCanNextPage()}
                    >
                        {'>>'}
                    </button>
                    <span className="flex items-center gap-1">
                        <strong>
                            {table.getState().pagination.pageIndex + 1} {i18next.t("of")}{' '}
                            {table.getPageCount()}
                        </strong>
                    </span>
                    <span className="flex items-center gap-1">
                        | {i18next.t("go_to_page")}:
                        <input
                            type="number"
                            defaultValue={table.getState().pagination.pageIndex + 1}
                            onChange={e => {
                                const page = e.target.value ? Number(e.target.value) - 1 : 0
                                table.setPageIndex(page)
                            }}
                            className="border p-1 rounded w-16"
                        />
                    </span>
                    <select
                        value={table.getState().pagination.pageSize}
                        onChange={e => {
                            table.setPageSize(Number(e.target.value))
                        }}
                    >
                        {[10, 20, 30, 40, 50].map(pageSize => (
                            <option key={pageSize} value={pageSize}>
                                {i18next.t("show")} {pageSize}
                            </option>
                        ))}
                    </select>
                </div>
            </figure>
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