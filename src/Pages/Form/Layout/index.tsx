import { motion } from 'framer-motion'
import { useEffect, useRef } from 'react'
import { useNavigate } from 'react-router-dom'

import { useData } from '../../../Contexts/DataContext'
import reset from '../../../Utils/reset'

import Footer from '../../../Components/Footer'
import LayoutSelectable from '../../../Components/LayoutSelectable'

import './styles.css'

export default function Layout() {
  const { layouts, options, setOptions } = useData()

  const navigate = useNavigate()
  const layoutsContainerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const container = layoutsContainerRef.current

    if (!container) return

    container.scrollLeft =
      (container.scrollWidth - container.clientWidth) / 2
  }, [layouts])

  return (
    <motion.div
      id='layout'
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
    >
      <div className='layout-container'>
        <h1 className='heading'>
          Pick the <div>Ideal</div> Layout!
        </h1>

        <div
          ref={layoutsContainerRef}
          className='layouts-container'
        >
          <div className='layouts-track'>
            {layouts
              .filter((layout) => !layout.disabled)
              .map((layout, idx) => (
                <LayoutSelectable
                  key={idx}
                  data={layout}
                  selected={options.layout == layout.kind}
                />
              ))}
          </div>
        </div>
      </div>

      <Footer
        backCallback={() => reset(setOptions, navigate)}
        continueCallback={() => navigate('/copies')}
        disabled={!options.layout}
      />
    </motion.div>
  )
}