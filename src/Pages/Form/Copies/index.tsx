import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'

import CopiesSelectable from '../../../Components/CopiesSelectable'
import Footer from '../../../Components/Footer'

import { useData } from '../../../Contexts/DataContext'
import reset from '../../../Utils/reset'

import './styles.css'

export default function Copies() {
  const { plans, options, setOptions, setImages } = useData()

  const navigate = useNavigate()

  return (
    <motion.div
      id='copies'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
        <div className='copies-container'>
          <h1 className="heading">Pick your <div>Perfect</div> Strip Package!</h1>
          <div className="plans-container">
            {plans.map(plan => <CopiesSelectable data={plan} selected={options.copies == plan.strips} />)}
            {/* <div className="digital-container" data-selected={options.digital}> */}
            {/* <div className="digital-container" data-selected={options.digital} onClick={() => setOptions(prev => ({ ...prev, digital: !prev.digital }))}>
              <div className="digital-grp-2">
                <div className="digital-title">Digital Copy</div>
                <div className="digital-label">Add-On</div>
              </div>
              <div className="digital-grp-1">
                <div className="digital-price">₹99</div>
                <div className="add-btn">{options.digital ? "Added" : "Add"}</div>
              </div>
            </div> */}
          </div>
        </div>
        <Footer
          backCallback={() => reset(setOptions, setImages, navigate)}
          continueCallback={() => navigate('/print')}
          disabled={!options.copies}
        />
    </motion.div>
  )
}
