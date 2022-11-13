import React from "react";
import KeyPaLogoFull from '../../assets/KeyPaShort.png'

export interface LogoFullProps { width: number, onClick: () => void }
export const LogoSmall: React.FC<LogoFullProps> = (props) => {
    const logo = React.useRef(undefined as unknown as HTMLImageElement);
    React.useEffect(() => {
        logo.current.src = KeyPaLogoFull
        return () => {

        }
    }, []);
    return (<>
        <img id='logo-short' ref={logo} width={props.width} onClick={props.onClick} style={{ cursor: "pointer" }} />
    </>)
}