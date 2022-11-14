import React from "react";

export interface KeycardRequestProps { }
export const KeycardRequest: React.FC<KeycardRequestProps> = (props) => {

    return (<>
        <KeycardRequestForm />
    </>)
}
export interface KeycardRequestFormProps { }
export const KeycardRequestForm: React.FC<KeycardRequestFormProps> = (props) => {
    const [isForAPeriodOnly, setIsForAPeriodOnly] = React.useState(false);
    return (<>
        <h1>Antrag auf neue Keycard</h1>
        <form>
            <label>
                Grund der neuen Keycard:
                <textarea />
            </label>
            <label>
                Zeitlich befristet:
                <input
                    checked={isForAPeriodOnly}
                    onChange={() => setIsForAPeriodOnly(prev => !prev)}
                    type={"checkbox"}
                />
            </label>
            {isForAPeriodOnly && <>
                <label>
                    Bis wann:
                    <input type={"date"} />
                </label>
            </>}
        </form>
    </>)
}