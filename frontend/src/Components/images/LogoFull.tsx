import React from "react";
import KeyPaLogoFull from '../../assets/KeyPaFull.png'

export interface LogoFullProps { width: number }
export const LogoFull: React.FC<LogoFullProps> = (props) => {
    const logo = React.useRef(undefined as unknown as HTMLImageElement);
    React.useEffect(() => {
        logo.current.src = KeyPaLogoFull
        return () => {

        }
    }, []);
    return (<>
        <img id='logo-full' ref={logo} width={props.width} />
    </>)
}