import i18next from 'i18next';
import React, { ImgHTMLAttributes } from 'react';
import { LogoFull } from '../Components/images/LogoFull';
import { TreeView } from '../Components/tree-view/TreeView';
export interface MainProps { }

export const Main: React.FC<MainProps> = (props) => {

    return (<main className='container'>
        <LogoFull width={300} /><br />
        <p>
            {i18next.t("intro")}
        </p>
    </main>)
}