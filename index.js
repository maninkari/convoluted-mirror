import "./app/css/styles.css"
import init, * as wasm from "./pkg/convoluted_mirror.js"
import mirrorwasm from "./pkg/convoluted_mirror_bg.wasm"

const WIDTH = 720.0
const HEIGHT = 480.0

let mirrorCanvas = document.getElementById("mirrorCanvas")
let mirrorConvolute = document.getElementById("mirrorConvolute")

// setup and play video
;(async () => {
  await init(mirrorwasm)

  const stream = await navigator.mediaDevices.getUserMedia({
    audio: false,
    video: {
      facingMode: "user",
      width: WIDTH,
      height: HEIGHT,
    },
  })

  video.srcObject = stream
  await video.play()

  const mirror = new wasm.Mirror(mirrorCanvas.getContext("2d"), WIDTH, HEIGHT)

  let i = 0
  async function animate() {
    // draw frame from video stream
    mirrorCanvas.getContext("2d").drawImage(video, 0, 0)
    // draw convolute reflection on the mirrorConvolute canvas
    mirror.convolute(mirrorConvolute.getContext("2d"))
    requestAnimationFrame(animate)
  }
  requestAnimationFrame(animate)
})()
