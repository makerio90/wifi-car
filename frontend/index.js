window.onload = function () {
let enable  = document.getElementById("enable")
let disable = document.getElementById("disable")
let fwd     = document.getElementById("fwd")
let bkwd    = document.getElementById("bkwd")
let left    = document.getElementById("left")
let right   = document.getElementById("right")

enable.onclick = () => {
  fetch("/api/enable", {
    method: "post"
  })
}

disable.onclick = () => {
  fetch("/api/disable", {
    method: "post"
  })
}

fwd.onmousedown   = () => drive( 1, 0)
bkwd.onmousedown  = () => drive(-1, 0)
left.onmousedown  = () => drive( 0,-1)
right.onmousedown = () => drive( 0, 1)

fwd.onmouseup     = () => drive(0, 0)
bkwd.onmouseup    = () => drive(0, 0)
left.onmouseup    = () => drive(0,0)
right.onmouseup   = () => drive(0,0)
}
function drive(accelerate,steer) {
  let parems = new URLSearchParams({accelerate,steer})
  fetch("/api/drive?" + parems.toString(), {
    method: "post"
  })
}
