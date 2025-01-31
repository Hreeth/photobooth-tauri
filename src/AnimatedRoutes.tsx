import React, { Suspense } from 'react'
import { AnimatePresence } from 'framer-motion'
import { Route, Routes, useLocation } from 'react-router-dom'

import DataProvider from './Contexts/DataContext'

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
