import { motion } from 'framer-motion'

import ModeSelectable from '../../../Components/ModeSelectable'

import { useData } from '../../../Contexts/DataContext'
import { Mode as ModeOptions } from '../../../types'

import './styles.css'

export default function Mode() {
  const arr = [
    ModeOptions.MANUAL,
    ModeOptions.AUTOMATIC,
  ]
  const { mode } = useData()

  return (
    <motion.div
      id='admin-mode'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
      <h1 className="heading">Choose what you <div>like?</div></h1>
      <div className="selectables-container">
        {arr.map((item, idx) => <ModeSelectable key={idx} data={item} selected={mode == item} />)}
      </div>
    </motion.div>
  )
}
