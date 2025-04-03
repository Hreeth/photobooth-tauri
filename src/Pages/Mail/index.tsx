import { motion } from 'framer-motion'
import { useEffect, useRef, useState } from 'react'
import { useNavigate } from 'react-router-dom'

import './styles.css'
import { documentDir } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'
import { useData } from '../../Contexts/DataContext'
import KeyboardReact from 'react-simple-keyboard'
import 'react-simple-keyboard/build/css/index.css';

export default function Mail() {
  const navigate = useNavigate()
  const [email, onSetEmail] = useState("")
  const [documentPath, setDocumentPath] = useState("")
  const [keyboardVisible, setKeyboardVisible] = useState(false)
  const [layoutName, setLayoutName] = useState("default")

  const keyboardRef = useRef(null)

  const { images } = useData()

  useEffect(() => {
    async function fetchPath() {
      const path = await documentDir()
      setDocumentPath(path)
    }

    fetchPath()
  }, [])

  async function handleEmail() {
    navigate("/greeting")

    invoke<string>("store_email", {
      documentPath: documentPath,
      userEmail: email,
      photoPaths: images
    })
    .then(async () => await invoke("send_email", { documentPath: documentPath }))
    .catch((err) => console.error("Error storing / sending email:", err))
  }

  const validate = (): boolean => {
    return /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(email)
  }

  function handleKeyPress(btn: string) {
    switch (btn) {
      case "{lock}": {
        setLayoutName(prev => prev == "lock" ? "default" : "lock")
        break
      }
      case "{shift}": {
        setLayoutName(prev => prev == "shift" ? "default" : "shift")
        break
      }
      case "{bksp}": {
        onSetEmail(prev => prev.slice(0, -1))
        break
      }
      default: break
    }
  }

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        keyboardRef.current &&
        !(keyboardRef.current as HTMLElement).contains(event.target as Node) &&
        !(document.getElementById("email-input") as HTMLElement)?.contains(event.target as Node)
      ) {
        setKeyboardVisible(false)
      }
    }

    if (keyboardVisible) {
      document.addEventListener("mousedown", handleClickOutside)
    } else {
      document.removeEventListener("mousedown", handleClickOutside)
    }

    return () => document.removeEventListener("mousedown", handleClickOutside)
  }, [keyboardVisible])

  return (
    <motion.div
      id='mail'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
        <div className='mail-container'>
          <h1 className="heading">Enter <div>email</div> to collect <div>digital downloads</div></h1>
          <div className="input-container">
            <div className="input-box">
              <input
                type="email"
                className="input"
                onChange={(_) => onSetEmail(_.target.value.trim())}
                placeholder='enteryouremail@gmail.com'
                onFocus={() => setKeyboardVisible(true)}
                value={email}
              />
              <button
                className="send-btn"
                onClick={() => handleEmail()}
                disabled={email == "" || !validate()}
              >
                Submit
              </button>
            </div>
            {(email != "" && !validate()) && <div className="err-text">Please enter a correct email</div>}
          </div>
        </div>
        <div className="disclaimer">
          Your photos will be sent to the provided email within 24 hours.
          <br />
          This email may also be used for marketing purposes, with an option to unsubscribe anytime.
        </div>
        {keyboardVisible && (
          <div id='keyboard' ref={keyboardRef}>
            <KeyboardReact
              onChange={(input) => onSetEmail(input)}
              inputName='email'
              layoutName={layoutName}
              onKeyPress={(btn) => handleKeyPress(btn)}
              layout={{
                default: [
                  "q w e r t y u i o p",
                  "a s d f g h j k l",
                  "{lock} z x c v b n m {bksp}",
                  "{shift} .com .in .net . {space} @ ,",
                ],
                lock: [
                  "Q W E R T Y U I O P",
                  "A S D F G H J K L",
                  "{lock} Z X C V B N M {bksp}",
                  "{shift} .com .in .net . {space} @ ,"
                ],
                shift: [
                  "1 2 3 4 5 6 7 8 9 0",
                  "! @ # $ % ^ & * ( )",
                  "` \" \' : ; ! ? {bksp}",
                  "{shift} .com .in .net . {space} ,",
                ],
              }}
              display={{
                "{bksp}": "⌫",
                "{lock}": "⇧",
                "{shift}": layoutName == "shift" ? "ABC" : "?123",
                "{space}": " "
              }}
            />
          </div>
        )}
    </motion.div>
  )
}
