import { useData } from '../../Contexts/DataContext'
import { Filter } from '../../types'

import './styles.css'

export default function FilterSelectable({ data, selected = false }: { data: Filter, selected?: boolean }) {
  const { setOptions } = useData()

  return (
    <div className="text-selectable" data-selected={selected} onClick={() => setOptions(prev => ({ ...prev, filter: data }))}>
        <div className="selectable-value">{data.toUpperCase()}</div>
    </div>
  )
}