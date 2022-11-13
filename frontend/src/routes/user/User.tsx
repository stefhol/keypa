import { useNavigate } from "react-router-dom"
import { AuthorizedBuildings } from "./keys/Key"

export interface UserProps { }
export const User: React.FC<UserProps> = (props) => {
    const navigate = useNavigate()

    return (<>
        <h1>
            Nutzerbereich
        </h1>
        <button onClick={(e) => {
            e.preventDefault()
            navigate("request")
        }}>Neue Zugaenge anfragen</button>
        <button onClick={(e) => {
            e.preventDefault()
            navigate("account")
        }}>Account</button>
        <AuthorizedBuildings />


        <h2>Meine Schluesselkarten</h2>
        <table>
            <thead>
                <tr><th>Kartenummer </th><th>Aktiv</th><th>Gueltig bis.</th></tr>
            </thead>
            <tbody>
                <tr>
                    <td>1.</td>
                    <td>x</td>
                    <td>
                        02.11.2023
                    </td>
                </tr>
                <tr>
                    <td>2.</td>
                    <td></td>
                    <td>
                        02.11.2020
                    </td>
                </tr>
            </tbody>
        </table>
    </>)
}