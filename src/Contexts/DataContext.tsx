import React, { createContext, useContext, useEffect, useMemo, useState } from "react"
import { getOrInitPricing } from "../Services/pricing"

export interface Options {
    copies: number | null,
    digital: boolean,
    print: Print | null
}

export enum Mode {
    AUTOMATIC,
    MANUAL
}
export enum Print {
    "B&W",
    COLOR
}

export interface Plan {
  title: string
  price: number
  strips: 2 | 4 | 6
  popular: boolean
}

interface DataContextProps {
    options: Options,
    setOptions: React.Dispatch<React.SetStateAction<Options>>,
    setPlans: React.Dispatch<React.SetStateAction<Plan[]>>
    plans: Array<Plan>,
    digitalEnabled: boolean,
    setDigitalEnabled: React.Dispatch<React.SetStateAction<boolean>>,
    mode: Mode,
    setMode: React.Dispatch<React.SetStateAction<Mode>>,
    images: Array<string>
    setImages: React.Dispatch<React.SetStateAction<Array<string>>>,
}

const DataContext = createContext<DataContextProps | undefined>(undefined)

export const useData = () => {
    const context = useContext(DataContext);
    if (!context) throw new Error("useData must be used within a DataProvider")

    return context
}

export default function DataProvider({ children }: { children: React.ReactNode }) {
    const [options, setOptions] = useState<Options>({ copies: null, digital: false, print: null })
    const [mode, setMode] = useState<Mode>(Mode.AUTOMATIC)
    const [images, setImages] = useState<Array<string>>([]);
    const [digitalEnabled, setDigitalEnabled] = useState<boolean>(false)
    const [plans, setPlans] = useState<Plan[]>([]);

    const defaults = useMemo<Plan[]>(() => [
        {
            strips: 2,
            title: 'Duo Delight',
            price: 199,
            popular: false
        },
        {
            strips: 4,
            title: 'Fantastic Four',
            price: 399,
            popular: true
        },
        {
            strips: 6,
            title: 'Super Six',
            price: 599,
            popular: false
        },
    ], [])

    useEffect(() => {
        const fetch = async () => {
            try {
                let data = await getOrInitPricing(defaults)
                setPlans(data)
            } catch (e) {
                console.error(e)
                if (plans.length < 1) setPlans(defaults)
            }
        }

        fetch()
    }, [])

    return (
        <DataContext.Provider value={{ options, setOptions, plans, setPlans, mode, setMode, images, setImages, digitalEnabled, setDigitalEnabled }}>
            {children}
        </DataContext.Provider>
    )
}