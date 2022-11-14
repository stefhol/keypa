import React from "react"

export const useLoading = (isLoading?: boolean) => {
    const data = React.useContext(LoadingContext);
    React.useEffect(() => {
        if (typeof isLoading === "boolean") {
            data.setLoading(isLoading)
        }
    }, [isLoading]);


    return data
}
interface ILoadingContext {
    isLoading: boolean,
    setLoading: (bool: boolean) => void
    startLoading: () => void,
    stopLoading: () => void
}
export const LoadingContext = React.createContext({} as ILoadingContext)

