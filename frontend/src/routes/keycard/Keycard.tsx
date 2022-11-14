import { useQuery } from "@tanstack/react-query"
import { useNavigate, useParams } from "react-router-dom"
import { createBasicColumns, Table } from "../../Components/table/Table"
import { Keycard } from "../../util/intefaces/Keycard"
import { Rest } from "../../util/Rest"

export interface KeycardProps { }
const getKeycardFormUser = async ({ queryKey }: { queryKey: string[] }) => {
    const userId = queryKey[1]
    return await Rest.getKeycardsFromUser(userId)
}
export const KeycardsFromUser: React.FC<KeycardProps> = (props) => {
    const { userId } = useParams();
    const { data } = useQuery(["keycard", userId || ""], getKeycardFormUser)
    return (<>
        <KeycardTable data={data || []} />
    </>)
}
export const KeycardSelf: React.FC<KeycardProps> = (props) => {
    const { data } = useQuery(["self", "keycard"], Rest.getSelfKeycard)
    return (<>
        <KeycardTable data={data || []} />
    </>)
}
export interface KeycardTableProps { data: Keycard[] }
export const KeycardTable: React.FC<KeycardTableProps> = ({ data }) => {
    const navigate = useNavigate()

    return (<>
        <button onClick={(e) => {
            e.preventDefault();
            navigate("/keycard/add-request")
        }}>
            Fuege neue Keycard hinzu
        </button>
        <Table data={data}
            rowAction={
                [
                    {
                        element: <button>Ändern</button>,
                        onClick(idx) {
                            navigate(`/keycard/change-request/${data[idx].keycard_id}`)
                        },
                    }
                ]
            }

            columns={createBasicColumns(data[0])} />
    </>)
}