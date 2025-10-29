import { motion } from 'framer-motion'
import { useEffect, useMemo, useState } from 'react'
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

  const greetings = useMemo(() => [
    "Photos so good, they might break the internet!",
    "Your photos are hotter than the flash we just used!",
    "Warning: These pictures may cause excessive smiling!",
    "We hope you love these pics as much as the camera loved you!",
    "Caution: These photos may cause extreme nostalgia in the future.",
    "Looking this good should be illegal!"
  ], [])

  const [greetingText, _] = useState(
    greetings[Math.floor(Math.random() * greetings.length)]
  )
  const [progressText, setProgressText] = useState("0 of 0")
  const [showLoader, setShowLoader] = useState(true)

  const stripCount = options.copies || 2

  useEffect(() => {
    const timers: NodeJS.Timeout[] = []

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
      }
    }

    printPhotos()

    setProgressText(`0 of ${stripCount}`)
    setShowLoader(true)

    const progressSteps: Record<number, { time: number, text: string }[]> = {
      2: [{ time: 16000, text: "2 of 2" }],
      4: [
        { time: 16000, text: "2 of 4" },
        { time: 30000, text: "4 of 4" },
      ],
      6: [
        { time: 16000, text: "2 of 6" },
        { time: 30000, text: "4 of 6" },
        { time: 40000, text: "6 of 6" },
      ],
    }

    const steps = progressSteps[stripCount] || []

    steps.forEach((step, idx) => {
      const last = idx == steps.length - 1;
      timers.push(setTimeout(() => {
        setProgressText(step.text)
        setShowLoader(!last)

        if (last) {
          setTimeout(() => {
            reset(setOptions, setImages, navigate)
          }, 2000);
        }
      }, step.time))
    })

    return () => timers.forEach(clearTimeout)
  }, [])

  return (
    <motion.div
      id='greeting'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
      <div className='greeting-container'>
        <div className="greeting-title">
          {greetingText}
        </div>
        <div className="greeting-subtitle">
          Collect your prints outside
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
      </div>
    </motion.div>
  )
}
