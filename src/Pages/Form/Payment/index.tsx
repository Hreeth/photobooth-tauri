import { motion } from 'framer-motion'
import { useEffect } from 'react'
import Header from '../../../Components/Header'
import { useNavigate } from 'react-router-dom'

import qr from '../../../assets/Images/sampleQR.png'

import './styles.css'

export default function Payment() {
  const navigate = useNavigate()

  useEffect(() => {
    const changeTimeout = setTimeout(() => {
      navigate('/countdown')
    }, 5000);

    return () => clearTimeout(changeTimeout)
  }, [])
  
  return (
    <motion.div
      id='payment'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
        <Header />
        <div className='payment-container'>
          <div className="payment-heading">
            <div className="payment-title">Scan the QR to make payment</div>
            <div className="payment-subtitle">QR will expire in 129s</div>
          </div>
          <div className="qr-container">
            <div className="qr-title">Payment</div>
            <div id="qr">
              <img src={qr} alt="" />
            </div>
          </div>
        </div>
    </motion.div>
  )
}
