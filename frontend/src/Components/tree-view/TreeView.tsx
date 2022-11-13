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
        getCurrentSelection: () => Selection
    }>

export interface TreeViewProps {
    selectionRef?: SelectionRef
    data: TreeData[]
}
interface BaseTree {
    bubbleDown?: (bool: boolean) => void
    bubbleUp?: (caller: string, bool: boolean) => void
    parentActive?: [string, boolean]
    keys: string[]

}
interface ITreeContext {
    selection?: Selection
}
const TreeContext = React.createContext({} as ITreeContext)

export const TreeView: React.FC<TreeViewProps> = (props) => {
    const [loading, setLoading] = React.useState(true);

    const selection = React.useRef({} as Selection);
    React.useEffect(() => {
        createSelectionOutOfTreeNodes(props.data, selection.current)
        console.log(selection);

        setLoading(false)
    }, []);


    React.useImperativeHandle(
        props.selectionRef,
        () => ({ getCurrentSelection: () => { return selection.current; } }),
        [selection],
    )
    return (<div className="tree">
        <TreeContext.Provider value={{ selection: selection.current }} >
            {
                !loading &&
                <TreeComponentWrapper value={props.data} keys={[]} />
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
    const { selection } = React.useContext(TreeContext);
    const [checked, setChecked] = React.useState(false);
    const [lastAction, setLastAction] = React.useState(props.value.name);
    const [showChildren, setShowChildren] = React.useState(false);
    React.useEffect(() => {
        let currentSelection = getCurrentSelection(selection as Selection, props.keys, props.value.name)

        currentSelection.value = checked
    }, [checked]);
    React.useEffect(() => {
        if (props.parentActive?.[1] !== undefined && props.parentActive?.[0] === props.parent?.name) {
            setChecked(props.parentActive?.[1])
            setLastAction(props.value.name)
        }
    }, [props.parentActive?.[0], props.parentActive?.[1]]);

    return (<li className="tree node">
        <label>
            {(props.value.children && props.value.children?.length > 0) && <button className='tree' onClick={(e) => {
                e.preventDefault()
                setShowChildren(prev => !prev)
            }}>{showChildren ? "⬇️" : "⬆️"}</button>}
            <input className='tree'
                type={"checkbox"}
                checked={checked}
                onChange={() => {
                    props.bubbleUp && props.bubbleUp(props.value.name, !checked)
                    setChecked(!checked)
                    setLastAction(props.value.name)
                }}
            />
            {props.value.name}
        </label>
        {
            ((props.value.children && props.value.children?.length > 0) && showChildren) &&
            <TreeComponentWrapper
                value={props.value.children}
                parent={props.value}
                bubbleUp={(caller, bool) => {

                    if (props.value.children?.find(val => val.name === caller) && !bool) {
                        setChecked(bool)
                        props.bubbleUp && props.bubbleUp(props.value.name, bool)
                        setLastAction(caller)
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


const getCurrentSelection = (obj: Selection, keys: string[], currentKey: string) => {
    let select = obj as Selection
    keys.forEach(key => {
        // @ts-ignore
        select = select[key].children
    })
    return select[currentKey]
}

const createSelectionOutOfTreeNodes = (input: TreeData[], output: Selection) => {
    for (let index = 0; index < input.length; index++) {
        const element = input[index];
        output[element.name] = {
            value: false,
            children: {}
        }
        if (element.children) {
            createSelectionOutOfTreeNodes(element.children, output[element.name].children)
        }
    }
}