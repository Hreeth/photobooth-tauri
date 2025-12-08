import { motion } from 'framer-motion'
import { useEffect, useState } from 'react'

import { Plan, useData } from '../../../Contexts/DataContext'
import { savePricing } from '../../../Services/pricing'

import EditIconSVG from '../../../assets/Images/edit.svg'

import './styles.css'

export default function Pricing() {
  const { plans, setPlans } = useData()
  const [localPlans, setLocalPlans] = useState<Plan[]>(plans)

  function setPopular(index: number) {
    setLocalPlans(prev =>
      prev.map((p, i) => ({
        ...p,
        popular: i === index
      }))
    )
  }

  useEffect(() => {
    setLocalPlans(plans)
  }, [plans])

  async function handleSave() {
    try {
      await savePricing(localPlans)
      setPlans(localPlans)
    } catch (e) {
      console.error(e)
    }
  }

  function updatePlanField<T extends keyof Plan>(
    index: number,
    key: T,
    value: Plan[T]
  ) {
    setLocalPlans(prev => {
      const updated = [...prev]
      updated[index] = { ...updated[index], [key]: value }
      return updated
    })
  }

  return (
    <>
      <motion.div
        id="admin-pricing"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
      >
        <h1 className="heading">
          Set your <div>Pricing</div> options!
        </h1>

        <div className="plans-container">
          {localPlans.map((plan, idx) => (
            <PlanOption
              key={idx}
              data={plan}
              isPopular={plan.popular}
              onSetPopular={() => setPopular(idx)}
              onChange={(key, value) => updatePlanField(idx, key, value)}
            />
          ))}
        </div>
      </motion.div>

      {JSON.stringify(plans) !== JSON.stringify(localPlans) && <div className="save-bar">
        You have unsaved changes!
        <button onClick={() => handleSave()} className="save-btn">Save</button>
      </div>}
    </>
  )
}

function PlanOption({
  data,
  isPopular,
  onSetPopular,
  onChange,
}: {
  data: Plan
  isPopular: boolean
  onSetPopular: () => void
  onChange: <T extends keyof Plan>(key: T, value: Plan[T]) => void
}) {
  const [editing, setEditing] = useState({
    title: false,
    price: false,
    strips: false,
  })

  function toggle(field: keyof typeof editing) {
    setEditing(e => ({ ...e, [field]: !e[field] }))
  }

  return (
    <div className="plan-option" data-selected={isPopular}>

      {/* TITLE */}
      <div className="plan-header">
        {editing.title ? (
          <input
            className='title-input'
            value={data.title}
            onChange={e => onChange("title", e.target.value)}
            onBlur={() => toggle("title")}
            autoFocus
          />
        ) : (
          <>
            <div className="plan-title">{data.title}</div>
            <EditIcon onClick={() => toggle("title")} />
          </>
        )}
      </div>

      {/* PRICE */}
      <div className="plan-price">
        <div className="plan-price-value">
          ₹
          {editing.price ? (
            <input
              type="number"
              className='price-input'
              value={data.price}
              onChange={e => onChange("price", Number(e.target.value))}
              onBlur={() => toggle("price")}
              autoFocus
            />
          ) : (
            <>
              <span>{data.price}</span>
              <EditIcon onClick={() => toggle("price")} />
            </>
          )}
        </div>

        {/* STRIPS */}
        <div className="plan-price-quantity">
          / {data.strips} strips
        </div>
      </div>

      {/* POPULAR */}
      <button
        className="select-btn"
        onClick={onSetPopular}
      >
        {isPopular ? "Popular" : "Set as popular"}
      </button>

    </div>
  )
}

/* PURE CSS / Unicode ICON */
function EditIcon({ onClick }: { onClick: () => void }) {
  return (
    <img
      src={EditIconSVG}
      className="edit-icon"
      alt="edit"
      onClick={onClick}
      draggable={false}
    />
  )
}