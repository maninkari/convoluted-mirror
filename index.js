import "./app/css/styles.css"
import init, * as wasm from "./pkg/convoluted_mirror.js"
import mirrorwasm from "./pkg/convoluted_mirror_bg.wasm"

// const WIDTH = 1020.0
// const HEIGHT = 680.0

const WIDTH = window.innerWidth
const HEIGHT = window.innerHeight

// let mirror = null
let mirrorConvolute = document.getElementById("mirrorConvolute")
let mirrorCanvas = document.getElementById("mirrorCanvas")
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

  let i = 0
  async function animate() {
    // draw frame coming from the video stream
    mirrorCanvas.getContext("2d").drawImage(video, 0, 0)
    // draw conlute reflection on the mirrorConvolute canvas
    mirror.convolute_kernel(mirrorConvolute.getContext("2d"), [3, 14, 3, 14, 100, 14, 3, 14, 3], 7)
    requestAnimationFrame(animate)
  }
  requestAnimationFrame(animate)
})()
