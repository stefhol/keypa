import { useQuery } from "@tanstack/react-query"
import { format } from "date-fns"
import React from "react"
import { useParams } from "react-router-dom"
import { Comment, Request } from "../../util/intefaces/Request"
import { Rest } from "../../util/Rest"
import { UserInfo } from "../user/UseChange"
import "../../css/comment.css"
import { BuildingFC } from "../user/keys/Key"
import { prepareData } from "../user/request/Request"
export interface ChangeRequestProps { }
const getUser = async ({ queryKey }: { queryKey: string[] }) => {
    const userId = queryKey[1]
    return await Rest.getSingleUser(userId)
}
const getRequest = async ({ queryKey }: { queryKey: string[] }) => {
    const requestId = queryKey[1]
    return await Rest.getSingleRequest(requestId)
}
const getBuildingWithDoorGroups = async ({ queryKey }: { queryKey: string[] }) => {
    const doorGroupId = queryKey[1]
    return await Rest.getDoorsWithDoorGroupId(doorGroupId)
}
export const ChangeRequest: React.FC<ChangeRequestProps> = (props) => {
    const { requestId } = useParams()

    const { data: request } = useQuery(["request", requestId || ""], getRequest)

    return (<>
        {request &&
            <ChangeRequestForm data={request} />
        }
    </>)
}

export interface ChangeRequestFormProps { data: Request, }
export const ChangeRequestForm: React.FC<ChangeRequestFormProps> = (props) => {
    const { data: building } = useQuery(["building", props.data.door_group_id], getBuildingWithDoorGroups)

    const [accept, setAccept] = React.useState(props.data.accept);
    const [reject, setReject] = React.useState(props.data.reject);
    const [pending, setPending] = React.useState(props.data.pending);
    return (<>
        <form>
            <UserInfo data={props.data.requester} />

            <h2>Beschreibung</h2>
            <p>{props.data.description}</p>
            <label>
                Status:
                <select name="status" onChange={(e) => {
                    let value = e.target.value
                    if (value === "1") {
                        setAccept(true)
                        setReject(false)
                        setPending(false)
                    }
                    if (value === "2") {
                        setAccept(false)
                        setReject(true)
                        setPending(false)
                    }
                    if (value === "3") {
                        setAccept(false)
                        setReject(false)
                        setPending(true)
                    }
                }}>
                    <option value="1" selected={accept}>Akzeptieren</option>
                    <option value="2" selected={reject}>Ablehnen</option>
                    <option value="3" selected={pending}>Ausstehend</option>
                </select>
            </label>
            <BuildingFC value={prepareData(building || [])} />

            <button>
                Aenderung Speichern
            </button>
            <CommentBoxFC
                data={props.data.comments || []}
                requester={props.data.requester_id}
            />
        </form>
    </>)
}

export interface CommentProps {
    isRequester: boolean,
    comment: Comment
}
export const CommentFC: React.FC<CommentProps> = (props) => {

    return (<>
        <div className={`comment ${props.isRequester && "blue"}`}>
            <span><strong>{props.comment.user.name}</strong></span>
            <span>{props.comment.comment}</span>
            <span className="date">{format(new Date(props.comment.written_at), "dd.MM.yyyy hh:mm")}</span>
        </div>
    </>)
}
export interface CommentBoxProps {
    data: Comment[]
    requester: string
}
export const CommentBoxFC: React.FC<CommentBoxProps> = (props) => {
    const [newComment, setNewComment] = React.useState("");
    return (<>
        {props.data.map(val => <CommentFC
            comment={val}
            isRequester={val.user_id === props.requester}
        />)}
        <div>
            Antwort:
            <textarea
                value={newComment}
                onChange={(e) => setNewComment(e.target.value)}
            />

        </div>
        <button onClick={(e) => { e.preventDefault() }}>Sende Nachricht</button>
    </>)
}