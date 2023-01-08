import { useNavigate } from "react-router-dom"
import { CreateKeycardRequestForm, CreateRequestForm } from "../../Components/request/CreateRequestForm"

export const CreateRoomRequest: React.FC<{}> = (props) => {

    return (<>
        <CreateRequestForm createKeycard={false} title={<h1>Zugangsantrag</h1>} />
    </>)
}
export const CreateTempRequest: React.FC<{}> = (props) => {

    return (<>
        <CreateRequestForm createKeycard={true} title={<>
            <h1>Pfand Keycard Antrag</h1>
            <h2>Hinweis: Pfand muss vor Abholung bezahtlt werden</h2>
            <p>Zahlungsinformationen erscheinen im angenommenen Antrag</p>
        </>} />
    </>)
}
export const CreateKeycardRequest: React.FC<{}> = (props) => {

    return (<>
        <CreateKeycardRequestForm title={<h1>Keycard Antrag</h1>} />
    </>)
}
export interface RequestPickerProps { }
export const RequestPicker: React.FC<RequestPickerProps> = (props) => {
    const navigate = useNavigate()
    return (<>
        <h2>
            Typ des Antrags
        </h2>
        <button onClick={e => {
            e.preventDefault()
            navigate("/request/add-request/room")
        }}>Zugangsantrag</button>
        <button onClick={e => {
            e.preventDefault()
            navigate("/request/add-request/keycard")
        }} >Keycardantrag</button>
        <button onClick={e => {
            e.preventDefault()
            navigate("/request/add-request/temp")
        }} >Pfandkeykarte mit Zugang</button>
    </>)
}