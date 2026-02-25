import { Layout, Mode, Plan, useData } from '../../Contexts/DataContext'

import './styles.css'

export default function CopiesSelectable({
  data,
  selected = false
}: {
  data: Plan,
  selected?: boolean
}) {
  const { setOptions, mode, options } = useData()

  return (
    <div className="copy-selectable" data-selected={selected} onClick={() => setOptions(prev => ({ ...prev, copies: data.copies }))}>
      <div className="selectable-header">
        <div className="selectable-title">{data.title}</div>
        {data.popular && <div className="popular-tag">Popular</div>}
      </div>
      <div className="selectable-price">
        <div className="selectable-price-value">
          {mode == Mode.AUTOMATIC ?
            `₹${data.price}` :
            `${data.copies * (options.layout == Layout.C ? 2 : 1)} ${options.layout == Layout.C ? "strips" : (data.copies === 1 ? "copy" : "copies")}`}
        </div>
        {mode == Mode.AUTOMATIC && <div className="selectable-price-quantity">/ {data.copies * (options.layout == Layout.C ? 2 : 1)} {options.layout == Layout.C ? "strips" : (data.copies === 1 ? "copy" : "copies")}</div>}
      </div>
      <button className="select-btn">{selected ? "Selected" : "Select"}</button>
    </div>
  )
}
