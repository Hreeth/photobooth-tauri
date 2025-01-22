import './styles.css'

export default function Footer({
  backCallback = () => { return },
  continueCallback = () => { return },
  disabled = false
}: {
  backCallback?: Function,
  continueCallback?: Function,
  disabled?: boolean
}) {
  return (
    <div id="footer">
      <button className="back-btn" onClick={() => backCallback()}>Back</button>
      <button className="continue-btn" onClick={() => continueCallback()} disabled={disabled}>Continue</button>
    </div>
  )
}
