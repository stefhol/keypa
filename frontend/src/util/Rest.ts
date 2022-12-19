import { LoginRequest } from "../routes/login/Login"
import { Building } from "./intefaces/Buildings"
import { Department } from "./intefaces/Departments"
import { Keycard } from "./intefaces/Keycard"
import { Keys } from "./intefaces/Keys"
import { User, Request } from "./intefaces/Request"

//@ts-ignore
const url = (process.env.NODE_ENV === "development") ? "http://localhost:8080" : window.location.origin
const api = "/api/v1/"
export class Rest {
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
        return await this.quickFetchJson<Building[]>("self/doors", "GET")
    }

    static getSelfKeycard = async () => {
        return await this.quickFetchJson<Keycard[]>("self/keycard", "GET")
    }
    static getKeycardsFromUser = async (userId: string) => {
        return await this.quickFetchJson<Keycard[]>(`user/${userId}/keycard`, "GET")
    }
    static getKeycard = async () => {
        return await this.quickFetchJson<Keycard>("keycard", "GET")
    }
    static getDoorsWithRequestId = async (requestId: string) => {
        return await this.quickFetchJson<Building[]>(`request/${requestId}/doors`, "GET")
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
    static getPendingRequests = async () => {
        return await this.quickFetchJson<Request[]>("request", "GET")
    }
    static getRequestsFromUser = async (userId: string) => {
        return await this.quickFetchJson<Request[]>(`user/${userId}/request`, "GET")
    }
    static getSingleUser = async (userId: string) => {
        return await this.quickFetchJson<User>(`users/${userId}`, "GET")
    }
    static getSingleWoker = async (userId: string) => {
        return await this.quickFetchJson<User>(`users/${userId}`, "GET")
    }
    static getBuildings = async () => {
        return await this.quickFetchJson<Building[]>("buildings", "GET")
    }
    static getDoorsByUser = async (userId: string) => {
        return await this.quickFetchJson<Building[]>(`users/${userId}/doors`, "GET")
    }
    static getSelfRequests = async () => {
        return await this.quickFetchJson<Request[]>("self/request", "GET")
    }
    static getSingleRequest = async (requestId: string) => {
        return await this.quickFetchJson<Request>(`request/${requestId}`, "GET")
    }
    static getSelfRequestsWithRequestId = async (request_id: string) => {
        return await this.quickFetchJson<Request>(`self/request?request_id=${request_id}`, "GET")
    }

    static quickFetchJson = async<T>(address: string, method: string, data?: any) => {
        let response = await fetch(`${url}${api}${address}`, {
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