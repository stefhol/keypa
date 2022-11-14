// Generated by https://quicktype.io

export interface Building {
    building_id: string;
    name: string;
    rooms: Room[];
}

export interface Room {
    room_id: string;
    name: string;
    floor: number;
    is_sensitive: boolean;
    building_id: string;
    doors: Door[];
}

export interface Door {
    door_id: string;
    name: string;
    owner: boolean;
    room_id: string;
}