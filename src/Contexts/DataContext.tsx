import React, { createContext, useContext, useMemo, useState } from "react"

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

interface DataContextProps {
    options: Options,
    setOptions: React.Dispatch<React.SetStateAction<Options>>,
    plans: Array<{
        strips: number,
        title: string,
        price: number,
        popular: boolean
    }>,
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

    const plans = useMemo(() => [
        {
            strips: 2,
            title: 'Duo Delight',
            price: 249,
            popular: false
        },
        {
            strips: 4,
            title: 'Fantastic Four',
            price: 449,
            popular: true
        },
        {
            strips: 6,
            title: 'Super Six',
            price: 599,
            popular: false
        },
    ], [])

    return (
        <DataContext.Provider value={{ options, setOptions, plans, mode, setMode, images, setImages }}>
            {children}
        </DataContext.Provider>
    )
}