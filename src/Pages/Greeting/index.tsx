import { motion } from 'framer-motion'
import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { invoke } from '@tauri-apps/api/core'
import { pictureDir } from '@tauri-apps/api/path'

import { Print, useData } from '../../Contexts/DataContext'
import reset from '../../Utils/reset'

import './styles.css'
import { path } from '@tauri-apps/api'
import { DotLottieReact } from '@lottiefiles/dotlottie-react'

export default function Greeting() {
  const { setOptions, options, images, setImages } = useData()
  const navigate = useNavigate()

  const greetings = [
    "Photos so good, they might break the internet!",
    "Your photos are hotter than the flash we just used!",
    "Warning: These pictures may cause excessive smiling!",
    "We hope you love these pics as much as the camera loved you!",
    "Caution: These photos may cause extreme nostalgia in the future.",
    "Looking this good should be illegal!"
  ]

  const [progressText, setProgressText] = useState("0 of 0")
  const [showLoader, setShowLoader] = useState(true)

  const stripCount = options.copies || 2

  useEffect(() => {
    const printPhotos = async () => {
      try {
        let pictures = await pictureDir()
        let img_path = await path.join(pictures, "print-strip.png")
        await invoke("print", {
          images: images,
          outputPath: img_path,
          colorMode: options.print == Print.COLOR ? "COLOR" : "B&W",
          copies: options.copies
        })

        console.log("Print successful")
      } catch (err) {
        console.error("Error during the printing:", err)
      } finally {
        let resetInterval = setTimeout(() => {
          reset(setOptions, setImages, navigate)
          clearTimeout(resetInterval)
        }, 40000)
      }
    }

    printPhotos()

    setProgressText(`0 of ${stripCount}`)
    setShowLoader(true)

    const timers: NodeJS.Timeout[] = []

    if (stripCount >= 2) {
      timers.push(setTimeout(() => {
        setProgressText(`2 of ${stripCount}`)
        setShowLoader(stripCount > 2)
      }, 16000))
    }

    if (stripCount >= 4) {
      timers.push(setTimeout(() => {
        setProgressText(`4 of ${stripCount}`)
        setShowLoader(stripCount > 4)
      }, 30000))
    }

    if (stripCount === 6) {
      timers.push(setTimeout(() => {
        setProgressText(`6 of 6`)
        setShowLoader(false)
      }, 40000))
    }

    return () => {
      timers.forEach(clearTimeout)
    }

  }, [setOptions, navigate, options])

  return (
    <motion.div
      id='greeting'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
      <div className='greeting-container'>
        <div className="greeting-title">
          {greetings[Math.floor(Math.random() * greetings.length)]}
        </div>
        <div className="greeting-subtitle">
          Collect your prints here
        </div>
        <div className="greeting-progress">
          {progressText}
          {showLoader && <DotLottieReact
            className='greeting-progress-loader'
            src='/loader.lottie'
            loop
            autoplay
          />}
        </div>
        <DotLottieReact
          className='greeting-arrow'
          src='/downArrow.lottie'
          loop
          autoplay
          width={1000}
          height={1000}
        />
      </div>
    </motion.div>
  )
}
