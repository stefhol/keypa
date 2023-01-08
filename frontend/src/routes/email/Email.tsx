import { useQuery } from "@tanstack/react-query"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { Rest } from "../../util/Rest"

export interface EmailProps { }
export const Email: React.FC<EmailProps> = (props) => {
    const { data } = useQuery(["email"], () => Rest.quickFetchJson<{}[]>("email", "GET"))
    return (<>
        {data &&
            <Table
                data={data}
                columns={createBasicColumns(data[0])}
                rowAction={[]}
            />
        }
    </>)
}