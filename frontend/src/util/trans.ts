import i18next from "i18next"

export const transBool = (input?: boolean) => {
    if (input) {
        return i18next.t("true") as string
    } else {
        return i18next.t("false") as string
    }
}   