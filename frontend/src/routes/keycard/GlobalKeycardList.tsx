import { createBasicColumns, Table } from "../../Components/table/Table"
export interface GlobalKeycardListProps { }
export const GlobalKeycardList: React.FC<GlobalKeycardListProps> = (props) => {

    return (<>
        <Table
            filter={
                <span className="container">
                    Nur ablaufende Keycards anzeigen <input type={"checkbox"} checked />
                </span>
            }
            rowAction={[
                {
                    element: <button>Per Email kontakieren</button>,
                    onClick(rowIndex) {

                    },
                },
                {
                    element: <button>Nutzer ansehen</button>,
                    onClick(rowIndex) {

                    },
                },
            ]}
            data={demoData}
            columns={createBasicColumns(demoData[0])}
        />

    </>)
}
const demoData = [
    {
        Nutzer: "Herbert Traum",
        "Rückgabe fällig": "in 2 Tagen",
        Grund: "Ablauf"
    },
    {
        Nutzer: "Ulrike Meier",
        "Rückgabe fällig": "in 5 Tagen",
        Grund: "Kündigung"
    }
]