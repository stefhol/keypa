import { useQuery } from "@tanstack/react-query"
import { Rest } from "../../util/Rest"
import { } from "@tanstack/react-table"
import { Table } from "../../Components/table/Table"
import { Building } from "../../util/intefaces/Buildings"
import { Keys } from "../../util/intefaces/Keys"
import { AuthorizedBuildings } from "../user/keys/Key"
import { ChangeWorker } from "../leader/ChangeWorker"

export interface DashboardProps { }
export const Dashboard: React.FC<DashboardProps> = (props) => {
    return (<>
        <main>
            <a href="user">
                Nutzerbereich
            </a><br />
            <h3>Verwaltung</h3>
            <a href="request">
                Alle austehenden Anfragen fuer Zugeaenge
            </a><br />
            <a href="leader">
                Alle Nutzer
            </a><br />

        </main>
    </>)
}
