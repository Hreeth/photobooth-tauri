import { LayoutData, useData } from '../../Contexts/DataContext'

import './styles.css'

export default function LayoutSelectable({
  data,
  selected = false
}: {
  data: LayoutData,
  selected?: boolean
}) {
  const { setOptions } = useData()

  return (
    <button
      className="layout-selectable"
      data-selected={selected && !data.disabled}
      disabled={data.disabled}
      onClick={() => setOptions(prev => ({ ...prev, layout: data.kind }))}
    >
        <div className="selectable-content">
          <img src={`/Layout ${data.kind.toString()}.png`} alt={data.kind.toString()} />
        </div>
        <div className="selectable-details">{`Layout ${data.kind}`}</div>
    </button>
  )
}
