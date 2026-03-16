import { motion } from 'framer-motion'
import { useEffect, useState } from 'react'

import { useData } from '../../../Contexts/DataContext'
import { savePages } from '../../../Services/commands'

import './styles.css'

export default function Pages() {
  const { pages, setPages } = useData()
  const [localPages, setLocalPages] = useState<number>(pages)

  useEffect(() => {
    setLocalPages(pages)
  }, [pages])

  async function handleSave() {
    try {
      await savePages(localPages)
      setPages(localPages)
    } catch (e) {
      console.error(e)
    }
  }

  return (
    <>
      <motion.div
        id="admin-pages"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
      >
        <h1 className="heading">
          Set your <div>Page</div> count
        </h1>

        <div className="pages-container">
          <input
            type="number"
            value={localPages}
            onChange={(e) => setLocalPages(Number(e.target.value))}
            className="page-number-input"
          />
        </div>
      </motion.div>

      {pages !== localPages && (
        <div className="save-bar">
          You have unsaved changes!
          <button onClick={handleSave} className="save-btn">Save</button>
        </div>
      )}
    </>
  )
}