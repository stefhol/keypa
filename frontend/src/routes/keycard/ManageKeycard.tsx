import React from "react";

export interface ManageKeycardProps { }
export const ManageKeycard: React.FC<ManageKeycardProps> = (props) => {

    return (<>
        <h1>Aenderungen Keycard</h1>
        <ManageKeycardForm />
    </>)
}
export interface ManageKeycardFormProps { }
export const ManageKeycardForm: React.FC<ManageKeycardFormProps> = (props) => {
    const [isRueckgabeButtonClicked, setIsRueckgabeButtonClicked] = React.useState(false);
    return (<>
        <form>
            <button onClick={(e) => { e.preventDefault() }}>
                Aktiv schalten
            </button>
            <button onClick={(e) => { e.preventDefault() }}>
                Als verloren melden
            </button>
            <button onClick={(e) => { e.preventDefault() }}>
                Sperren
            </button>
            <button onClick={(e) => { e.preventDefault(); setIsRueckgabeButtonClicked(true) }}>
                Rueckgabe veranlassen
            </button>
            {isRueckgabeButtonClicked && <p>
                Rueckgabe veranlasst. Bitte senden sie die Karte per Post oder werfen sie diese in den Briefkasten am Gebaude der Verwaltung
            </p>}
        </form>
    </>)
}