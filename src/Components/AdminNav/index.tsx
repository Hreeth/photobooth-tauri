import { NavLink } from 'react-router-dom'

import './styles.css'

export default function AdminNav() {
  return (
    <nav id="admin-nav">
        <NavLink
            to="/admin/mode"
            className="admin-nav-link"
            style={({ isActive }) => ({ pointerEvents: isActive ? 'none' : 'auto' })}
        >
          Mode
        </NavLink>
        <NavLink
            to="/admin/pricing"
            className="admin-nav-link"
            style={({ isActive }) => ({ pointerEvents: isActive ? 'none' : 'auto' })}
        >
          Pricing
        </NavLink>
    </nav>
  )
}
