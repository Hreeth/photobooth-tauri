import { useState } from "react"

import './styles.css'

export type KeyboardLayout = string[][]

interface KeyboardProps {
    value: string,
    onChange: (next: string) => void,
    selectionStart: number,
    selectionEnd: number,
    onSelectionChange?: (start: number, end: number) => void
}

const defaultLayout: KeyboardLayout = [
    ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
    ["a", "s", "d", "f", "g", "h", "j", "k", "l"],
    ["{lock}", "z", "x", "c", "v", "b", "n", "m", "{bksp}"],
    ["{shift}", ".com", ".in", ".net", ".", "{space}", "@", ","]
]

const shiftLayout: KeyboardLayout = [
    ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"],
    ["A", "S", "D", "F", "G", "H", "J", "K", "L"],
    ["{lock}", "Z", "X", "C", "V", "B", "N", "M", "{bksp}"],
    ["{shift}", ".com", ".in", ".net", ".", "{space}", "@", ","]
]

const symbolLayout: KeyboardLayout = [
    ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"],
    ["!", "@", "#", "$", "%", "^", "&", "*", "(", ")"],
    ["`", "\"", "'", ";", ":", "?", "!", "{bksp}"],
    ["{shift}", ".com", ".in", ".net", ".", "{space}", ","]
]

export default function Keyboard({
    value,
    onChange,
    selectionStart,
    selectionEnd,
    onSelectionChange
}: KeyboardProps) {
    const [mode, setMode] = useState<"default" | "shift" | "symbol">("default")
    const [caps, setCaps] = useState<boolean>(false)

    const layout =
        mode === "symbol" ? symbolLayout :
            mode === "shift" || caps ? shiftLayout :
                defaultLayout;

    function insertText(text: string) {
        const before = value.slice(0, selectionStart)
        const after = value.slice(selectionEnd)

        const next = before + text + after
        const newCursor = before.length + text.length

        onChange(next)
        onSelectionChange?.(newCursor, newCursor)
    }

    function backspace() {
        if (selectionStart !== selectionEnd) {
            const before = value.slice(0, selectionStart)
            const after = value.slice(selectionEnd)

            onChange(before + after)
            onSelectionChange?.(selectionStart, selectionStart)
            return
        }

        if (selectionStart == 0) return

        const before = value.slice(0, selectionStart - 1)
        const after = value.slice(selectionEnd)
        const next = before + after

        const newCursor = selectionStart - 1
        onChange(next)
        onSelectionChange?.(newCursor, newCursor)
    }

    function handleKey(key: string) {
        switch (key) {
            case "{bksp}":
                backspace()
                break
            case "{space}":
                insertText(" ")
                break
            case "{shift}":
                setMode(prev => prev == "symbol" ? "default" : prev == "shift" ? "default" : "symbol")
                break
            case "{lock}":
                setCaps(prev => !prev)
                break
            default:
                insertText(key)
                if (mode == "shift") setMode("default")
                break
        }
    }

    return (
        <div className="keyboard">
            {layout.map((row, i) => (
                <div className="keyboard-row" key={i}>
                    {row.map((key) => (
                        <button
                            key={key}
                            className={`keyboard-key ${key.startsWith("{") ? "keyboard-fn" : ""}`}
                            onClick={(e) => {
                                handleKey(key)
                                e.currentTarget.blur()
                            }}
                            data-key={key}
                            data-mode={mode}
                        >
                            {renderKeyLabel(key, mode)}
                        </button>
                    ))}
                </div>
            ))}
        </div>
    )
}

function renderKeyLabel(key: string, mode: string) {
    switch (key) {
        case "{bksp}": return "⌫";
        case "{space}": return " ";
        case "{shift}": return mode === "symbol" ? "ABC" : "?123";
        case "{lock}": return "⇧";
        default: return key;
    }
}