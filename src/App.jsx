import { useState, useEffect, useRef } from 'react'

import init, { seam_carve } from '../core/pkg/core.js'

function App() {
  let [coreAvailable, setCoreAvailable] = useState(false)
  
  useEffect(() => {
    init().then(() => {
      setCoreAvailable(true)
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
    setImgWidth(e.target.naturalWidth)
    setImgHeight(e.target.naturalHeight)

    setSliderWidth(e.target.naturalWidth)
    setSliderHeight(e.target.naturalHeight)
  }

  const [sliderWidth, setSliderWidth] = useState(0)
  const [sliderHeight, setSliderHeight] = useState(0)

  const img = useRef(null)

  const carvedCanvasRef = useRef(null)
  const [carvedImageBitmap, setCarvedImageBitmap] = useState(null)

  useEffect(() => {
    if (carvedCanvasRef.current) {
      carvedCanvasRef.current.width = carvedImageBitmap.width
      carvedCanvasRef.current.height = carvedImageBitmap.height

      const ctx = carvedCanvasRef.current.getContext('2d')
      ctx.drawImage(carvedImageBitmap, 0, 0, carvedImageBitmap.width, carvedImageBitmap.height)
    }
  }, [carvedImageBitmap])

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
            <img src={fileSrc} onLoad={handleImageLoad} style={{ maxHeight: "100vh" }} ref={img} />
          </figure>

          <h2>Seam Target</h2>

          <div className="grid">
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

          <input type="submit" value="Carve" onClick={async () => {
            while (!coreAvailable) {}

            const canvas = new OffscreenCanvas(imgWidth, imgHeight)
            const context = canvas.getContext('2d')
            context.drawImage(img.current, 0, 0, imgWidth, imgHeight)

            const imageData = context.getImageData(0, 0, imgWidth, imgHeight).data

            const arr =
              new Uint8ClampedArray(seam_carve(imageData, imgWidth, imgHeight, sliderWidth, sliderHeight))
            
            const carvedImageData = new ImageData(arr, imgHeight, imgWidth)

            setCarvedImageBitmap(await createImageBitmap(
              carvedImageData,
              0,
              0,
              carvedImageData.width,
              carvedImageData.height
            ))
          }} />

          {
            carvedImageBitmap !== null &&
            <>
              <hr />
              <canvas ref={carvedCanvasRef}></canvas>
            </>
          }
        </>
      }
    </main>
  </>
}

export default App
