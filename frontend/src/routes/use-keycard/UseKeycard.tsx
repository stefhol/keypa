import { useQuery } from "@tanstack/react-query"
import React, { useMemo } from "react"
import { User } from "../../util/intefaces/Request"
import { Rest } from "../../util/Rest"

export const UseKeycard: React.FC<{}> = (props) => {
    const { data: dataKeycard } = useQuery(["keycard"], Rest.getKeycard)
    const { data: building } = useQuery(["building"], Rest.getBuildings)
    const userData = React.useMemo(() => {
        if (Array.isArray(dataKeycard)) {
            const userData: { [key: string]: User } = {}
            for (let index = 0; index < dataKeycard.length; index++) {
                const element = dataKeycard[index];
                if (element.request?.requester.user_id)
                    userData[element.request?.requester.user_id] = element.request.requester
            }
            let uniqueUserData: User[] = []
            for (const key in userData) {
                if (Object.prototype.hasOwnProperty.call(userData, key)) {
                    uniqueUserData.push(userData[key])
                }
            }
            return uniqueUserData
        }
        return []

    }, [dataKeycard?.length])



    const [selectedBuilding, setSelectedBuilding] = React.useState("");
    const [selectedUser, setSelectedUser] = React.useState("");
    const [selectedKeycard, setSelectedKeycard] = React.useState("");
    const [selectedRoom, setSelectedRoom] = React.useState("");
    const [selectedDoor, setSelectedDoor] = React.useState("");
    const [status, setStatus] = React.useState("");
    const filterdKeycardData = useMemo(() => dataKeycard?.filter(val => val.user_id === selectedUser), [selectedUser, dataKeycard?.length])
    const filteredRooms = useMemo(() => building?.filter(val => val?.building_id === selectedBuilding).map(val => val.rooms)?.flat(), [selectedBuilding, building?.length])
    const filteredDoor = useMemo(() => filteredRooms?.filter(val => val?.room_id === selectedRoom).map(val => val.doors)?.flat(), [selectedRoom, filteredRooms?.length])
    return (<>
        <form onSubmit={e => {
            e.preventDefault()
            Rest.quickAdd("use-keycard", "PUT", {
                door_id: selectedDoor,
                keycard_id: selectedKeycard
            }).then(res => {
                if (res.ok) {
                    setStatus(`Success`)
                }
            })
        }}>
            <label>
                User:
                <select value={selectedUser} onChange={e => setSelectedUser(e.target.value)}>
                    <option value=""></option>
                    {userData?.map((val, idx) => <option key={idx} value={val.user_id}>
                        {val.name}
                    </option>)}
                </select>
            </label>
            <label>
                Keycard:
                <select value={selectedKeycard} onChange={e => setSelectedKeycard(e.target.value)}>
                    <option value=""></option>
                    {filterdKeycardData?.map((val, idx) => <option key={idx} value={val.keycard_id}>
                        {val.keycard_id}
                    </option>)}
                </select>
            </label>
            <label>
                Building:
                <select value={selectedBuilding} onChange={e => setSelectedBuilding(e.target.value)}>
                    <option value=""></option>
                    {building?.map((val, idx) => <option key={idx} value={val.building_id}>
                        {val.name}
                    </option>)}
                </select>
            </label>

            <label>
                Room:
                <select value={selectedRoom} onChange={e => setSelectedRoom(e.target.value)}>
                    <option value=""></option>
                    {filteredRooms?.map((val, idx) => <option key={idx} value={val.room_id}>
                        {val.name}
                    </option>)}
                </select>
            </label>
            <label>
                Door:
                <select value={selectedDoor} onChange={e => setSelectedDoor(e.target.value)}>
                    <option value=""></option>
                    {filteredDoor?.map((val, idx) => <option key={idx} value={val.door_id}>
                        {val.door_id}
                    </option>)}
                </select>
            </label>
            <button>Send</button>
        </form>
        {status && <p>{status}</p>}
    </>)
}