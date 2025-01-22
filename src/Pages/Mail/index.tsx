import { motion } from 'framer-motion'
import { useState } from 'react'

import './styles.css'
import { useNavigate } from 'react-router-dom'

export default function Mail() {
  const navigate = useNavigate()
  const [email, onSetEmail] = useState("")

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
                onClick={() => navigate('/greeting')}
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
