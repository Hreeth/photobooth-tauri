import { motion } from 'framer-motion'
import { Outlet, useNavigate } from 'react-router-dom'

import AdminNav from '../../Components/AdminNav'
import Header from '../../Components/Header'

import './styles.css'

export default function Admin() {
  const navigate = useNavigate()

  return (
      <motion.div
        id='admin'
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
      >
        <Header backCallback={() => navigate('/')} />
        <AdminNav />
        <div className="admin-container">
          <Outlet />
        </div>
      </motion.div>
  )
}
