import { useQuery } from "@tanstack/react-query"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { useLoading } from "../../hooks/useLoading"
import { Rest } from "../../util/Rest"

export interface LogsProps { }
export const Logs: React.FC<LogsProps> = (props) => {
    const { data: userData, isLoading } = useQuery(["users"], Rest.getUsers)
    useLoading(isLoading)

    return (<>
        <div className="container">
            <h1>Keycard Log</h1>
            <Table
                data={demoDataKeycard}
                filter={
                    <>
                        <div className="container">
                            Gebaude:
                            <select value={1}>
                                <option value={1}>
                                    ITZ
                                </option>
                            </select>
                        </div>
                        <div className="container">
                            Raum Nummer:
                            <select value={1}>
                                <option value={1}>
                                    102
                                </option>
                            </select>
                        </div>
                        <div className="container">
                            Keycard:
                            <select value={1}>
                                <option value={1}>

                                </option>
                            </select>
                        </div>
                        <div className="container">
                            Nutzer:
                            <select value={1}>
                                <option value={1}>

                                </option>
                            </select>
                        </div>
                    </>
                }
                columns={createBasicColumns(demoDataKeycard[0])} rowAction={[{
                    element: <button>Detail Ansicht</button>, onClick(rowIndex) {

                    },
                }]} />
            <button>Log Herunterladen</button>
        </div>
        <div className="container">
            <h1>Antrag Log</h1>
            <Table
                data={demoData}
                filter={
                    <div className="container">
                        Nutzer:
                        <select value={1}>
                            <option value={1}>
                                Ulrike Meier
                            </option>
                        </select>
                    </div>
                }
                columns={createBasicColumns(demoData[0])} rowAction={[{
                    element: <button>Detail Ansicht</button>, onClick(rowIndex) {

                    },
                }]} />

            <button>Log Herunterladen</button>
        </div>
    </>)
}

const demoDataKeycard = [
    {
        Keycard: "011",
        "Raum": "102",
        "Benutzt zum": "01.01.2001 13:44",
        "erfolg": "Ja"
    }
    ,
    {
        Keycard: "010",
        "Raum": "102",
        "Benutzt zum": "01.01.2001 13:43",
        "erfolg": "Nein"
    },
    {
        Keycard: "001",
        "Raum": "102",
        "Benutzt zum": "01.01.2001 12:43",
        "erfolg": "Ja"
    },

]
const demoData = [
    {
        Nachricht: "Keycard entsperrt",
        "geändert am": "21.01.2001 14:13",
        "geändert von": "Ulrike Meier",
        "betroffene Antrag": "114",
        "betroffener Antrag-Tür-Historie": "",
    },
    {
        Nachricht: "Zugriff geändert",
        "geändert am": "21.01.2001 13:13",
        "geändert von": "Ulrike Meier",
        "betroffene Antrag": "120",
        "betroffener Antrag-Tür-Historie": "121",
    },
    {
        Nachricht: "Keycard gesperrt",
        "geändert am": "20.01.2001 23:59",
        "geändert von": "Ulrike Meier",
        "betroffene Antrag": "123",
        "betroffener Antrag-Tür-Historie": "",
    },



]