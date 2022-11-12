import { useQuery } from "@tanstack/react-query"
import { Rest } from "../../util/Rest"
import { } from "@tanstack/react-table"
import { Table } from "../../Components/table/Table"
import { Building } from "../../util/intefaces/Buildings"
import { Keys } from "../../util/intefaces/Keys"
import { Key } from "../user/keys/Key"
const getData = async () => {
    return await Rest.getSelfRequests()
}
export interface DashboardProps { }
export const Dashboard: React.FC<DashboardProps> = (props) => {



    const onTableRowClick = (index: number) => {
        console.log(index);

    }
    return (<>
        <main>
            <article>
                <a href="request">
                    Neuen Schluessel anfordern
                </a>
            </article>
            <Key />
        </main>
    </>)
}
