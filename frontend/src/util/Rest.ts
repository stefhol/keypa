import { LoginRequest } from "../routes/login/Login"
import { Building, BuildingWithOwner } from "./intefaces/Buildings"
import { Department } from "./intefaces/Departments"
import { Keycard } from "./intefaces/Keycard"
import { KeycardUsageHistory } from "./intefaces/KeycardUsageHistory"
import { Keys } from "./intefaces/Keys"
import { Log } from "./intefaces/Log"
import { User, Request, Comment } from "./intefaces/Request"

//@ts-ignore
const url = (process.env.NODE_ENV === "development") ? `http://${window.location.hostname}:8080` : window.location.origin
const api = "/api/v1/"
const translations = "/translations/"
export class Rest {
    static getRessourceBundle = async (locale: string) => {
        let response = await fetch(`${url}${translations}${locale}.json`, {
            method: "GET"
        })
        let json = await response.json()
        return json as Object
    }
    static getLogs = async () => {
        return await this.quickFetchJson<Log[]>("logs", "GET")

    }

    static getSelfDepartments = async () => {
        return await this.quickFetchJson<Department[]>("self/department", "GET")

    }
    static getUserDepartments = async (userId: string) => {
        return await this.quickFetchJson<Department[]>(`users/${userId}/department`, "GET")


    }
    static getSelfDepartmentsWithKeycard = async (keycard_id: string) => {
        return await this.quickFetchJson<Department[]>(`self/department/${keycard_id}`, "GET")

    }
    static getUserDepartmentsWithKeycards = async (userId: string, keycard_id: string) => {
        return await this.quickFetchJson<Department[]>(`users/${userId}/department/${keycard_id}`, "GET")


    }
    static getDepartments = async () => {
        return await this.quickFetchJson<Department[]>("department", "GET")

    }
    static sendLogin = async (payload: LoginRequest) => {
        return await this.quickFetch("login", "POST", payload)
    }
    static sendLogout = async () => {
        return await this.quickFetch("logout", "GET")
    }
    static getSelfDoors = async () => {
        return await this.quickFetchJson<BuildingWithOwner[]>("self/doors", "GET")
    }
    static getSelfDoorsKeycard = async (keycardId: string) => {
        return await this.quickFetchJson<BuildingWithOwner[]>(`self/doors/${keycardId}`, "GET")
    }

    static getSelfKeycard = async () => {
        return await this.quickFetchJson<Keycard[]>("self/keycard", "GET")
    }
    static getKeycard = async () => {
        return await this.quickFetchJson<Keycard[]>("keycard", "GET")
    }
    static getKeycardUsageHistory = async () => {
        return await this.quickFetchJson<KeycardUsageHistory[]>("keycard-usage-history", "GET")
    }
    static getKeycardsFromUser = async (userId: string) => {
        return await this.quickFetchJson<Keycard[]>(`user/${userId}/keycard`, "GET")
    }
    static getSingleKeycard = async (keycardId: string) => {
        return await this.quickFetchJson<Keycard>(`keycard/${keycardId}`, "GET")

    }
    static getDoorsWithRequestId = async (requestId: string) => {
        return await this.quickFetchJson<BuildingWithOwner[]>(`request/${requestId}/doors`, "GET")
    }
    static getSelf = async () => {
        return await this.quickFetchJson<User>("self", "GET")
    }
    static getUserByUserId = async (userId: string) => {
        return await this.quickFetchJson<User>(`users/${userId}`, "GET")
    }
    static getUsers = async () => {
        return await this.quickFetchJson<User[]>("users", "GET")
    }
    static getRequests = async (queryParams?: string) => {
        return await this.quickFetchJson<Request[]>("request", "GET", undefined, queryParams)
    }
    static getRequestsFromUser = async (userId: string, queryParams?: string) => {
        return await this.quickFetchJson<Request[]>(`user/${userId}/request`, "GET", undefined, queryParams)
    }
    static getSelfRequests = async (queryParams?: string) => {
        return await this.quickFetchJson<Request[]>("self/request", "GET", undefined, queryParams)
    }
    static getSingleUser = async (userId: string) => {
        return await this.quickFetchJson<User>(`users/${userId}`, "GET")
    }
    static getSingleWoker = async (userId: string) => {
        return await this.quickFetchJson<User>(`users/${userId}`, "GET")
    }
    static getComment = async (requestId: string) => {
        return await this.quickFetchJson<Comment[]>(`request/${requestId}/comment`, "GET")
    }
    static createComment = async (requestId: string, comment: { comment: string }) => {
        return await this.quickAdd(`request/${requestId}/comment`, "PUT", comment)
    }
    static getBuildings = async () => {
        return await this.quickFetchJson<Building[]>("buildings", "GET")
    }
    static getDoorsByUser = async (userId: string) => {
        return await this.quickFetchJson<BuildingWithOwner[]>(`users/${userId}/doors`, "GET")
    }
    static getDoorsByUserAndKeycard = async (userId: string, keycardId: string) => {
        return await this.quickFetchJson<BuildingWithOwner[]>(`users/${userId}/doors/${keycardId}`, "GET")
    }

    static getSingleRequest = async (requestId: string, queryParams?: string) => {
        return await this.quickFetchJson<Request>(`request/${requestId}`, "GET", undefined, queryParams)
    }
    static getSelfRequestsWithRequestId = async (request_id: string) => {
        return await this.quickFetchJson<Request>(`self/request?request_id=${request_id}`, "GET")
    }

    static quickFetchJson = async<T>(address: string, method: string, data?: any, queryParams?: string) => {
        if (queryParams) {
            queryParams = `?${queryParams}`
        }
        else {
            queryParams = ""
        }
        let response = await fetch(`${url}${api}${address}${queryParams}`, {
            method, // *GET, POST, PUT, DELETE, etc.
            headers: data === undefined ? {} : {
                'Content-Type': 'application/json',
            },
            credentials: "include",

            body: data === undefined ? undefined : JSON.stringify(data)// body data type must match "Content-Type" header
        })
        let json = await response.json()
        return json as T
    }
    static quickFetch = async (address: string, method: string, data?: any) => {
        let response = await fetch(`${url}${api}${address}`, {
            method, // *GET, POST, PUT, DELETE, etc.
            headers: data === undefined ? {} : {
                'Content-Type': 'application/json',
            },
            credentials: "include",

            body: data === undefined ? undefined : JSON.stringify(data)// body data type must match "Content-Type" header
        })
        return response;
    }
    static quickAdd = async (address: string, method: string, data: any) => {
        let response = await fetch(`${url}${api}${address}`, {
            method, // *GET, POST, PUT, DELETE, etc.
            headers: data === undefined ? {} : {
                'Content-Type': 'application/json',
            },
            credentials: "include",

            body: data === undefined ? undefined : JSON.stringify(data)// body data type must match "Content-Type" header
        })
        return response;
    }
}