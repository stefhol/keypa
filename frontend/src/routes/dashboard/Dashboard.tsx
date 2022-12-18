export interface DashboardProps { }
export const Dashboard: React.FC<DashboardProps> = (props) => {
    return (<>
        <main>
            <h1>Dashboard</h1>
            <a href="user">
                Mein Bereich
            </a><br />
            <h3>Verwaltung</h3>
            <a href="request">
                Austehende Antragsformulare
            </a><br />
            <a href="propositons">
                Anfragen
            </a><br />
            <a href="/keycard">
                Keycard Übersicht
            </a><br />
            <a href="leader">
                Nutzer Übersicht
            </a><br />
            <h3>Tools</h3>
            <a href="stats">
                Statistiken
            </a><br />
            <a href="logs">
                Logdateien
            </a><br />
        </main>
    </>)
}
