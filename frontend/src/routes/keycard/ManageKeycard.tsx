import { useQuery } from "@tanstack/react-query";
import { format } from "date-fns";
import i18next from "i18next";
import React from "react";
import { useParams } from "react-router-dom";
import UserContext from "../../context/UserContext";
import { Keycard } from "../../util/intefaces/Keycard";
import { Rest } from "../../util/Rest";
import { transBool } from "../../util/trans";

export interface ManageKeycardProps { }
export const ManageKeycard: React.FC<ManageKeycardProps> = (props) => {
    const { requestId } = useParams()
    const { data, refetch } = useQuery(["keycard", requestId], ({ queryKey }) => Rest.getSingleKeycard(queryKey[1] || ""))
    return (<>
        <h1>{i18next.t("changes_keycard")}</h1>
        {data && <ManageKeycardForm keycard={data} refetch={refetch} />}
    </>)
}
export interface ManageKeycardFormProps { keycard: Keycard, refetch: () => void }
export const ManageKeycardForm: React.FC<ManageKeycardFormProps> = (props) => {
    const [isRueckgabeButtonClicked, setIsRueckgabeButtonClicked] = React.useState(false);
    const { is_admin, is_leader, is_worker } = React.useContext(UserContext);
    return (<>
        <h2>{i18next.t("keycard_status")}</h2>
        <p>{i18next.t("active_until")}  {props.keycard.active_until ? format(new Date(props.keycard.active_until), "dd.MM.yyyy") : ""}</p>
        <p>{i18next.t("given_out")}: {props.keycard.given_out ? format(new Date(props.keycard.given_out), "dd.MM.yyyy") : ""}</p>
        <p>{i18next.t("for_ever_deactivated")}: {transBool(props.keycard.is_deactivated)}</p>
        <p>{i18next.t("is_given_back")}: {transBool(props.keycard.is_given_back)}</p>
        <p>{i18next.t("is_locked")}: {transBool(props.keycard.is_locked)}</p>
        <p>{i18next.t("is_lost")}: {transBool(props.keycard.is_lost)}</p>
        {typeof props?.keycard?.request?.payed === "boolean" && <>
            <div>
                {i18next.t("payed")}: {transBool(props?.keycard?.request?.payed)}
            </div>
            {!props?.keycard?.request?.payed && <div>
                {i18next.t("payment_info")} {props.keycard.keycard_id}
            </div>}
        </>}
        <div className="grid">
            {
                !props.keycard.is_deactivated && <>
                    {
                        props.keycard?.given_out ? <>
                            {(props.keycard.is_lost === false) && <button className="outline contrast" onClick={(e) => {
                                e.preventDefault()
                                send(props.keycard.keycard_id, { is_lost: true }).then(() => props.refetch())

                            }}>
                                {i18next.t("mark_lost")}

                            </button>}
                            {(is_leader || is_worker) && <>
                                {(props.keycard.is_locked === false) ? <button className="outline contrast" onClick={(e) => {
                                    e.preventDefault()
                                    send(props.keycard.keycard_id, { is_locked: true }).then(() => props.refetch())

                                }}>
                                    {i18next.t("lock")}
                                </button>
                                    :
                                    <button className="outline contrast" onClick={(e) => {
                                        e.preventDefault()
                                        send(props.keycard.keycard_id, { is_locked: false }).then(() => props.refetch())

                                    }}>
                                        {i18next.t("unlock")}
                                    </button>
                                }
                                {!props.keycard.is_deactivated && <button className="outline contrast" onClick={e => {
                                    e.preventDefault()
                                    send(props.keycard.keycard_id, { is_deactivated: true }).then(() => props.refetch())

                                }} >
                                    {i18next.t("deactivate_for_ever")}
                                </button>}

                                {(props.keycard?.given_out && props.keycard.is_given_back === false) && <button className="outline contrast" onClick={e => {
                                    e.preventDefault()
                                    send(props.keycard.keycard_id, { is_given_back: true }).then(() => props.refetch())

                                }} >
                                    {i18next.t("confirm_give_back")}
                                </button>}
                            </>
                            }

                        </>
                            : <>
                                {(is_worker || is_leader) && <>
                                    <button className="outline contrast" onClick={(e) => {
                                        e.preventDefault();
                                        setIsRueckgabeButtonClicked(true)
                                        send(props.keycard.keycard_id, { is_given_out: true }).then(() => props.refetch())
                                    }}>
                                        {i18next.t("keycard_print")}

                                    </button>
                                </>}
                            </>
                    }





                </>}
        </div>
        {<p>
            {i18next.t("card_info_payment")}
        </p>}
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