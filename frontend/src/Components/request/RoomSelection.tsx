import i18next from "i18next";
import React from "react";
import { Building, BuildingWithOwner } from "../../util/intefaces/Buildings";
import { createTreeDatafromBuiding, createTreeDatafromBuidingWithOwner, treeDataToStringArr, TreeView } from "../tree-view/TreeView";

interface CreateProps {
    buildings: Building[]
    nmbr: number
    onChange: (newVal: string[]) => void
}
export const RoomSelection: React.FC<CreateProps> = (props) => {
    const [selected, setSelected] = React.useState("");
    const selectedBuilding = React.useMemo(() => props.buildings.find(val => val.building_id == selected), [selected])
    const selectedRooms = React.useRef({ getCurrentSelection: undefined as any });
    const treeData = React.useMemo(() => selectedBuilding ? createTreeDatafromBuiding([selectedBuilding], []) : [], [selected])
    return (<>
        <div className="my-container">
            <label>
                {i18next.t("select_building")}
                <select value={selected} onChange={e => setSelected(e.target.value)}>
                    <option key={-1} value={""}></option>
                    {props.buildings.map((val, idx) =>
                        <option key={idx} value={val.building_id}>{val.name}</option>
                    )}

                </select>
            </label>
            {selectedBuilding && <label>
                {i18next.t("select_rooms")}
                <TreeView filter={false} selectionRef={selectedRooms} data={treeData}
                    onChange={tree => props.onChange(treeDataToStringArr(tree))} />
            </label>}
        </div>
    </>)
}
