import { useState, useEffect, useRef } from 'react'

import init from '../wasm/pkg/wasm.js'

function App() {
  let wasmAvailable = false
  
  useEffect(() => {
    init().then(() => {
      wasmAvailable = true
    })
  }, [])

  const [fileSrc, setFileSrc] = useState(null)

  function handleFileChange(e) {
    let url = URL.createObjectURL(fileRef.current.files[0])
    setFileSrc(url)
  }

  const fileRef = useRef(null)

  const [imgWidth, setImgWidth] = useState(0)
  const [imgHeight, setImgHeight] = useState(0)

  function handleImageLoad(e) {
    console.log('image load')
    
    setImgWidth(e.target.naturalWidth)
    setImgHeight(e.target.naturalHeight)

    setSliderWidth(e.target.naturalWidth)
    setSliderHeight(e.target.naturalHeight)
  }

  const [sliderWidth, setSliderWidth] = useState(0)
  const [sliderHeight, setSliderHeight] = useState(0)

  return <>
    <header className="container">
      <h1>Seam Carving</h1>
    </header>
    <main className="container">
      <input
        type="file"
        id="file"
        name="file"
        accept="image/png, image/jpeg"
        onChange={handleFileChange}
        ref={fileRef}
      />

      {
        fileSrc !== null &&
        <>
          <figure>
            <img src={fileSrc} onLoad={handleImageLoad} style={{ maxHeight: "100vh" }} />
          </figure>

          <h2>Seam Target</h2>

          <div class="grid">
            <div>
              <label htmlFor="width-slider">
                Width ({sliderWidth}px)
                <input
                  type="range"
                  min="0"
                  max={imgWidth}
                  value={sliderWidth}
                  name="width-slider"
                  onChange={e => setSliderWidth(e.target.value)}
                />
              </label>
            </div>
          
            <div>
              <label htmlFor="height-slider">
                Height ({sliderHeight}px)
                <input
                  type="range"
                  min="0"
                  max={imgHeight}
                  value={sliderHeight}
                  name="height-slider"
                  onChange={e => setSliderHeight(e.target.value)}
                />
              </label>
            </div>
          </div>

          <input type="submit" value="Carve" />
        </>
      }
    </main>
  </>
}

export default App
