// TODO: Prevent data race if user tried to carve another image while the previous one is still running

import init, { seam_carve } from '../../core/pkg/interface.js'

let available = false

init().then(() => {
  available = true
})

onmessage = e => {
  while (!available) {}
  
  const imageData = e.data[0]
  const imgWidth = e.data[1]
  const imgHeight = e.data[2]
  const sliderWidth = e.data[3]

  const arr = new Uint8ClampedArray(seam_carve(imageData, imgWidth, imgHeight, sliderWidth))
  const carvedImageData = new ImageData(arr, sliderWidth, imgHeight)

  postMessage(carvedImageData)
}
