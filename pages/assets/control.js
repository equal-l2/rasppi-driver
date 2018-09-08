// send a request for operation
function req(op) {
  fetch(`/driver`, {
    method: "PUT",
    cache: "no-store",
    credentials: "same-origin",
    body: JSON.stringify({"state":op})
  }).then((resp) => {
      if(resp.ok) { // if the request is successful
        console.log(`SUCCESS: ${op}`);
      } else {
        throw new Error(`${resp.status} ${resp.statusText}`);
      }
    })
    .catch((err) => {
      // print error to console
      console.log(`ERROR: ${err.message}`);
    });
}
