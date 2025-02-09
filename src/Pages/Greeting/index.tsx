import { motion } from 'framer-motion'
import { useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import { invoke } from '@tauri-apps/api/core'
import { pictureDir } from '@tauri-apps/api/path'

import { Print, useData } from '../../Contexts/DataContext'
import reset from '../../Utils/reset'

import './styles.css'
import { path } from '@tauri-apps/api'

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

  useEffect(() => {
    const printPhotos = async () => {
      try {
        let pictures = await pictureDir();
        let img_path = await path.join(pictures, "print-strip.png");
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
        }, 4000);
      }
    }
    
    printPhotos()
  }, [setOptions, navigate, options])

  return (
    <motion.div
      id='greeting'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
        <div className='greeting-container'>
          <div className="greeting-title">{greetings[Math.floor(Math.random() * greetings.length)]}</div>
          <div className="greeting-subtitle">Your Prints are ready to be collected</div>
        </div>
    </motion.div>
  )
}
