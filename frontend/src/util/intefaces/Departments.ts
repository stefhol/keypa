import { Building } from "./Buildings";

export interface Department {
    department_id: string;
    name: string;
    description: string;
    buildings: Building[];
    is_sensitive?: boolean
}

