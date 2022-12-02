import { useNavigate } from "react-router-dom"
import { createBasicColumns, Table } from "../../Components/table/Table"
import React from 'react'
export interface PropositionBaseProps { }
export const PropositionBase: React.FC<PropositionBaseProps> = (props) => {
    const navigate = useNavigate()
    return (<>
        <Table
            filter={<span><label>finished <input type='checkbox' /></label></span>}
            data={demoData}
            rowAction={
                [
                    {
                        element: <button>Bearbeiten</button>,
                        onClick() {
                            navigate(`demo`)
                        },
                    },
                ]
            }
            columns={createBasicColumns(demoData[0])} />
    </>)
}
const demoData = [
    {
        from: "Bernd Habeck",
        created_at: new Date().toISOString(),
        changed_at: new Date().toISOString(),
        finished: false
    },
    {
        from: "Luisa Storch",
        created_at: new Date().toISOString(),
        changed_at: new Date().toISOString(),
        finished: false
    }
]