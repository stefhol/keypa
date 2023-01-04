export interface HomeProps { }
export const Home: React.FC<HomeProps> = (props) => {
    return (<>
        <main>
            <h1>Home</h1>
            <a href="user">
                Mein Bereich
            </a><br />
            <h3>Verwaltung</h3>
            <a href="request">
                Antragsformulare
            </a><br />
            <a href="/keycard">
                Keycard Übersicht
            </a><br />
            <a href="users">
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
