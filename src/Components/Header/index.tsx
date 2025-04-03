import './styles.css'

export default function Header({
  backCallback = () => { return },
}: {
  backCallback?: () => void,
}) {
  return (
    <div id="header">
      <button className="back-btn" onClick={() => backCallback()}>Back</button>
    </div>
  )
}
