import { useQuery } from "@tanstack/react-query";
import { format } from "date-fns";
import React from "react";
import { useParams } from "react-router-dom";
import UserContext from "../../context/UserContext";
import { Keycard } from "../../util/intefaces/Keycard";
import { Rest } from "../../util/Rest";

export interface ManageKeycardProps { }
export const ManageKeycard: React.FC<ManageKeycardProps> = (props) => {
    const { requestId } = useParams()
    const { data, refetch } = useQuery(["keycard", requestId], ({ queryKey }) => Rest.getSingleKeycard(queryKey[1] || ""))
    return (<>
        <h1>Änderungen Keycard</h1>
        {data && <ManageKeycardForm keycard={data} refetch={refetch} />}
    </>)
}
export interface ManageKeycardFormProps { keycard: Keycard, refetch: () => void }
export const ManageKeycardForm: React.FC<ManageKeycardFormProps> = (props) => {
    const [isRueckgabeButtonClicked, setIsRueckgabeButtonClicked] = React.useState(false);
    const { is_admin, is_leader, is_worker } = React.useContext(UserContext);
    return (<>
        <h2>Keycard Status</h2>
        <p>Ausgegeben am: {props.keycard.given_out ? format(new Date(props.keycard.given_out), "dd.MM.yyyy") : ""}</p>
        <p>Fuer immer Deaktiviert: {props.keycard.is_deactivated ? "Ja" : "Nein"}</p>
        <p>Zurueckgegeben: {props.keycard.is_given_back ? "Ja" : "Nein"}</p>
        <p>Gesperrt: {props.keycard.is_locked ? "Ja" : "Nein"}</p>
        <p>Verloren: {props.keycard.is_lost ? "Ja" : "Nein"}</p>
        {!props.keycard.is_deactivated && <>
            {(is_leader || is_worker) && <>
                {(props.keycard.is_locked === false) ? <button onClick={(e) => {
                    e.preventDefault()
                    send(props.keycard.keycard_id, { is_locked: true }).then(() => props.refetch())

                }}>
                    Sperren
                </button>
                    :
                    <button onClick={(e) => {
                        e.preventDefault()
                        send(props.keycard.keycard_id, { is_locked: false }).then(() => props.refetch())

                    }}>
                        Entsperren
                    </button>
                }
                {!props.keycard.is_deactivated && <button onClick={e => {
                    e.preventDefault()
                    send(props.keycard.keycard_id, { is_deactivated: true }).then(() => props.refetch())

                }} >Fuer immer Deaktivieren</button>}
                {props.keycard.is_given_back === false && <button onClick={e => {
                    e.preventDefault()
                    send(props.keycard.keycard_id, { is_given_back: true }).then(() => props.refetch())

                }} >Bestaetige Rueckgabe der Karte</button>}
            </>
            }


            {(props.keycard.is_lost === false) && <button onClick={(e) => {
                e.preventDefault()
                send(props.keycard.keycard_id, { is_lost: true }).then(() => props.refetch())

            }}>
                Als verloren melden
            </button>}

            {!props.keycard?.given_out && <button onClick={(e) => {
                e.preventDefault();
                setIsRueckgabeButtonClicked(true)
                send(props.keycard.keycard_id, { is_given_out: true }).then(() => props.refetch())
            }}>
                Karte erstmals aktivieren
            </button>}
            {<p>
                Info: Bei Rueckgabe der Karte: bitte senden sie die Karte per Post oder werfen sie diese in den Briefkasten am Gebaude der Verwaltung
            </p>}
        </>}
    </>)
}
interface ChangeKeycard {
    is_lost?: boolean,
    is_locked?: boolean,
    is_deactivated?: boolean,
    is_given_back?: boolean,
    is_given_out?: boolean,
}

const send = (keycard_id: string, obj: ChangeKeycard) => {
    return Rest.quickAdd(`keycard/${keycard_id}`, "POST", obj)
}