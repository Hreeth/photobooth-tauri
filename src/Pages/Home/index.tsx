import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'

import arrow from '../../assets/Images/arrow.png'

import './styles.css'
import { useEffect, useState } from 'react'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

export default function Home() {
    const [qPressCount, setQPressCount] = useState(0)

    let navigate = useNavigate()
  let rand = Math.floor(Math.random() * 3)

    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (e.key.toLowerCase() == 'q') {
                setQPressCount(prev => prev + 1)
            } else {
                setQPressCount(0)
            }
        }

        const controller = new AbortController()
        document.addEventListener('keydown', handleKeyDown, { signal: controller.signal })

        return () => controller.abort()
    }, [])
    
    useEffect(() => {
        if (qPressCount >= 4) {
            WebviewWindow.getCurrent().close()
        }
    }, [qPressCount])

  return (
    <motion.div
      id='home'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
        <div className='home-container'>
            <div className="heading-col-1">
                <div className='heading'>
                    {
                        rand == 0
                            ?
                        <>
                            <h1 className='heading-text'>Every <div className='heading-highlight'>CLICK</div> tells</h1>
                            <h1 className='heading-text'>a story</h1>
                            {/* <h1 className='heading-subtitle'>what's yours?</h1> */}
                        </>
                            :
                        rand == 1
                            ?
                        <>
                            <h1 className='heading-text'>Where <div className='heading-highlight'>SMILES</div> turn</h1>
                            <h1 className='heading-text'>into keepsakes</h1>
                            {/* <h1 className='heading-subtitle'>and memories last</h1> */}
                        </>
                            :
                        rand == 2
                            ?
                        <>
                            <h1 className='heading-text'>Picture the <div className='heading-highlight'>MAGIC</div>,</h1>
                            <h1 className='heading-text'>treasure the <div className='heading-highlight'>MOMENT</div></h1>
                            {/* <h1 className='heading-subtitle'>cherish forever</h1> */}
                        </>
                            :
                        <>
                            <h1 className='heading-text'>Experience the</h1>
                            <h1 className='heading-text'><div className='heading-highlight'>ULTIMATE</div> photobooth</h1>
                            {/* <h1 className='heading-subtitle'>unforgettable</h1> */}
                        </>
                    }
                </div>
                <button className='admin-btn' onClick={() => navigate('/passcode')} />
            </div>
            <div className='heading-col-2'>
                <div className='arrow-img'>
                    <img src={arrow} />
                </div>
                <button className='start-btn' onClick={() => navigate('/copies')}>Start</button>
            </div>
        </div>
    </motion.div>
  )
}
