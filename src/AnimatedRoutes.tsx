import React, { Suspense, useEffect } from 'react'
import { AnimatePresence } from 'framer-motion'
import { Route, Routes, useLocation, useNavigate } from 'react-router-dom'

import DataProvider, { useData } from './Contexts/DataContext'
import reset from './Utils/reset'

const Home = React.lazy(() => import('./Pages/Home'))
const Mail = React.lazy(() => import('./Pages/Mail'))
const Greeting = React.lazy(() => import('./Pages/Greeting'))
const Mode = React.lazy(() => import('./Pages/Mode'))
const Countdown = React.lazy(() => import('./Pages/Countdown'))
const Passcode = React.lazy(() => import('./Pages/Passcode'))
const Copies = React.lazy(() => import('./Pages/Form/Copies'))
const Print = React.lazy(() => import('./Pages/Form/Print'))
const Payment = React.lazy(() => import('./Pages/Form/Payment'))

export default function AnimatedRoutes() {
    const location = useLocation()

    return (
        <Suspense fallback={<div>Loading...</div>}>
            <AnimatePresence>
                <DataProvider>
                    <RedirectAfterTimeout />
                    
                    <Routes location={location} key={location.pathname}>
                        <Route path='/' element={<Home />} />
                        <Route path='/mail' element={<Mail />} />
                        <Route path='/greeting' element={<Greeting />} />
                        <Route path='/passcode' element={<Passcode />} />
                        <Route path='/mode' element={<Mode />} />
                        <Route path='/countdown' element={<Countdown />} />
                        <Route path='/copies' element={<Copies />} />
                        <Route path='/print' element={<Print />} />
                        <Route path='/payment' element={<Payment />} />
                    </Routes>
                </DataProvider>
            </AnimatePresence>
        </Suspense>
    )
}

function RedirectAfterTimeout() {
    const navigate = useNavigate()
    const location = useLocation()

    const { setOptions, setImages } = useData()

    useEffect(() => {
        if (location.pathname == "/") return;

        const timeout = setTimeout(() => {
            reset(setOptions, setImages, navigate)
        }, 4 * 60 * 1000);

        return () => clearTimeout(timeout)
    }, [navigate])
    
    return null;
}