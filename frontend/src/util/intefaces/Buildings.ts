export interface Building {
    building_id: string;
    name: string;
    rooms: Room[];
}

export interface Room {
    building_id: string;
    floor: number;
    is_sensitive: boolean;
    name: string;
    room_id: string;
    doors: Door[];
}

export interface Door {
    door_id: string;
    name: string;
    room_id: string;
}
export interface BuildingWithOwner {
    building_id: string;
    name: string;
    rooms: RoomWithOwner[];
}

export interface RoomWithOwner {
    building_id: string;
    floor: number;
    is_sensitive: boolean;
    name: string;
    room_id: string;
    doors: DoorWithOwner[];
}

export interface DoorWithOwner {
    door_id: string;
    owner: boolean,
    name: string;
    room_id: string;
}
