import { useState, useEffect } from 'react'

import init from '../wasm/pkg/wasm.js'

function App() {
  let wasmAvailable = false
  
  useEffect(() => {
    init().then(() => {
      wasmAvailable = true
    })
  }, [])

  return <p>hi</p>
}

export default App
