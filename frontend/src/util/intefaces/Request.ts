

// Generated by https://quicktype.io

export interface Request {
    request_id: string;
    requester_id: string;
    requester: User;
    created_at: string;
    changed_at: string;
    description: string;
    accept: boolean;
    reject: boolean;
    pending: boolean;
    comments?: Comment[];
    active_until: string;
    is_proposal: boolean;
    active: boolean;
    keycard_id: string;
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
