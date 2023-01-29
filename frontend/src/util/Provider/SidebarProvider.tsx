import React from "react";

interface ISidebar {
    value: boolean,
    set: React.Dispatch<React.SetStateAction<boolean>>
}
export const SidebarContext = React.createContext<ISidebar>({} as ISidebar)
export interface SidebarProviderProps {
    children: any
}
export const SidebarProvider: React.FC<SidebarProviderProps> = (props) => {
    const [extended, setExtended] = React.useState(false);
    return (<>
        <SidebarContext.Provider value={{ value: extended, set: setExtended }}>
            {props.children}
        </SidebarContext.Provider>
    </>)
}
