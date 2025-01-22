import './styles.css'

export default function Header({
  backCallback = () => { return },
}: {
  backCallback?: Function,
}) {
  return (
    <div id="header">
      <button className="back-btn" onClick={() => backCallback()}>Back</button>
    </div>
  )
}
