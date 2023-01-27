import { createContext } from "react";
export interface IUserContext {
    set?: (obj: IUserContext) => void,
    is_admin?: boolean,
    is_worker?: boolean,
    is_leader?: boolean,
    loggedIn?: boolean
}
export default createContext({} as IUserContext)