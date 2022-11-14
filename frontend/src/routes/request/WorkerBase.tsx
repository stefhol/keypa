import { Outlet } from "react-router-dom"

export interface WorkerBaseProps { }
export const RequestBase: React.FC<WorkerBaseProps> = (props) => {

    return (<>
        <main>

        <Outlet />
        </main>
    </>)
}