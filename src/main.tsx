import React, { lazy } from 'react'
import ReactDOM from 'react-dom/client'
import { ConfigProvider, theme } from 'antd'
import { appWindow } from '@tauri-apps/api/window'
import { Navigate, RouterProvider, createBrowserRouter } from 'react-router-dom'
import '~/styles/index.scss'

const Chat = lazy(async () => await import('~/components/Chat'))

const router = createBrowserRouter([
  {
    path: '/',
    element: <Navigate to="/1?name=free" />,
  },
  {
    path: '/:chatID',
    element: (
      <React.Suspense fallback={null}>
        <Chat />
      </React.Suspense>
    ),
  },
])

appWindow.theme().then((v) => {
  ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
      <ConfigProvider
        theme={{
          algorithm: theme.defaultAlgorithm,
        }}
      >
        <RouterProvider router={router} />
      </ConfigProvider>
    </React.StrictMode>
  )
})
