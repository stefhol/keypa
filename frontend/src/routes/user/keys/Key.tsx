import { useQuery } from "@tanstack/react-query"
import { json } from "react-router-dom"
import { TreeData, TreeView } from "../../../Components/tree-view/TreeView"
import { Building } from "../../../util/intefaces/Buildings"
import { Keys } from "../../../util/intefaces/Keys"
import { Rest } from "../../../util/Rest"
import { prepareData } from "../request/Request"

export interface KeyProps { }
export const Key: React.FC<KeyProps> = (props) => {
    const { data: keys } = useQuery(["self", "keys"], Rest.getSelfKeys)
    const { data: building } = useQuery(["buildings"], Rest.getBuildings)
    return (<>
        {
            (keys && building) &&
            <BuildingFC value={prepareData(building)} />
        }
    </>)
}
const filterNotOwned = (keys: Keys[] = [], building: Building[] = []): Building[] => {
    return building.filter(val => val.rooms.filter(val => val.doors.filter(door => keys.find(key => key.door_id === door.door_id)).length > 0).length > 0);
}
export interface BuildingFCProps {
    value: TreeData[]
}
export const BuildingFC: React.FC<BuildingFCProps> = (props) => {

    return (<>
        {
            props.value.map((val, idx) => (<>{
                val.children && val.children.length > 0 ?
                    <details key={idx}>
                        <summary >

                            {val.name}
                        </summary>
                        <BuildingFC value={val.children} />
                    </details>
                    : <div key={idx}>
                        {val.name}
                    </div>
            }</>))
        }
    </>)
}
