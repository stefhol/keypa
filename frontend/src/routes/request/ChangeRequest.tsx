import { useQuery } from "@tanstack/react-query"
import { format } from "date-fns"
import React from "react"
import { useParams } from "react-router-dom"
import { SelectionRef, TreeView } from "../../Components/tree-view/TreeView"
import "../../css/comment.css"
import { useLoading } from "../../hooks/useLoading"
import { Building } from "../../util/intefaces/Buildings"
import { Comment, Request, User } from "../../util/intefaces/Request"
import { Rest } from "../../util/Rest"
import { getCountOfRooms } from "../user/keys/Key"
import { prepareData } from "../user/request/Request"
import { UserInfo } from "../user/UseChange"
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
    const requestId = queryKey[1]
    return await Rest.getDoorsWithRequestId(requestId)
}
export const ChangeRequest: React.FC<ChangeRequestProps> = (props) => {
    const { requestId } = useParams()

    const { data: request, isLoading } = useQuery(["request", requestId || ""], getRequest)
    useLoading(isLoading)

    return (<>
        {request &&
            <ChangeRequestForm data={request} />
        }
    </>)
}

export interface ChangeRequestFormProps { data: Request, }
export const ChangeRequestForm: React.FC<ChangeRequestFormProps> = (props) => {
    const { data: building } = useQuery(["building", props.data.request_id], getBuildingWithDoorGroups)

    const [accept, setAccept] = React.useState(props.data.accept);
    const [reject, setReject] = React.useState(props.data.reject);
    const [pending, setPending] = React.useState(props.data.pending);
    const selection = React.useRef({ getCurrentSelection: () => Selection }) as unknown as SelectionRef;

    return (<>
        <h1>Antrag</h1>
        <form>
            <div className="container">
                <h2>Kontaktinformationen</h2>
                <p>
                    Name: {props.data.requester.name}
                </p>
                <p>
                    Email: {props.data.requester.email}
                </p>
                <p>
                    Rolle: {props.data.requester.role.name}
                </p>
                <p>
                    Tel: +49 151 2549983
                </p>
            </div>

            <div className="container">
                <h2>Beschreibung</h2>
                <p>
                    Ich brauche Zugang zum Labor damit ich ein Experiment durchführen kann.
                </p>
            </div>

            <div className="container">
                <h2>Angefragte Räume</h2>
                <div className="container">
                    <h2>Angefragte Individuelle Räume</h2>
                    <div className="container">
                        <label>
                            <b>Text aus Antrag</b>
                        </label>
                        <p>FIM</p>
                        <p>
                            S104, S105
                        </p>
                    </div>
                    <label>
                        Gebäude auswählen
                        <select value={1}>
                            <option value={1}>FIM</option>
                        </select>
                    </label>
                    <div>
                        <b>Nachtragen</b>
                    </div>
                    <TreeView expanded filter={false} selectionRef={{ current: {} } as any} data={prepareData(demoData)} />
                    <button>Anderes Gebäude hinzufügen</button>
                </div>
                <div className="container">
                    <h2>Angefragte Individuelle Räume</h2>
                    <label>
                        Gebäude auswählen
                        <select value={1}>
                            <option value={1}>FIM</option>
                        </select>
                    </label>
                    <TreeView expanded filter={false} selectionRef={{ current: {} } as any} data={prepareData(demoData)} />
                    <button>Anderes Gebäude hinzufügen</button>
                </div>
                <div className="container">
                    <h2>Angefragte Raumgruppen</h2>
                    <div className="container">
                        <label>
                            <b>IT Security</b>
                        </label>

                        <div>
                            ITZ: S201, S203, S204
                        </div>
                        <div>
                            FIM: S100, S101
                        </div>
                        <button>Löschen</button>
                    </div>
                    <button>Andere Raumgruppe hinzufügen</button>
                </div>



            </div>
            <div className="container">
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

                <button>
                    Änderung Speichern
                </button>
            </div>
        </form>

        <div className="container">
            <CommentBoxFC
                data={props.data.comments || []}
                requester={props.data.requester_id}
            />
        </div>
    </>)
}

export interface CommentProps {
    isRequester: boolean,
    comment: Partial<Comment>
}
export const CommentFC: React.FC<CommentProps> = (props) => {

    return (<>
        <div className={`comment ${props.isRequester && "blue"}`}>

            <span><strong>{props?.comment?.user?.name}</strong></span>
            <span>{props.comment.comment}</span>
            <span className="date">{format(new Date(props?.comment?.written_at || ""), "dd.MM.yyyy hh:mm")}</span>
        </div>
    </>)
}
export interface CommentBoxProps {
    data: Comment[]
    requester: string
}
export const CommentBoxFC: React.FC<CommentBoxProps> = (props) => {
    const [newComment, setNewComment] = React.useState("");
    return (<div className="comment-box">
        <h2>
            Kommunikationsverlauf
        </h2>
        <CommentFC
            comment={{
                user: {
                    ...{} as User,
                    name: "Peter Rolf"
                },
                written_at: Date(),
                comment: "Einige Raueme existieren nicht"
            }}
            isRequester={false}
        />
        <CommentFC
            comment={{
                user: {
                    ...{} as User,
                    name: "Mike Fischer"
                },
                written_at: Date(),
                comment: "Ohja stimmt ich meinte ITZ statt FIM"
            }}
            isRequester
        />
        <div>
            Antwort:
            <textarea
                value={newComment}
                onChange={(e) => setNewComment(e.target.value)}
            />

        </div>
        <button onClick={(e) => { e.preventDefault() }}>Sende Nachricht</button>
    </div>)
}

const demoData: Building[] = [

    {
        building_id: "FIM",
        name: "Fim",
        rooms: [
            {
                building_id: "FIM",
                floor: 1,
                is_sensitive: false,
                name: "S104",
                room_id: "1",
                doors: [
                    {
                        door_id: "1",
                        name: "",
                        room_id: "1"
                    }
                ]
            },
            {
                building_id: "FIM",
                floor: 2,
                is_sensitive: false,
                name: "S204",
                room_id: "1",
                doors: [
                    {
                        door_id: "1",
                        name: "",
                        room_id: "1"
                    }
                ]
            },
            {
                building_id: "FIM",
                floor: 1,
                is_sensitive: true,
                name: "S105",
                room_id: "1",
                doors: [
                    {
                        door_id: "1",
                        name: "",
                        room_id: "1"
                    }
                ]
            }
        ]
    }
]
