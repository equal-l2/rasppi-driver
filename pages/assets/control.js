// send a request for operation
function req(op) {
  fetch(`/driver/${op}`)
    .then((resp) => {
      if(resp.ok) { // if the request is successful
        console.log(`SUCCESS: ${op}`);
        let spanStatus = document.getElementById("status");
        spanStatus.innerHTML = op[0].toUpperCase() + op.slice(1);
      } else {
        throw new Error(`${resp.status} ${resp.statusText}`);
      }
    })
    .catch((err) => {
      // print error to console
      console.log(`ERROR: ${err.message}`);
    });
}

function setOpListener(op) {
  document.getElementById(op).addEventListener(
    "click",
    () => { req(op); }
  );
}

window.addEventListener("load", () => {
  setOpListener("forward");
  setOpListener("backward");
  setOpListener("left");
  setOpListener("right");
  setOpListener("stop");
});
