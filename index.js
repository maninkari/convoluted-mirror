import "./app/css/styles.css"
import init, * as wasm from "./pkg/convoluted_mirror.js"
import mirrorwasm from "./pkg/convoluted_mirror_bg.wasm"

const WIDTH = 1020.0
const HEIGHT = 680.0

// let mirror = null
let mirrorConvolute = document.getElementById("mirrorConvolute")
let mirrorCanvas = document.getElementById("mirrorCanvas")
let video = document.getElementById("video")
let btnStart = document.getElementById("btnStart")

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
    // mirror.convolute_kernel_f1(
    //   mirrorConvolute.getContext("2d"),
    //   [
    //     1, 1, -4, -5, -4, 1, 1, 1, -5, 8, 14, 8, -5, 1, -4, 8, -6, -37, -6, 8, -4, -5, 14, -37, 100, -37, 14, -5, -4, 8,
    //     -6, -37, -6, 8, -4, 1, -5, 8, 14, 8, -5, 1, 1, 1, -4, -5, -4, 1, 1,
    //   ],
    //   7
    // )
    // mirror.convolute_kernel(
    //   mirrorConvolute.getContext("2d"),
    //   [
    //     -6, -8, 8, -12, 14, -12, 8, -8, -6, -8, 10, 3, 3, -22, 3, 3, 10, -8, 8, 3, -3, 30, 37, 30, -3, 3, 8, -12, 3, 30,
    //     -32, -61, -32, 30, 3, -12, 14, -22, 37, -61, 100, -61, 37, -22, 14, -12, 3, 30, -32, -61, -32, 30, 3, -12, 8, 3,
    //     -3, 30, 37, 30, -3, 3, 8, -8, 10, 3, 3, -22, 3, 3, 10, -8, -6, -8, 8, -12, 14, -12, 8, -8, -6,
    //   ],
    //   7
    // )
    requestAnimationFrame(animate)
  }
  requestAnimationFrame(animate)
})()
