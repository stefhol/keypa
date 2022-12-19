import { Outlet } from "react-router-dom"

export const RequestBase: React.FC<{}> = (props) => {

    return (<>
        <main>

            <Outlet />
        </main>
    </>)
}