import React from "react"
import { AxisOptions, Chart } from "react-charts"
import { addDays } from 'date-fns'
export interface StatsDemoProps { }
type DailyStars = {

    date: Date,

    stars: number,

}
type Series = {

    label: string,

    data: DailyStars[]

}

const dataGen = (): Series[] => {
    const arr = [] as Series[]
    const data = [] as DailyStars[]
    const today = new Date()
    for (let index = 0; index < 10; index++) {
        let day = addDays(today, -index)
        data.push({
            date: day,
            stars: Math.floor(index * 10 * Math.random())
        })

    }
    return [{
        data,
        label: "stats"
    }]
}

const data: Series[] = [

    {

        label: 'React Charts',

        data: [

            {

                date: new Date(),

                stars: 202123,

            }

            // ...

        ]

    },

    {

        label: 'React Query',

        data: [

            {

                date: new Date(),

                stars: 10234230,

            }

            // ...

        ]

    }

]
export const StatsDemo: React.FC<StatsDemoProps> = (props) => {

    const primaryAxis = React.useMemo(

        (): AxisOptions<DailyStars> => ({

            getValue: stars => stars.date,

        }),

        []

    )



    const secondaryAxes = React.useMemo(

        (): AxisOptions<DailyStars>[] => [

            {

                getValue: stars => stars.stars,


            },

        ],

        []

    )



    return (

        <div>

            <h2>Beispiel Statistik</h2>
            <button onClick={(e) => {
                e.preventDefault()
            }}>
                Daten Export
            </button>
            <p>
                ...Weitere Statistiken folgen
            </p>
            <Chart

                options={{

                    data: dataGen(),

                    primaryAxis,

                    secondaryAxes,

                }}

            />

        </div>

    )
}