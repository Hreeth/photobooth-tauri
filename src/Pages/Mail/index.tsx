import { motion } from 'framer-motion'
import { useEffect, useRef, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { documentDir } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'

import { useData } from '../../Contexts/DataContext'

import Keyboard from '../../Components/Keyboard'

import './styles.css'

export default function Mail() {
  const navigate = useNavigate()
  const [email, onSetEmail] = useState("")
  const [selectionStart, setSelectionStart] = useState<number>(0)
  const [selectionEnd, setSelectionEnd] = useState<number>(0)
  const [documentPath, setDocumentPath] = useState("")
  const [keyboardVisible, setKeyboardVisible] = useState(false)

  const keyboardRef = useRef(null)
  const inputRef = useRef<HTMLInputElement | null>(null)

  const { images, options } = useData()

  useEffect(() => {
    async function fetchPath() {
      const path = await documentDir()
      setDocumentPath(path)
    }

    fetchPath()
  }, [])

  async function handleEmail() {
    try {
      await invoke<string>("store_email", {
        documentPath: documentPath,
        userEmail: email,
        photoPaths: images,
        layout: options.layout
      })
    } catch (err) {
      console.error("Error storing email:", err)
    } finally {
      navigate("/greeting")
    }
  }

  const validate = (): boolean => {
    return /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(email)
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

  useEffect(() => {
    if (!inputRef.current) return
    
    inputRef.current.focus()
    inputRef.current.setSelectionRange(selectionStart, selectionEnd)
  }, [email, selectionStart, selectionEnd])

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
                ref={inputRef}
                type="text"
                inputMode='email'
                className="input"
                placeholder='enteryouremail@gmail.com'
                value={email}
                onFocus={() => setKeyboardVisible(true)}
                onSelect={() => setKeyboardVisible(true)}
                onChange={(e) => {
                  const v = e.target.value
                  onSetEmail(v)
                  setSelectionStart(e.target.selectionStart ?? v.length)
                  setSelectionEnd(e.target.selectionEnd ?? v.length)
                }}
                onClick={(e) => {
                  const el = e.currentTarget
                  setSelectionStart(el.selectionStart ?? 0)
                  setSelectionEnd(el.selectionEnd ?? 0)
                }}
                onKeyUp={(e) => {
                  const el = e.currentTarget
                  setSelectionStart(el.selectionStart ?? 0)
                  setSelectionEnd(el.selectionEnd ?? 0)
                }}
                onKeyDown={(e) => {
                  const el = e.currentTarget
                  requestAnimationFrame(() => {
                    setSelectionStart(el.selectionStart ?? 0)
                    setSelectionEnd(el.selectionEnd ?? 0)
                  })
                }}
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
            <Keyboard
              value={email}
              onChange={(next) => onSetEmail(next)}
              selectionStart={selectionStart}
              selectionEnd={selectionEnd}
              onSelectionChange={(start, end) => {
                setSelectionStart(start)
                setSelectionEnd(end)
              }}
            />
          </div>
        )}
    </motion.div>
  )
}