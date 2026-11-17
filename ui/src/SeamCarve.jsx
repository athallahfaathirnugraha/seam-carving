import { useState, useEffect, useRef } from 'react'

import init, { seam_carve } from '../../core/pkg/interface.js'

const worker = new Worker(new URL('./worker.js', import.meta.url), {
  type: 'module'
})

function SeamCarve() {
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
  }

  const [sliderWidth, setSliderWidth] = useState(0)

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

  const [isLoading, setIsLoading] = useState(false)

  return <>
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

        <div>
          <label htmlFor="width-slider">
            Target Width
            <input
              type="number"
              min="0"
              max={imgWidth}
              value={sliderWidth}
              name="width-slider"
              onChange={e => setSliderWidth(e.target.value)}
            />
          </label>
        </div>

        <input type="submit" value="Carve" onClick={async () => {
          const canvas = new OffscreenCanvas(imgWidth, imgHeight)
          const context = canvas.getContext('2d')
          context.drawImage(img.current, 0, 0, imgWidth, imgHeight)

          const imageData = context.getImageData(0, 0, imgWidth, imgHeight).data

          setIsLoading(true)

          worker.postMessage([imageData, imgWidth, imgHeight, sliderWidth])

          worker.onmessage = async e => {
            const carvedImageData = e.data
            
            setCarvedImageBitmap(await createImageBitmap(
              carvedImageData,
              0,
              0,
              carvedImageData.width,
              carvedImageData.height
            ))

            setIsLoading(false)
          }
        }} />

        {
          isLoading &&
          <progress></progress>
        }

        {
          carvedImageBitmap !== null &&
          <>
            <hr />
            <h2>Carved Image</h2>
            <figure>
              <canvas style={{ display: 'block' }} ref={carvedCanvasRef}></canvas>
            </figure>
            <button style={{ width: '100%' }} onClick={() => {
              carvedCanvasRef.current.toBlob(blob => {
                const url = URL.createObjectURL(blob)

                const link = document.createElement('a')
                link.href = url
                link.download = 'carved.png'
                document.body.appendChild(link)
                link.click()
                document.body.removeChild(link)

                URL.revokeObjectURL(url)
              })
            }}>Download</button>
          </>
        }
      </>
    }
  </>
}

export default SeamCarve
