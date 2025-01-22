import { motion } from 'framer-motion'
import { useEffect } from 'react'

import './styles.css'
import reset from '../../Utils/reset'
import { useData } from '../../Contexts/DataContext'
import { useNavigate } from 'react-router-dom'

export default function Greeting() {
  const { setOptions } = useData()
  const navigate = useNavigate()

  const greetings = [
    "Photos so good, they might break the internet!",
    "Your photos are hotter than the flash we just used!",
    "Warning: These pictures may cause excessive smiling!",
    "We hope you love these pics as much as the camera loved you!",
    "Caution: These photos may cause extreme nostalgia in the future.",
    "Looking this good should be illegal!"
  ]

  useEffect(() => {
    const resetTimeout = setTimeout(() => reset(setOptions, navigate), 5000)

    return () => clearTimeout(resetTimeout)
  }, [])

  return (
    <motion.div
      id='greeting'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
        <div className='greeting-container'>
          <div className="greeting-title">{greetings[Math.floor(Math.random() * greetings.length)]}</div>
          <div className="greeting-subtitle">Your Prints are ready to be collected outside</div>
        </div>
    </motion.div>
  )
}
