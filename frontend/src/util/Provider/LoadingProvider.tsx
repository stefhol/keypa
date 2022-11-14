import React from "react";
import { LoadingContext } from "../../hooks/useLoading";

export interface LoadingProviderProps { children: any }
export const LoadingProvider: React.FC<LoadingProviderProps> = (props) => {
    const [isLoading, setIsLoading] = React.useState(false);
    return (<>
        <span className={`${isLoading ? "loading" : ""}`}>
        </span>
        <LoadingContext.Provider value={{ isLoading, startLoading: () => setIsLoading(true), stopLoading: () => setIsLoading(false), setLoading: (bool) => setIsLoading(bool) }}>
            {props.children}
        </LoadingContext.Provider>
    </>
    )
}