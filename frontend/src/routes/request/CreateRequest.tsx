import i18next from "i18next"
import { useNavigate } from "react-router-dom"
import { CreateKeycardRequestForm, CreateRequestForm } from "../../Components/request/CreateRequestForm"

export const CreateRoomRequest: React.FC<{}> = (props) => {

    return (<>
        <CreateRequestForm createKeycard={false} title={<h1>{i18next.t("access_request")}</h1>} />
    </>)
}
export const CreateTempRequest: React.FC<{}> = (props) => {

    return (<>
        <CreateRequestForm createKeycard={true} title={<>

            <h1>
                {i18next.t("temp_keycard_request")}

            </h1>
            {i18next.t("temp_keycard_request_info")}
        </>} />
    </>)
}
export const CreateKeycardRequest: React.FC<{}> = (props) => {

    return (<>
        <CreateKeycardRequestForm title={<h1>{i18next.t("keycard_request")}</h1>} />
    </>)
}
export interface RequestPickerProps { }
export const RequestPicker: React.FC<RequestPickerProps> = (props) => {
    const navigate = useNavigate()
    return (<>
        <h2>
            {i18next.t("type_of_request")}
        </h2>
        <div className="grid">
            <button className="outline contrast" onClick={e => {
                e.preventDefault()
                navigate("/request/add-request/room")
            }}>
                {i18next.t("access_request")}
            </button>
            <button className="outline contrast" onClick={e => {
                e.preventDefault()
                navigate("/request/add-request/keycard")
            }} >
                {i18next.t("keycard_request")}

            </button>
            <button className="outline contrast" onClick={e => {
                e.preventDefault()
                navigate("/request/add-request/temp")
            }} >
                {i18next.t("temp_keycard_request")}
            </button>
        </div>
    </>)
}