import { useQuery } from "@tanstack/react-query"
import { prepareData, TreeData, TreeView } from "../../../Components/tree-view/TreeView"
import { Building } from "../../../util/intefaces/Buildings"
import { Rest } from "../../../util/Rest"

export interface KeyProps {
    data: Building[]

}
export const AuthorizedBuildings: React.FC<KeyProps> = (props) => {
    return (<>
        <h2>Meine Zugänge</h2>
        {
            (props.data) &&
            <TreeView data={prepareData(props.data)} expanded readonly filter />
        }
    </>)
}

export interface BuildingFCProps {
    value: TreeData[]
}
export const BuildingFC: React.FC<BuildingFCProps> = (props) => {
    return (<>
        {
            props.value.map((val, idx) => (<>{
                val.children && val.children.length > 0 ?
                    <details key={idx} open>
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
export const getCountOfRooms = (data: Building[]) => {
    return data.reduce((acc, building) => (acc + building.rooms.reduce((acc, room) => acc + (room.doors.find(val => val.owner) ? 1 : 0), 0)), 0)

}