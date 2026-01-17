import { motion } from 'framer-motion'
import { useNavigate } from 'react-router-dom'

import Footer from '../../../Components/Footer'

import { useData } from '../../../Contexts/DataContext'
import reset from '../../../Utils/reset'

import './styles.css'
import LayoutSelectable from '../../../Components/LayoutSelectable'

export default function Layout() {
  const { layouts, options, setOptions, setImages } = useData()

  const navigate = useNavigate()

  return (
    <motion.div
      id='layout'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
        <div className='layout-container'>
          <h1 className="heading">Pick the <div>Ideal</div> Layout!</h1>
          <div className="layouts-container">
            {layouts.map((layout, idx) => <LayoutSelectable key={idx} data={layout} selected={options.layout == layout.kind} />)}
          </div>
        </div>
        <Footer
          backCallback={() => reset(setOptions, setImages, navigate)}
          continueCallback={() => navigate('/copies')}
          disabled={!options.layout}
        />
    </motion.div>
  )
}
