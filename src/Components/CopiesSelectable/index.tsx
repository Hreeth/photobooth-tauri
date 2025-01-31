import { useData } from '../../Contexts/DataContext'

import './styles.css'

interface CopiesData {
  title: string,
  price: number,
  strips: number,
  popular: boolean
}

export default function CopiesSelectable({
  data,
  selected = false
}: {
  data: CopiesData,
  selected?: boolean
}) {
  const { setOptions } = useData()

  return (
    <div className="copy-selectable" data-selected={selected} onClick={() => setOptions(prev => ({ ...prev, copies: data.strips }))}>
        <div className="selectable-header">
            <div className="selectable-title">{data.title}</div>
            {data.popular && <div className="popular-tag">Popular</div>}
        </div>
        <div className="selectable-price">
            <div className="selectable-price-value">₹{data.price}</div>
            <div className="selectable-price-quantity">/ {data.strips} strips</div>
        </div>
        <button className="select-btn">{selected ? "Selected" : "Select"}</button>
    </div>
  )
}
