import { useQuery } from "@tanstack/react-query"
import { TreeData } from "../../../Components/tree-view/TreeView"
import { Rest } from "../../../util/Rest"
import { prepareData } from "../request/Request"

export interface KeyProps { }
export const AuthorizedBuildings: React.FC<KeyProps> = (props) => {
    const { data: keys } = useQuery(["self", "doors"], Rest.getSelfDoors)
    return (<>
        <h2>Meine Zugänge</h2>
        {
            (keys && keys) &&
            <BuildingFC value={prepareData(keys)} />
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
