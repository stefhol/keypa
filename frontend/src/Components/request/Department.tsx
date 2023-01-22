import i18next from "i18next";
import React, { CSSProperties } from "react";
import { Department } from "../../util/intefaces/Departments";
interface DepartmentGroupProp {
    department?: Department[]
    onChange: (newVal: string) => void
    nmbr: number
}

export const DepartmentGroup: React.FC<DepartmentGroupProp> = (props) => {
    const [selected, setSelected] = React.useState("");
    const selected_option = React.useMemo(() => props.department?.find(val => val.department_id === selected), [selected, props.department?.length])
    const [style, setStyle] = React.useState({} as CSSProperties);
    return (<>

        <div className="container" key={props.nmbr} style={style}>


            <select value={selected} onChange={e => {
                setSelected(e.target.value)
                props.onChange(e.target.value)
            }} name={`department-select-${props.nmbr}`}>
                <option value={""}></option>
                {props.department?.map((val, idx) => <option key={idx} value={val.department_id}>
                    {val.name} {val.is_sensitive ? i18next.t("is_sensitive") : ""}
                </option>)}
            </select>

            <button onClick={(e) => {
                e.preventDefault()
                props.onChange(undefined as unknown as string)
                setSelected("")
                setStyle({ display: "none" })
            }}>X</button>
            {selected_option &&
                <div>
                    {i18next.t("includes")}: {selected_option.buildings.map((val, idx) => <div key={idx}>
                        <b>{val.name}:</b>{` ${val.rooms.map((val) => val.name).join(", ")} `}
                    </div>)}</div>
            }

        </div>

    </>)
}

