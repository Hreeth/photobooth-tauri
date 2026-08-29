import { useNavigate } from 'react-router-dom'
import { motion } from 'framer-motion'


import { useData } from '../../../Contexts/DataContext'
import { startSession } from '../../../Services/commands'
import { Mode, Filter as FilterOptions } from '../../../types'

import Footer from '../../../Components/Footer'
import FilterSelectable from '../../../Components/FilterSelectable'

import './styles.css'

export default function Filter() {
  const arr = [
    FilterOptions.BW,
    FilterOptions.Color
  ]
  const { options, mode } = useData()

  const navigate = useNavigate()

  return (
    <motion.div
      id='filter'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
        <div className='filter-container'>
          <h1 className="heading">Choose what you <div>like?</div></h1>
          <div className="selectables-container">
            {arr.map((item, idx) => <FilterSelectable key={idx} data={item} selected={options.filter == item} />)}
          </div>
        </div>
        <Footer
          backCallback={() => navigate(-1)}
          continueCallback={() => {
            void startSession(options)
            mode == Mode.AUTOMATIC ? navigate('/payment') : navigate('/camera')
          }}
          continueText={mode == Mode.AUTOMATIC ? "Continue to Payment" : "Start Countdown"}
          disabled={options.filter == null}
        />
    </motion.div>
  )
}
