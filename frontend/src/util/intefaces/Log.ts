import { User } from "./Request";

export interface Log {
    log_id: number;
    message: string;
    keycard_history_id: null;
    keycard_history: null;
    door_to_request_history_id: number | null;
    door_to_request_history: DoorToRequestHistory | null;
    changed_at: string;
    changed_by_id: string;
    changed_by: User;
}




export interface DoorToRequestHistory {
    door_to_request_history_id: number;
    door_id: string;
    request_id: string;
    action: string;
    changed_by: string;
}
