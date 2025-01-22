import './styles.css'
import { Mode, useData } from '../../Contexts/DataContext'

export default function ModeSelectable({ data, selected = false }: { data: Mode, selected?: boolean }) {
  const { setMode } = useData()

  function convert() {
    if (data == Mode.AUTOMATIC) return "AUTOMATIC"
    else return "MANUAL"
  }

  return (
    <div className="text-selectable" data-selected={selected} onClick={() => setMode(data)}>
        <div className="selectable-value">{convert()}</div>
    </div>
  )
}