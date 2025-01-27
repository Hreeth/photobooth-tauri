import { invoke } from "@tauri-apps/api/core";
import { useCallback, useRef, useState } from "react";

interface QrResponse {
    id: string,
    image_url: string,
    close_by: number
}

export default function usePayment() {
    const [qrCode, setQrCode] = useState<QrResponse | undefined>(undefined)
    const [loading, setLoading] = useState<boolean>(false)
    const [error, setError] = useState<string | null>(null)
    const [paid, setPaid] = useState<boolean | null>(null)

    const pollingIntervalRef = useRef<NodeJS.Timeout | null>(null)

    const fetchQrCode = useCallback(async (amt: number) => {
        setLoading(true)
        setError(null)

        try {
            const res = await invoke<QrResponse>('create_qr', { amount: amt, closeBySecs: 180 })
            setQrCode(res)
            setPaid(null)
            pollPaymentStatus(res)
            return res
        } catch (err) {
            console.error("Error fetching QR Code:", err)
            setError("Failed to fetch QR Code")
            return undefined
        } finally {
            setLoading(false)
        }
    }, [])

    const pollPaymentStatus = useCallback((qr: QrResponse) => {
        if (pollingIntervalRef.current) clearInterval(pollingIntervalRef.current)

        const interval = setInterval(async () => {
            try {
                const currentTime = Math.floor(Date.now() / 1000)
                if (currentTime >= qr.close_by) {
                    console.error("QR Code expired")
                    setPaid(false)
                    clearInterval(interval)
                    return
                }

                const res = await invoke<boolean>("check_payment_status", { qrCodeId: qr.id })
                if (res) {
                    console.log("Payment successful")
                    setPaid(true)
                    clearInterval(interval)
                }
            } catch (err) {
                console.error("Failed to check payment status:", err)
                clearInterval(interval)
            }
        }, 2000);

        pollingIntervalRef.current = interval
    }, [])

    return { qrCode, loading, error, fetchQrCode, paid, pollingIntervalRef }
}