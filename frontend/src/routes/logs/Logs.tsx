export interface LogsProps { }
export const Logs: React.FC<LogsProps> = (props) => {

    return (<>
        <h1>Ereignisse</h1>
        <div className="container">
            <div className="container">

                <p>
                    {new Date().toISOString()}
                </p>
                <p>Neuer Antrag von Nutzer A</p>
            </div>
            <div className="container">
                <p>
                    {new Date().toISOString()}
                </p>
                <p>Neuer Antrag von Nutzer B</p>
            </div>
            <div className="container">
                <p>
                    {new Date().toISOString()}
                </p>
                <p>Neuer Antrag von Nutzer B</p>
            </div>
            <div className="container">
                <p>
                    {new Date().toISOString()}
                </p>
                <p>Erinnerung an Nutzer A gesendet</p>
            </div>
            <p>...</p>
        </div>
        <button>Logdatei Herunterladen</button>
    </>)
}