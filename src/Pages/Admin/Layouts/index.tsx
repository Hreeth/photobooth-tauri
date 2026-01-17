import { motion } from 'framer-motion'
import { useEffect, useState } from 'react'

import { LayoutData, useData } from '../../../Contexts/DataContext'
import { saveLayouts } from '../../../Services/commands'

import './styles.css'

export default function Layouts() {
  const { layouts, setLayouts } = useData()
  const [localLayouts, setLocalLayouts] = useState<LayoutData[]>(layouts)

  useEffect(() => {
    setLocalLayouts(layouts)
  }, [layouts])

  function toggleDisabled(index: number) {
    setLocalLayouts(prev => 
      prev.map<LayoutData>((layout, i) => 
        i === index
          ? { ...layout, disabled: !layout.disabled }
          : layout
      )
    )
  }

  async function handleSave() {
    try {
      await saveLayouts(localLayouts)
      setLayouts(localLayouts)
    } catch (e) {
      console.error(e)
    }
  }

  return (
    <>
      <motion.div
        id="admin-layouts"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
      >
        <h1 className="heading">
          Set your <div>Pricing</div> options!
        </h1>

        <div className="layouts-container">
          {localLayouts.map((layout, idx) => (
            <LayoutOption
              key={idx}
              data={layout}
              onToggle={() => toggleDisabled(idx)}
            />
          ))}
        </div>
      </motion.div>

      {JSON.stringify(layouts) !== JSON.stringify(localLayouts) && (
        <div className="save-bar">
          You have unsaved changes!
          <button onClick={() => handleSave()} className="save-btn">Save</button>
        </div>
      )}
    </>
  )
}

function LayoutOption({
  data,
  onToggle
}: {
  data: LayoutData
  onToggle: () => void
}) {
  return (
    <button
      className="layout-option"
      data-disabled={data.disabled}
      onClick={onToggle}
    >
        <div className="layout-details">{`Layout ${data.kind}`}</div>
        <div className="layout-content">
          <img src={`/Layout ${data.kind.toString()}.png`} alt={data.kind.toString()} />
        </div>
        <button className='layout-btn'>{data.disabled ? "Disabled" : "Enabled"}</button>
    </button>
  )
}