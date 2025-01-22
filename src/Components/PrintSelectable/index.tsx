import { Print, useData } from '../../Contexts/DataContext'

import './styles.css'

export default function PrintSelectable({ data, selected = false }: { data: Print, selected?: boolean }) {
  const { setOptions } = useData()

  function convert() {
    if (data == Print.COLOR) return "COLOR"
    else return "B&W"
  }

  return (
    <div className="text-selectable" data-selected={selected} onClick={() => setOptions(prev => ({ ...prev, print: data }))}>
        <div className="selectable-value">{convert()}</div>
    </div>
  )
}