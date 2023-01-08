

export interface KeycardUsageHistory {
    keycard_history_id: number;
    keycard_id: string;
    door_id: string;
    used_at: string;
    success: boolean;
    room_id: string;
    room_name: string;
    room_floor: number;
    is_sensitive?: boolean;
    building_id: string;
    building_name: string;
    username?: string;
    role_id?: number;
    user_id: string;
}
