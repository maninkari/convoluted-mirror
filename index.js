import "./app/css/styles.css"
import init, * as wasm from "./pkg/convoluted_mirror.js"
import mirrorwasm from "./pkg/convoluted_mirror_bg.wasm"

const WIDTH = 1100.0
const HEIGHT = 600.0

// let mirror = null
let mirrorConvolute = document.getElementById("mirrorConvolute")
let mirrorCanvas = document.getElementById("mirrorCanvas")
const mirrorCanvasCtx = mirrorCanvas.getContext("2d", { willReadFrequently: true })
const mirrorConvoluteCtx = mirrorConvolute.getContext("2d", { willReadFrequently: true })
let video = document.getElementById("video")
let btnStart = document.getElementById("btnStart")
let btnFullscreen = document.getElementById("btnFullscreen")

mirrorConvolute.width = WIDTH
mirrorConvolute.height = HEIGHT
mirrorCanvas.width = WIDTH
mirrorCanvas.height = HEIGHT
video.width = WIDTH
video.height = HEIGHT

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

  const mirror = new wasm.Mirror(mirrorCanvas, WIDTH, HEIGHT)
  console.log(mirror.talk())

  btnStart.addEventListener("click", async () => {
    console.log("start")
    mirror.convolute(mirrorConvolute.getContext("2d"))
  })

  btnFullscreen.addEventListener("click", () => {
    if (mirrorCanvas.requestFullscreen) {
      mirrorCanvas.requestFullscreen()
    } else if (mirrorCanvas.mozRequestFullScreen) {
      // For Firefox
      mirrorCanvas.mozRequestFullScreen()
    } else if (mirrorCanvas.webkitRequestFullscreen) {
      // For Chrome and Safari
      mirrorCanvas.webkitRequestFullscreen()
    } else if (mirrorCanvas.msRequestFullscreen) {
      // For IE/Edge
      mirrorCanvas.msRequestFullscreen()
    }
  })

  document.addEventListener("fullscreenchange", () => {
    if (!document.fullscreenElement) {
      console.log("exiting full screen")
    }
  })

  let i = 0
  const WINDOW_SIZE = 7 // adjust as needed
  async function animate() {
    // draw frame coming from the video stream
    mirrorCanvasCtx.drawImage(video, 0, 0)
    // draw conlute reflection on the mirrorConvolute canvas
    await mirror.convolute_kernel_windowed(mirrorConvoluteCtx, [3, 14, 3, 14, 100, 14, 3, 14, 3], 7, WINDOW_SIZE)
    requestAnimationFrame(animate)
  }
  requestAnimationFrame(animate)
})()
