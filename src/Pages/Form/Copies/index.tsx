import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'

import CopiesSelectable from '../../../Components/CopiesSelectable'
import Footer from '../../../Components/Footer'

import { useData } from '../../../Contexts/DataContext'

import './styles.css'

export default function Copies() {
  const { plans, options, setOptions, digitalEnabled } = useData()

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
            {plans.map((plan, idx) => <CopiesSelectable key={idx} data={plan} selected={options.copies == plan.copies} />)}
            {digitalEnabled && <div className="digital-container" data-selected={options.digital} onClick={() => setOptions(prev => ({ ...prev, digital: !prev.digital }))}>
              <div className="digital-grp-2">
                <div className="digital-title">Digital Copy</div>
                <div className="digital-label">Add-On</div>
              </div>
              <div className="digital-grp-1">
                <div className="digital-price">₹99</div>
                <div className="add-btn">{options.digital ? "Added" : "Add"}</div>
              </div>
            </div>}
          </div>
        </div>
        <Footer
          backCallback={() => navigate(-1)}
          continueCallback={() => navigate('/print')}
          disabled={!options.copies}
        />
        <div
          style={{
            position: 'absolute',
            bottom: '4.5rem',
            opacity: 0.8,
            fontSize: '1.2rem'
          }}
        >
          NOTE: For Layout C, each copy includes 2 strips.
        </div>
    </motion.div>
  )
}
