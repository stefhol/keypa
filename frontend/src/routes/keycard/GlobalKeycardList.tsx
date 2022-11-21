export interface GlobalKeycardListProps { }
export const GlobalKeycardList: React.FC<GlobalKeycardListProps> = (props) => {

    return (<>
        <table>
            <thead>
                <tr>
                    <th colSpan={100}>
                        Nur ablaufende Keycards anzeigen <input type={"checkbox"} />
                    </th>
                </tr>
                <tr>
                    <th>

                    </th>
                    <th>
                        Nutzer
                    </th>
                    <th>
                        Rückgabe fällig
                    </th>
                    <th>
                        Grund
                    </th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>
                        <button>Erinnerung schicken</button>
                    </td>
                    <td>
                        Herbert Traum
                    </td>
                    <td>
                        in 2 Tagen
                    </td>
                    <td>
                        Ablauf
                    </td>
                </tr>
                <tr>
                    <td>
                        <button>Erinnerung schicken</button>
                    </td>
                    <td>
                        Ulrike Meier
                    </td>
                    <td>
                        in 5 Tagen
                    </td>
                    <td>
                        Kündigung
                    </td>
                </tr>
            </tbody>
        </table>
    </>)
}