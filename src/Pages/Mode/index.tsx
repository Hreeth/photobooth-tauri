import { useData } from '../../Contexts/DataContext'
import { useNavigate } from 'react-router-dom'
import { motion } from 'framer-motion'
import { Mode as ModeOptions } from '../../Contexts/DataContext'

import './styles.css'
import ModeSelectable from '../../Components/ModeSelectable'
import Header from '../../Components/Header'

export default function Mode() {
  const arr = [
    ModeOptions.MANUAL,
    ModeOptions.AUTOMATIC,
  ]
  const { mode } = useData()

  const navigate = useNavigate()

  return (
    <motion.div
      id='mode'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
        <Header backCallback={() => navigate('/')} />
        <div className='mode-container'>
          <h1 className="heading">Choose what you <div>like?</div></h1>
          <div className="selectables-container">
            {arr.map(item => <ModeSelectable data={item} selected={mode == item} />)}
          </div>
        </div> 
    </motion.div>
  )
}
