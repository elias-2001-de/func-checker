const { greet, check_funcs } = wasm_bindgen;

async function main() {
  await wasm_bindgen("./rust_bg.wasm");

  // console.log(greet("javasript"));

  var app = Elm.Main.init({
    node: document.getElementById("elmDiv"),
  });

  app.ports.setFunc.subscribe(function (message) {
    let result = check_funcs(message);
    if (result[0] == null) {
      result[0] = "Correct";
    }
    console.log(result);
    app.ports.getStatus.send(result);
  });
}