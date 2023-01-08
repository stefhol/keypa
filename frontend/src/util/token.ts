import * as jose from 'jose'
export const decodeToken = (params: URLSearchParams) => {
    return jose.decodeJwt(params.get("token") as string) as {
        is_admin: boolean,
        is_leader: boolean,
        is_worker: boolean,
        sub: string,
    }

}