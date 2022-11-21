import React from "react";
import { LoadingContext } from "../../hooks/useLoading";

export interface LoadingProviderProps { children: any }
export const LoadingProvider: React.FC<LoadingProviderProps> = (props) => {
    const [isLoading, setIsLoading] = React.useState(false);
    const body = React.useRef(null as unknown as HTMLBodyElement);
    React.useEffect(() => {
        body.current = document.getElementsByTagName("body")?.[0] || null;
    }, [])
    React.useEffect(() => {

       if(body.current){
           if (isLoading) {
               body.current.style.cursor = "progress"
           } else {
               body.current.style.cursor = "default"
           }
       }
    }, [isLoading]);
    return (<>
        <span className={`${isLoading ? "loading" : ""}`}>
        </span>
        <LoadingContext.Provider value={{ isLoading, startLoading: () => setIsLoading(true), stopLoading: () => setIsLoading(false), setLoading: (bool) => setIsLoading(bool) }}>
            {props.children}
        </LoadingContext.Provider>
    </>
    )
}