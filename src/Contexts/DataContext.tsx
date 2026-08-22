import React, { createContext, useContext, useEffect, useMemo, useState } from "react"
import { getOrInitConfig, getOrInitLayouts, getOrInitPages } from "../Services/commands"
import { Addon, Layout, LayoutData, Mode, Options, Plan } from "../types"

export interface Config {
    plans: Plan[],
    digital: Addon
}

interface DataContextProps {
    options: Options,
    setOptions: React.Dispatch<React.SetStateAction<Options>>,

    setConfig: React.Dispatch<React.SetStateAction<Config>>
    config: Config,

    setLayouts: React.Dispatch<React.SetStateAction<LayoutData[]>>
    layouts: Array<LayoutData>,

    mode: Mode,
    setMode: React.Dispatch<React.SetStateAction<Mode>>,

    images: Array<string>
    setImages: React.Dispatch<React.SetStateAction<Array<string>>>,

    pages: number,
    setPages: React.Dispatch<React.SetStateAction<number>>
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
    const [config, setConfig] = useState<Config>({
        plans: [],
        digital: { enabled: false, price: 0, title: "Digital Copy" }
    });
    const [layouts, setLayouts] = useState<LayoutData[]>([]);
    const [pages, setPages] = useState<number>(0);

    const defaultConfig = useMemo<Config>(() => ({
        plans: [
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
        ],
        digital: {
            enabled: false,
            price: 99,
            title: "Digital Copy"
        }
    }), [])

    const defaultLayouts = useMemo<LayoutData[]>(() => [
        {
            title: "Big frame.\nOwn it.",
            kind: Layout.A,
            disabled: false,
            disclaimer: "1-2 people"
        },
        {
            title: "Four shots.\nMake them count.",
            kind: Layout.B,
            disabled: false,
            disclaimer: "1-2 per shot"
        },
        {
            title: "Full frame.\nFull energy.",
            kind: Layout.C,
            disabled: false,
            disclaimer: "up to 5 people"
        },
    ], [])

    useEffect(() => {
        const fetch = async () => {
            try {
                let configData = await getOrInitConfig(defaultConfig)
                setConfig(configData)

                let layoutData = await getOrInitLayouts(defaultLayouts)
                setLayouts(layoutData)

                let pages = await getOrInitPages()
                setPages(pages)
            } catch (e) {
                console.error(e)
                setConfig(defaultConfig)
                if (layouts.length < 1) setLayouts(defaultLayouts)
            }
        }

        fetch()
    }, [])

    const value = {
        options,
        setOptions,
        config,
        setConfig,
        layouts,
        setLayouts,
        mode,
        setMode,
        images,
        setImages,
        pages,
        setPages
    }

    return (
        <DataContext.Provider value={value}>
            {children}
        </DataContext.Provider>
    )
}