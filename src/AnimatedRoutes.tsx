import React, { Suspense, useEffect } from 'react'
import { AnimatePresence } from 'framer-motion'
import { Route, Routes, useLocation, useNavigate } from 'react-router-dom'

import DataProvider, { useData } from './Contexts/DataContext'
import reset from './Utils/reset'

const Home = React.lazy(() => import('./Pages/Home'))
const Mail = React.lazy(() => import('./Pages/Mail'))
const Greeting = React.lazy(() => import('./Pages/Greeting'))
const Admin = React.lazy(() => import('./Pages/Admin'))
const AdminMode = React.lazy(() => import('./Pages/Admin/Mode'))
const AdminConfig = React.lazy(() => import('./Pages/Admin/Config'))
const AdminLayouts = React.lazy(() => import('./Pages/Admin/Layouts'))
const AdminPages = React.lazy(() => import('./Pages/Admin/Pages'))
const Camera = React.lazy(() => import('./Pages/Camera'))
const Passcode = React.lazy(() => import('./Pages/Passcode'))
const Layout = React.lazy(() => import('./Pages/Form/Layout'))
const Copies = React.lazy(() => import('./Pages/Form/Copies'))
const Print = React.lazy(() => import('./Pages/Form/Filter'))
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
                        <Route path='/admin' element={<Admin />}>
                            <Route path='mode' element={<AdminMode />} />
                            <Route path='config' element={<AdminConfig />} />
                            <Route path='layouts' element={<AdminLayouts />} />
                            <Route path='pages' element={<AdminPages />} />
                        </Route>
                        <Route path='/Camera' element={<Camera />} />
                        <Route path='/layout' element={<Layout />} />
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

    const { setOptions } = useData()

    useEffect(() => {
        if (location.pathname == "/") return;

        let timeout: NodeJS.Timeout

        const resetTimeout = () => {
            clearTimeout(timeout)

            timeout = setTimeout(() => {
                reset(setOptions, navigate)
            }, 4 * 60 * 1000);
        }

        const events = ["pointerdown", "keydown"]
        events.forEach((ev) => document.addEventListener(ev, resetTimeout))

        resetTimeout()

        return () => {
            clearTimeout(timeout)

            events.forEach((ev) => document.removeEventListener(ev, resetTimeout))
        }
    }, [location.pathname, navigate, setOptions])
    
    return null;
}