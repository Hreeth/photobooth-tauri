import { useEffect, useState } from 'react'
import { motion } from 'framer-motion'

import { Config as ConfigType, useData } from '../../../Contexts/DataContext'
import { saveConfig } from '../../../Services/commands'
import { Addon, Plan } from '../../../types'

import EditIconSVG from '../../../assets/Images/edit.svg'

import './styles.css'

export default function Config() {
  const { config, setConfig } = useData()
  const [localConfig, setLocalConfig] = useState<ConfigType>(config)

  useEffect(() => {
    setLocalConfig(config)
  }, [config])

  async function handleSave() {
    try {
      setConfig(localConfig)
      await saveConfig(localConfig)
    } catch (e) {
      console.error(e)
    }
  }

  function setPopular(index: number) {
    setLocalConfig(prev => ({
      ...prev,
      plans: prev.plans.map((plan, i) => ({
        ...plan,
        popular: i === index,
      })),
    }))
  }

  function updatePlanField<T extends keyof Plan>(
    index: number,
    key: T,
    value: Plan[T]
  ) {
    setLocalConfig(prev => {
      const updated = [...prev.plans]

      updated[index] = {
        ...updated[index],
        [key]: value,
      }

      return {
        ...prev,
        plans: updated,
      }
    })
  }

  function toggleDigital() {
    setLocalConfig(prev => ({
      ...prev,
      digital: {
        ...prev.digital,
        enabled: !prev.digital.enabled,
      },
    }))
  }

  function updateDigitalPrice(price: number) {
    setLocalConfig(prev => ({
      ...prev,
      digital: {
        ...prev.digital,
        price,
      },
    }))
  }

  function updateDigitalTitle(title: string) {
    setLocalConfig(prev => ({
      ...prev,
      digital: {
        ...prev.digital,
        title,
      },
    }))
  }

  return (
    <>
      <motion.div
        id="admin-config"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
      >
        <h1 className="heading">
          Set your <div>Config</div> options!
        </h1>

        <div className="plans-container">
          {localConfig.plans.map((plan, idx) => (
            <PlanOption
              key={idx}
              data={plan}
              isPopular={plan.popular}
              onSetPopular={() => setPopular(idx)}
              onChange={(key, value) =>
                updatePlanField(idx, key, value)
              }
            />
          ))}
        </div>

        <AddonOption
          addon={localConfig.digital}
          onToggle={toggleDigital}
          onPriceChange={updateDigitalPrice}
          onTitleChange={updateDigitalTitle}
        />
      </motion.div>

      {JSON.stringify(config) !== JSON.stringify(localConfig) && (
        <div className="save-bar">
          You have unsaved changes!

          <button
            onClick={handleSave}
            className="save-btn"
          >
            Save
          </button>
        </div>
      )}
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
  onChange: <T extends keyof Plan>(
    key: T,
    value: Plan[T]
  ) => void
}) {
  const [editing, setEditing] = useState({
    title: false,
    price: false,
  })

  function toggle(field: keyof typeof editing) {
    setEditing(e => ({
      ...e,
      [field]: !e[field],
    }))
  }

  return (
    <div
      className="plan-option"
      data-selected={isPopular}
    >
      <div className="plan-header">
        {editing.title ? (
          <input
            className="title-input"
            value={data.title}
            onChange={e =>
              onChange('title', e.target.value)
            }
            onBlur={() => toggle('title')}
            autoFocus
          />
        ) : (
          <>
            <div className="plan-title">
              {data.title}
            </div>

            <EditIcon
              onClick={() => toggle('title')}
            />
          </>
        )}
      </div>

      <div className="plan-price">
        <div className="plan-price-value">
          ₹

          {editing.price ? (
            <input
              type="number"
              className="price-input"
              value={data.price}
              onChange={e =>
                onChange(
                  'price',
                  Number(e.target.value)
                )
              }
              onBlur={() => toggle('price')}
              autoFocus
            />
          ) : (
            <>
              <span>{data.price}</span>

              <EditIcon
                onClick={() => toggle('price')}
              />
            </>
          )}
        </div>

        <div className="plan-price-quantity">
          / {data.copies}{' '}
          {data.copies === 1 ? 'copy' : 'copies'}
        </div>
      </div>

      <button
        className="select-btn"
        onClick={onSetPopular}
      >
        {isPopular ? 'Popular' : 'Set as popular'}
      </button>
    </div>
  )
}

function AddonOption({
  addon,
  onToggle,
  onPriceChange,
  onTitleChange,
}: {
  addon: Addon
  onToggle: () => void
  onPriceChange: (price: number) => void
  onTitleChange: (title: string) => void
}) {
  const [editingTitle, setEditingTitle] = useState(false)
  const [editingPrice, setEditingPrice] = useState(false)

  return (
    <div
      className="additional-container"
      data-selected={addon.enabled}
    >
      <div className="additional-grp-2">
        <div className="field-header">
          {editingTitle ? (
            <input
              className="title-input"
              value={addon.title}
              onChange={e =>
                onTitleChange(e.target.value)
              }
              onBlur={() => setEditingTitle(false)}
              autoFocus
            />
          ) : (
            <>
              <div className="additional-title">
                {addon.title}
              </div>

              <EditIcon
                onClick={() => setEditingTitle(true)}
              />
            </>
          )}
        </div>

        {addon.enabled && (
          <div className="plan-price-value">
            ₹

            {editingPrice ? (
              <input
                type="number"
                className="price-input"
                value={addon.price}
                onChange={e =>
                  onPriceChange(Number(e.target.value))
                }
                onBlur={() => setEditingPrice(false)}
                autoFocus
              />
            ) : (
              <>
                <span>{addon.price}</span>

                <EditIcon
                  onClick={() => setEditingPrice(true)}
                />
              </>
            )}
          </div>
        )}
      </div>

      <div className="additional-grp-1">
        <div
          className="add-btn"
          onClick={onToggle}
        >
          {addon.enabled ? 'Enabled' : 'Disabled'}
        </div>
      </div>
    </div>
  )
}

function EditIcon({
  onClick,
}: {
  onClick: () => void
}) {
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