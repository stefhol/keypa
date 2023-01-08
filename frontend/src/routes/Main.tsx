import React, { ImgHTMLAttributes } from 'react';
import { LogoFull } from '../Components/images/LogoFull';
import { TreeView } from '../Components/tree-view/TreeView';
export interface MainProps { }

export const Main: React.FC<MainProps> = (props) => {

    return (<main>
        <LogoFull width={300} /><br />
        <p>
            Universitäten sind heutzutage immer auf der Suche nach Wegen, um die Sicherheit zu verbessern. Ein Schluesselverwaltungssystem ist eine gute Möglichkeit, um den Überblick ueber die vielen Schluessel zu behalten, die im Umlauf sind. Mit einem Schluesselverwaltungssystem können Sie den Zugriff auf bestimmte Bereiche einschränken und so die Sicherheit erhöhen.
        </p>

        <p>
            KeyPa ist ein Schluesselverwaltungssystem, das speziell fuer Universitäten entwickelt wurde. Es bietet eine zentrale Stelle, an der alle Schluessel verwaltet werden können, und ermöglicht es den Benutzern, Schluessel online zu bestellen und abzuholen. Dieses System bietet eine Reihe von Vorteilen, darunter eine verbesserte Sicherheit, eine einfachere Verwaltung und eine reduzierte Kosten.
        </p>
    </main>)
}