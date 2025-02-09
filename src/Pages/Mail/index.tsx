import { motion } from 'framer-motion'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'

import './styles.css'
import { documentDir } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'
import { useData } from '../../Contexts/DataContext'

export default function Mail() {
  const navigate = useNavigate()
  const [email, onSetEmail] = useState("")
  const [documentPath, setDocumentPath] = useState("")

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
    .then(() => invoke("send_email", { documentPath: documentPath }))
    .catch((err) => console.error("Error storing / sending email:", err))
  }

  const validate = (): boolean => {
    return /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(email)
  }

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
    </motion.div>
  )
}
