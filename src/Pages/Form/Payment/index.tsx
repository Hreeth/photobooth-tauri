import { motion } from 'framer-motion'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { DotLottieReact } from '@lottiefiles/dotlottie-react'

import { useData } from '../../../Contexts/DataContext'
import usePayment from '../../../Hooks/usePayment'
import calculate from '../../../Utils/calculate'
import reset from '../../../Utils/reset'

import Header from '../../../Components/Header'

import './styles.css'

export default function Payment() {
  const navigate = useNavigate()
  const [time, setTime] = useState<number>(0)
  const { qrCode, loading, fetchQrCode, paid, pollingIntervalRef } = usePayment()

  const dev = false;

  const { setOptions, options, config, mode } = useData()

  useEffect(() => {
    fetchQrCode(calculate(options, mode, config))
  }, [fetchQrCode])
  
  useEffect(() => {
    if (dev) navigate('/camera')

    switch (paid) {
      case false: {
        reset(setOptions, navigate)
        break
      }
      case true: {
        navigate('/camera')
        break
      }
      default: {
        break
      }
    }
  }, [paid])

  useEffect(() => {
    if (qrCode) {
      const timeInterval = setInterval(() => {
        const currentTime = Math.floor(Date.now() / 1000)
        const timeLeft = qrCode.close_by - currentTime

        if (timeLeft <= 0) {
          clearInterval(timeInterval)
          setTime(0)
        } else {
          setTime(timeLeft)
        }
      }, 1000);

      return () => clearInterval(timeInterval)
    }
  }, [qrCode, setOptions, navigate])

  return (
    <motion.div
      id='payment'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
        <Header
          backCallback={() => {
            if (pollingIntervalRef.current) clearInterval(pollingIntervalRef.current)
            navigate(-1)
          }}
        />
        {loading && (
          <div
            style={{
              width: "100%",
              height: "100vh",
              display: "flex",
              justifyContent: "center",
              placeItems: "center"
            }}
          >
            <DotLottieReact
              className='greeting-progress-loader'
              src='/loader.lottie'
              loop
              autoplay
            />
          </div>
        )}
        {
          !loading && (
            <div className='payment-container'>
              <div className="payment-heading">
                <div className="payment-title">Scan the QR to make payment</div>
                <div className="payment-subtitle">QR will expire in {time}s</div>
              </div>
              <div className="qr-container">
                <div className="qr-title">Payment</div>
                <div id="qr">
                  <img src={qrCode?.image_url} alt="" />
                </div>
              </div>
            </div>
          )
        }
    </motion.div>
  )
}
