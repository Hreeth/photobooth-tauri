import { motion } from 'framer-motion'

import ModeSelectable from '../../../Components/ModeSelectable'

import { Mode as ModeOptions } from '../../../Contexts/DataContext'
import { useData } from '../../../Contexts/DataContext'

import './styles.css'

export default function Mode() {
  const arr = [
    ModeOptions.MANUAL,
    ModeOptions.AUTOMATIC,
  ]
  const { mode, setDigitalEnabled, digitalEnabled } = useData()

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
      <div className="digital-container" data-selected={digitalEnabled} onClick={() => setDigitalEnabled(prev => !prev)}>
        <div className="digital-grp-2">
          <div className="digital-title">Digital Downloads</div>
        </div>
        <div className="digital-grp-1">
          <div className="add-btn">{digitalEnabled ? "Enabled" : "Disabled"}</div>
        </div>
      </div>
    </motion.div>
  )
}
