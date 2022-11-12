import React, { ImgHTMLAttributes } from 'react';
import { LogoFull } from '../Components/images/LogoFull';
import { TreeView } from '../Components/tree-view/TreeView';
export interface MainProps { }

export const Main: React.FC<MainProps> = (props) => {

    return (<main>
        <LogoFull width={300} /><br />
        <a href="/login">Zum Login</a><br />
        <a href="/dashboard">Zum Dashboard</a>
        <p>
            Universitäten sind heutzutage immer auf der Suche nach Wegen, um die Sicherheit zu verbessern. Ein Schlüsselverwaltungssystem ist eine gute Möglichkeit, um den Überblick über die vielen Schlüssel zu behalten, die im Umlauf sind. Mit einem Schlüsselverwaltungssystem können Sie den Zugriff auf bestimmte Bereiche einschränken und so die Sicherheit erhöhen.
        </p>

        <p>
            KeyPa ist ein Schlüsselverwaltungssystem, das speziell für Universitäten entwickelt wurde. Es bietet eine zentrale Stelle, an der alle Schlüssel verwaltet werden können, und ermöglicht es den Benutzern, Schlüssel online zu bestellen und abzuholen. Dieses System bietet eine Reihe von Vorteilen, darunter eine verbesserte Sicherheit, eine einfachere Verwaltung und eine reduzierte Kosten.
        </p>
    </main>)
}