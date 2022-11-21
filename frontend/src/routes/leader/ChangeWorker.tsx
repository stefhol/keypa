import { useQuery } from "@tanstack/react-query";
import React from "react";
import { useParams } from "react-router-dom";
import { User } from "../../util/intefaces/Request";
import { Rest } from "../../util/Rest";

export interface ChangeWorkerProps { }
const getUser = async ({ queryKey }: { queryKey: string[] }) => {
    const userId = queryKey[1]
    return await Rest.getSingleUser(userId)
}
export const ChangeWorker: React.FC<ChangeWorkerProps> = (props) => {
    let { userId } = useParams();
    const [boss, setBoss] = React.useState("Rainer Winkler");
    const { data } = useQuery(["user", userId as string], getUser)
    const [isWorker, setIsWorker] = React.useState(true)

    return (<>
        <h1>Rechte vergeben</h1>
        {data &&
            <><h2>Informationen</h2>
                <p>
                    Name: {data.name}<br />
                    Email: {data.email}<br />
                    Berufsbezeichnung: {data.role.name} <br />
                    Ist momentan Verwaltungsmitarbeiter:
                    {data.worker ? "Ja" : "Nein"}
                </p>
                <h2>Daten anpassen</h2>
                <WorkerForm data={data} />
            </>
        }
    </>)
}
export interface WorkerFormProps { data: User }
export const WorkerForm: React.FC<WorkerFormProps> = (props) => {
    const [isLeader, setIsLeader] = React.useState(props.data.is_leader);
    const [isWorker, setIsWorker] = React.useState(!!props.data.worker);
    const [boss, setBoss] = React.useState(props?.data?.worker?.boss?.name || "");
    return (<>
        <form>
            <h3>
                Arbeiter Daten:
            </h3>
            <label>
                Ist Arbeiter:
                <input
                    checked={isWorker}
                    type="checkbox"
                    onChange={(e) => setIsWorker(prev => !prev)}
                />
            </label>
            <h3>
                Vorgesetzter Daten:
            </h3>
            Ist Vorgesetzter:
            <input
                checked={isLeader}
                type="checkbox"
                onChange={(e) => setIsLeader(prev => !prev)}
            />
            <br />
            <button>Absenden</button>
        </form>
    </>)
}