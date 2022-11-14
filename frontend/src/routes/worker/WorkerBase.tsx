import { Outlet } from "react-router-dom"

export interface WorkerBaseProps { }
export const WorkerBase: React.FC<WorkerBaseProps> = (props) => {

    return (<>

        <Outlet />
    </>)
}