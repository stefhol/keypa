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
    const { data: userData } = useQuery(["self", "user"], Rest.getSelf)


    const onTableRowClick = (index: number) => {
        console.log(index);

    }
    return (<>
        <main>
            <a href="user">
                Nutzerbereich
            </a><br />
            <a href="worker">
                Verwaltungsmitarbeiterbereich
            </a><br />
            <a href="leader">
                Verwaltungsvorgesetzterbereich
            </a><br />
        </main>
    </>)
}
