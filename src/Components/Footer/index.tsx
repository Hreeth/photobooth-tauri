import './styles.css'

export default function Footer({
  backCallback = () => {},
  continueCallback = () => {},
  continueText = "Continue",
  disabled = false
}: {
  backCallback?: () => void,
  continueCallback?: () => void,
  continueText?: String
  disabled?: boolean
}) {
  return (
    <div id="footer">
      <button className="back-btn" onClick={() => backCallback()}>Back</button>
      <button className="continue-btn" onClick={() => continueCallback()} disabled={disabled}>{continueText}</button>
    </div>
  )
}
