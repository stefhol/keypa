export const CreatePropostion: React.FC<{}> = (props) => {

    return (<>
        <form>
            <h1>Neue Anfrage</h1>
            <form>
                <label> Beschreibung:
                    <textarea />
                </label>
                <label> Bis wann:
                    <input type={"date"} />
                </label>
                <div className="container">
                    <h2>Gruppen</h2>
                    <div className="container">
                        <label> 1.

                            <select>
                                <option>Lehrstuhl IT-Sec</option>
                                <option>Lehrstuhl Wirtschaft</option>
                                <option>Lehrstuhl Operational Science</option>
                            </select>
                        </label>
                        <p>Info: Beinhaltet Gebäude ITZ: Raum 204, Raum 203, Gebäude Fim: Raum 200</p>
                    </div>
                    <button>Anderen Gruppe hinzufügen</button>
                </div>
                <div className="container">
                    <h2>Zusätzliche Räume</h2>
                    <div className="container">
                        <label>
                            Gebäude auswählen
                            <select>
                                <option>Gebäude ITZ</option>
                                <option>Gebäude Fim</option>
                            </select>
                        </label>
                        <label>
                            Räume auswählen
                            <textarea />
                        </label>
                    </div>
                    <button>Raum in anderen Gebäude hinzufügen</button>
                </div>
                <button>Speichern</button>
            </form>

        </form>
    </>)
}