import React, { MutableRefObject, Ref } from 'react';
import '../../css/tree.css';
import { Building, Room } from '../../util/intefaces/Buildings';

export type SelectionRef =
    MutableRefObject<{
        getCurrentSelection: () => TreeData[]
    }>

export interface TreeViewProps {
    displayFilter?: boolean
    onChange?: (newVal: TreeData[]) => void
    filter?: boolean
    isInitialValueTrue?: boolean
    selectionRef?: SelectionRef
    data: TreeData[]
    readonly?: boolean
    expanded?: boolean
}
interface BaseTree {
    className?: string
    bubbleDown?: (bool: boolean) => void
    bubbleUp?: (caller: string, bool: boolean) => void
    parentActive?: [string, boolean]
    keys: string[]
    isInitialValueTrue?: boolean
    readonly?: boolean
    expanded?: boolean

}
interface ITreeContext {
    selection?: TreeData[]
    filter?: boolean
    onChange: () => void
    reset: number
}
const TreeContext = React.createContext({} as ITreeContext)

export const TreeView: React.FC<TreeViewProps> = (props) => {
    const [loading, setLoading] = React.useState(true);
    const [filter, setFilter] = React.useState(!!props.filter);
    const [reset, setReset] = React.useState(0);
    const selection = React.useRef([] as TreeData[]);
    const onChange = () => {
        if (props.onChange) props.onChange(selection.current)
    }
    React.useEffect(() => {
        selection.current = props.data
        setReset(prev => prev + 1)
        setLoading(false)
    }, [props.data]);
    const selectionRef = React.useRef({});
    if (props.selectionRef) {
        selectionRef.current = props.selectionRef.current
    }
    React.useImperativeHandle(
        selectionRef,
        () => ({ getCurrentSelection: () => { return selection.current; } }),
        [selection],
    )
    return (<div className="tree">
        <TreeContext.Provider value={{ selection: selection.current, filter, onChange, reset }} >
            {
                !loading &&
                <>
                    {
                        props.displayFilter
                        &&
                        <label>
                            Filter
                            <input
                                checked={filter}
                                onChange={() => setFilter(prev => !prev)}
                                type={"checkbox"} />
                        </label>
                    }
                    <TreeComponentWrapper
                        readonly={props.readonly}
                        expanded={props.expanded}
                        value={props.data} keys={[]} isInitialValueTrue={props.isInitialValueTrue} />
                </>
            }
        </TreeContext.Provider>
    </div>)
}
interface IWrapperProps extends BaseTree {
    parent?: TreeData,
    value?: TreeData[]
}
export const TreeComponentWrapper: React.FC<IWrapperProps> = (props) => {
    return <ul className="tree children">
        {
            props?.value &&
            props.value.map((val, idx) =>
                <TreeComponent key={idx} {...props} value={val} />)
        }
    </ul>
}
interface ICompProps extends BaseTree {
    parent?: TreeData,
    value: TreeData
}
export const TreeComponent: React.FC<ICompProps> = (props) => {
    const { selection, filter, onChange, reset } = React.useContext(TreeContext);
    const [checked, setChecked] = React.useState(!!props.value.value);
    const [hasATrueValueInTree, setHasATrueValueInTree] = React.useState(false);
    const [lastAction, setLastAction] = React.useState(props.value.name);
    const [showChildren, setShowChildren] = React.useState(!!props.expanded);
    React.useEffect(() => {
        let currentSelection = getCurrentTreeData(selection, props.keys, props.value.name)

        currentSelection.value = checked

        if (typeof filter === "boolean") {

            setHasATrueValueInTree(isTrueInChildren(currentSelection))
        }
        onChange()

    }, [checked, filter]);
    React.useEffect(() => {
        setChecked(false)
    }, [reset])
    React.useEffect(() => {
        if (props.parentActive?.[1] !== undefined && props.parentActive?.[0] === props.parent?.name) {
            setChecked(props.parentActive?.[1])
            setLastAction(props.value.name)
        }
    }, [props.parentActive?.[0], props.parentActive?.[1]]);
    React.useEffect(() => {
        setChecked(!!props.value.value)
    }, []);
    return (<li className={`tree node ${props.className || ""} ${(!hasATrueValueInTree && filter) ? "hidden" : ""}`}>
        <label>
            {(props.value.children && props.value.children?.length > 0) && <button className='tree' onClick={(e) => {
                e.preventDefault()
                setShowChildren(prev => !prev)
            }}>{showChildren ? "⬇️" : "⬆️"}</button>}
            <input className='tree'
                type={"checkbox"}
                checked={checked}
                readOnly={props.readonly}
                onChange={(e) => {
                    if (!props.readonly) {
                        props.bubbleUp && props.bubbleUp(props.value.name, !checked)
                        setChecked(!checked)
                        setLastAction(props.value.name)
                    }
                }}
            />
            {props.value.name}
        </label>
        {
            ((props.value.children && props.value.children?.length > 0)) &&
            <TreeComponentWrapper
                {...props}
                className={`${showChildren ? "" : "hidden"}`}
                value={props.value.children}
                parent={props.value}
                bubbleUp={(caller, bool) => {

                    if (props.value.children?.find(val => val.name === caller) && !bool) {
                        setChecked(bool)
                        props.bubbleUp && props.bubbleUp(props.value.name, bool)
                        setLastAction(caller)
                        setShowChildren(true)
                    }
                }}
                parentActive={[lastAction, checked]}
                keys={[...props.keys, props.value.name]}
            />
        }
    </li>)
}
export interface TreeData {
    name: string,
    value?: boolean,
    children?: TreeData[]
    id?: string
}


const getCurrentTreeData = (obj: TreeData[] = [], keys: string[], currentKey: string): TreeData => {
    let curr = obj
    for (let index = 0; index < keys.length; index++) {
        const key = keys[index];
        curr = curr.find(val => val.name === key)?.children || []
    }
    return curr.find(val => val.name === currentKey) || {} as TreeData
}

const isTrueInChildren = (input: TreeData): boolean => {
    if (input.value) {
        return true
    }
    let treeData = input.children || []
    for (let index = 0; index < treeData.length; index++) {
        const el = treeData[index];
        let found = isTrueInChildren(el)
        if (found) {
            return true
        }
    }
    return false
}
export const prepareData = (data: Building[], filter?: boolean) => {

    return data.map(val => ({
        name: `Gebäude ${val.name}`,
        children: prepareStockwerke(val.rooms)
    }))
}
const prepareStockwerke = (data: Room[]): TreeData[] => {
    let ret = [] as TreeData[]


    let floors = new Set(data.map(val => val.floor) as number[])
    floors.forEach(floor => {
        ret.push({
            name: `Stockwerk: ${floor}`,
            children: data.filter(val => val.floor == floor)
                .map((val, idx) => ({
                    name: `Raum: ${val.name} ${val.is_sensitive ? "Sensitiv" : ""}`,
                    id: val.room_id,
                    value: !!val.doors.find(val => val?.owner === true),
                    children: []
                }))
        })
    })
    return ret
}

export const treeDataToStringArr = (tree: TreeData[]): string[] => {
    let rooms = []
    for (let index = 0; index < tree.length; index++) {
        const building = tree[index];
        if (building.children) {
            for (let index = 0; index < building.children.length; index++) {
                const floor = building.children[index];
                if (floor.children) {
                    for (let index = 0; index < floor.children.length; index++) {
                        const room = floor.children[index];
                        if (room.value)
                            rooms.push(room.id as string)
                    }
                }
            }
        }
    }
    return rooms;
}