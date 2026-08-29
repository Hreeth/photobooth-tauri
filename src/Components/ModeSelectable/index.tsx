import { useData } from '../../Contexts/DataContext'
import { Mode } from '../../types'

import './styles.css'

export default function ModeSelectable({ data, selected = false }: { data: Mode, selected?: boolean }) {
  const { setMode } = useData()

  return (
    <div className="text-selectable" data-selected={selected} onClick={() => setMode(data)}>
        <div className="selectable-value">{data.toUpperCase()}</div>
    </div>
  )
}