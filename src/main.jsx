import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.jsx'

import '@picocss/pico'

createRoot(document.querySelector('body')).render(
  <StrictMode>
    <main className="container">
      <App />
    </main>
  </StrictMode>,
)
