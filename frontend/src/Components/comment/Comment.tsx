import { useQuery } from "@tanstack/react-query"
import { format } from "date-fns"
import React from "react"
import { Rest } from "../../util/Rest"
import { Comment } from '../../util/intefaces/Request'
import '../../css/comment.css'
import i18next from "i18next"
export interface CommentViewProps {
    requestId: string,
    requesterId: string,
}
export const CommentView: React.FC<CommentViewProps> = (props) => {
    const { data: comments, refetch } = useQuery(["comment", props.requestId], ({ queryKey }) => Rest.getComment(queryKey[1]), { refetchInterval: 5000 })
    return (<>
        {comments &&
            <CommentBoxFC
                data={comments}
                {...props}
                refetch={refetch}
            />
        }
    </>)
}
interface CommentProps {
    isRequester: boolean,
    comment: Partial<Comment>

}
const CommentFC: React.FC<CommentProps> = (props) => {

    return (<>
        <div className={`comment ${props.isRequester && "blue"}`}>

            <span><strong>{props?.comment?.user?.name}</strong></span>
            <span>{props.comment.comment}</span>
            <span className="date">{format(new Date(props?.comment?.written_at || ""), "dd.MM.yyyy HH:mm")}</span>
        </div>
    </>)
}
interface CommentBoxProps extends CommentViewProps {
    data: Comment[]
    refetch: () => void;
}
const CommentBoxFC: React.FC<CommentBoxProps> = (props) => {
    const [newComment, setNewComment] = React.useState("");
    return (<div className="comment-box">
        <header>
            {i18next.t("communication_history")}
        </header>
        {props.data.map((val, idx) => <CommentFC
            key={idx}
            comment={val}
            isRequester={val?.user?.user_id == props.requesterId}
        />)}

        <div>
            <textarea style={{ width: 300, height: 100, maxWidth: 300, minWidth: 300, maxHeight: 400 }}
                value={newComment}
                onChange={(e) => setNewComment(e.target.value)}
            />

        </div>
        <button
            disabled={!newComment}
            onClick={(e) => {
            e.preventDefault()
            Rest.createComment(props.requestId, { comment: newComment }).then(() =>
                props.refetch()
            );
            }}>{i18next.t("send_response")}
        </button>
    </div>)
}

export default CommentView