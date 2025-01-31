import React from 'react'
import { useNavigate } from 'react-router-dom'

import clear from '../../assets/Images/clear.svg'
import backspace from '../../assets/Images/backspace.svg'

import './styles.css'

export default function Numpad({
    correct,
    code,
    setCode,
    setStatus
}: {
    correct: string,
    code: Array<string>,
    setCode: React.Dispatch<React.SetStateAction<Array<string>>>,
    setStatus: React.Dispatch<React.SetStateAction<number>>
}) {
	const navigate = useNavigate()

    function handleNumPress(num: string): void {
        const nextIndex = code.findIndex((digit) => digit === "")
        if (nextIndex === -1) return

        const updatedCode = [...code]
        updatedCode[nextIndex] = num
        setCode(updatedCode)

        if (nextIndex === 3 && updatedCode.join("") === correct) {
            setStatus(1)
			setTimeout(() => {
				navigate("/mode")
			}, 1000);
        }

        if (nextIndex === 3 && updatedCode.join("") !== correct) {
			setStatus(0)
			setTimeout(() => {
				setStatus(-1)
				setCode(["", "", "", ""])
			}, 1000);
        }
    }

    function handleBackspace(): void {
        const nextIndex = code.findIndex((digit) => digit === "");
        const indexToClear = nextIndex === -1 ? 3 : nextIndex - 1;

        if (indexToClear >= 0) {
            const updatedCode = [...code];
            updatedCode[indexToClear] = "";
            setCode(updatedCode);
        }
    }

  return (
    <div id='numpad'>
        {["1", "2", "3", "4", "5", "6", "7", "8", "9"].map((num, index) => (
          <button
            className="numpad-btn"
            key={index}
            onClick={() => handleNumPress(num)}
          >
            {num}
          </button>
        ))}
        <button
          className="numpad-btn"
          onClick={() => setCode(["", "", "", ""])}
        >
          <img src={clear} alt="" />
        </button>
        <button
          className="numpad-btn"
          onClick={() => handleNumPress('0')}
        >
          0
        </button>
        <button
          className="numpad-btn"
          onClick={() => handleBackspace()}
        >
          <img src={backspace} alt="" />
        </button>
    </div>
  )
}