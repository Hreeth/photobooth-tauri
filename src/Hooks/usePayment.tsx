import { invoke } from "@tauri-apps/api/core";
import { useCallback, useState } from "react";

export default function usePayment() {
    const [qrCodeUrl, setQrCodeUrl] = useState<string | null>(null)
    const [loading, setLoading] = useState<boolean>(false)
    const [error, setError] = useState<string | null>(null)

    const fetchQrCode = useCallback(async (amount: number, receipt: string) => {
        setLoading(true)
        setError(null)

        try {
            const url = await invoke<string>('create_order', { amount, receipt })
            setQrCodeUrl(url)
            return url
        } catch (err) {
            console.error("Error fetching QR Code:", err)
            setError("Failed to fetch QR Code")
            return null
        } finally {
            setLoading(false)
        }
    }, [])

    return { qrCodeUrl, loading, error, fetchQrCode }
}