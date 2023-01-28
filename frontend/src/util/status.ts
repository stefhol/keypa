export const convertToStatusString = (val?: {
    pending?: boolean,
    reject?: boolean,
    accept?: boolean
}) => {
    if (val?.accept) {
        return 'accept'
    }
    if (val?.pending) {
        return 'pending'
    }
    if (val?.reject) {
        return 'reject'
    }
    return ""
}