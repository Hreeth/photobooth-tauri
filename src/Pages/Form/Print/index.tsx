import { useNavigate } from 'react-router-dom'
import { motion } from 'framer-motion'

import PrintSelectable from '../../../Components/PrintSelectable'

import Footer from '../../../Components/Footer'
import { Print as PrintOptions } from '../../../Contexts/DataContext'
import { useData } from '../../../Contexts/DataContext'

import './styles.css'

export default function Print() {
  const arr = [
    PrintOptions['B&W'],
    PrintOptions.COLOR
  ]
  const { options } = useData()

  const navigate = useNavigate()

  return (
    <motion.div
      id='print'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
        <div className='print-container'>
          <h1 className="heading">Choose what you <div>like?</div></h1>
          <div className="selectables-container">
            {arr.map(item => <PrintSelectable data={item} selected={options.print == item} />)}
          </div>
        </div>
        <Footer
          backCallback={() => navigate(-1)}
          continueCallback={() => navigate('/countdown')}
          disabled={options.print == null}
        />
    </motion.div>
  )
}
