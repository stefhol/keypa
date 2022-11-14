import React, { MutableRefObject, Ref } from 'react';
import '../../css/tree.css';
interface Selection {
    [key: string]: {
        value: boolean,
        children: Selection
    }
}
export type SelectionRef =
    MutableRefObject<{
        getCurrentSelection: () => TreeData[]
    }>

export interface TreeViewProps {
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
}
const TreeContext = React.createContext({} as ITreeContext)

export const TreeView: React.FC<TreeViewProps> = (props) => {
    const [loading, setLoading] = React.useState(true);
    const [filter, setFilter] = React.useState(!!props.filter);
    const selection = React.useRef([] as TreeData[]);
    React.useEffect(() => {
        selection.current = props.data

        setLoading(false)
    }, [props.data]);
    const temp = React.useRef({});
    let selectionRef = temp;
    if (props.selectionRef) {
        selectionRef = props.selectionRef
    }
    React.useImperativeHandle(
        selectionRef,
        () => ({ getCurrentSelection: () => { return selection.current; } }),
        [selection],
    )
    return (<div className="tree">
        <TreeContext.Provider value={{ selection: selection.current, filter }} >
            {
                !loading &&
                <>
                    <label>
                        Filter
                        <input
                            checked={filter}
                            onChange={() => setFilter(prev => !prev)}
                            type={"checkbox"} />
                    </label>
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
    const { selection, filter } = React.useContext(TreeContext);
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

    }, [checked, filter]);
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
}
const data: TreeData[] = [
    {
        name: "Building 1",
        children: [
            {
                name: "Room 1",
                children: undefined
            },
            {
                name: "Room 2",
                children: [
                    {
                        name: "Door 1"
                    }
                ]
            }
        ]
    },
    {
        name: "Building 2",
        children: undefined

    },
]


// const getCurrentSelection = (obj: Selection, keys: string[], currentKey: string) => {
//     let select = obj as Selection
//     keys.forEach(key => {
//         // @ts-ignore
//         select = select[key].children
//     })
//     return select[currentKey]
// }
const getCurrentTreeData = (obj: TreeData[] = [], keys: string[], currentKey: string): TreeData => {
    let curr = obj
    for (let index = 0; index < keys.length; index++) {
        const key = keys[index];
        curr = curr.find(val => val.name === key)?.children || []
    }
    return curr.find(val => val.name === currentKey) || {} as TreeData
}

// const createSelectionOutOfTreeNodes = (input: TreeData[], output: Selection) => {
//     for (let index = 0; index < input.length; index++) {
//         const element = input[index];
//         output[element.name] = {
//             value: !!element.value,
//             children: {}
//         }
//         if (element.children) {
//             createSelectionOutOfTreeNodes(element.children, output[element.name].children)
//         }
//     }
// }
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