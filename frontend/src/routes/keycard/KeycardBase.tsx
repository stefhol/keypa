import { Outlet } from "react-router-dom"

export interface KeycardBaseProps { }
export const KeycardBase: React.FC<KeycardBaseProps> = (props) => {

    return (<><main className="container">
        <Outlet />
    </main></>)
}