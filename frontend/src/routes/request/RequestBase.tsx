import { Outlet } from "react-router-dom"

export const RequestBase: React.FC<{}> = (props) => {

    return (<>
        <main className="container">

            <Outlet />
        </main>
    </>)
}