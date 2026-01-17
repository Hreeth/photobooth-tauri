import React, { createContext, useContext, useEffect, useMemo, useState } from "react"
import { getOrInitLayouts, getOrInitPricing } from "../Services/commands"

export interface Options {
    layout: Layout | null,
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
export enum Layout { A = "A", B = "B", C = "C" }

export interface Plan {
  title: string
  price: number
  copies: 1 | 2 | 3
  popular: boolean
}

export interface LayoutData {
  kind: Layout,
  disabled: boolean
}

interface DataContextProps {
    options: Options,
    setOptions: React.Dispatch<React.SetStateAction<Options>>,
    setPlans: React.Dispatch<React.SetStateAction<Plan[]>>
    plans: Array<Plan>,
    setLayouts: React.Dispatch<React.SetStateAction<LayoutData[]>>
    layouts: Array<LayoutData>,
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
    const [options, setOptions] = useState<Options>({
        layout: null,
        copies: null,
        digital: false,
        print: null
    })
    const [mode, setMode] = useState<Mode>(Mode.AUTOMATIC)
    const [images, setImages] = useState<Array<string>>([]);
    const [digitalEnabled, setDigitalEnabled] = useState<boolean>(false)
    const [plans, setPlans] = useState<Plan[]>([]);
    const [layouts, setLayouts] = useState<LayoutData[]>([]);

    const defaultPlans = useMemo<Plan[]>(() => [
        {
            copies: 1,
            title: 'Solo Special',
            price: 199,
            popular: false
        },
        {
            copies: 2,
            title: 'Duo Delight',
            price: 399,
            popular: true
        },
        {
            copies: 3,
            title: 'Triple Treat',
            price: 599,
            popular: false
        },
    ], [])

    const defaultLayouts = useMemo<LayoutData[]>(() => [
        {
            kind: Layout.A,
            disabled: false
        },
        {
            kind: Layout.B,
            disabled: false
        },
        {
            kind: Layout.C,
            disabled: false
        },
    ], [])

    useEffect(() => {
        const fetch = async () => {
            try {
                let planData = await getOrInitPricing(defaultPlans)
                setPlans(planData)

                let layoutData = await getOrInitLayouts(defaultLayouts)
                setLayouts(layoutData)
            } catch (e) {
                console.error(e)
                if (plans.length < 1) setPlans(defaultPlans)
                if (layouts.length < 1) setLayouts(defaultLayouts)
            }
        }

        fetch()
    }, [])



    const value = {
        options,
        setOptions,
        plans,
        layouts,
        setPlans,
        setLayouts,
        mode,
        setMode,
        images,
        setImages,
        digitalEnabled,
        setDigitalEnabled
    }

    return (
        <DataContext.Provider value={value}>
            {children}
        </DataContext.Provider>
    )
}