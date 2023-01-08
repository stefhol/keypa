
// Generated by https://quicktype.io

export interface Request {
    request_id: string;
    requester_id: string;
    requester: User;
    created_at: string;
    changed_at: string;
    description: null;
    accept: boolean;
    reject: boolean;
    pending: boolean;
    active_until: string;
    active: boolean;
    keycard_id: string;
    request_type: string;
    departments?: string[];
    doors: string[];
    additional_rooms?: string,

}

export interface Comment {
    comment_id: string;
    request_id: string;
    user_id: string;
    user: User;
    comment: string;
    written_at: string;
}

export interface User {
    user_id: string;
    name: string;
    role_id: number;
    email: string;
    tel: null;
    address: null;
    picture_url: null;
}
