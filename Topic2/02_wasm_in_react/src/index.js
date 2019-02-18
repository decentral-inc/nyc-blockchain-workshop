import React from "react";
import ReactDOM from "react-dom";
//import * as wasm from "@hskang9/wasm-tutorial"
const wasm = import("wasm-loader!../build/react_rust_wasm_bg");

wasm.then(wasm => {
  const App = () => {
    return (
      <div>
        <h1>Hi there</h1>
        <input></input>
        <button onClick={wasm.fibonacci(144)}>Run Computation</button>
      </div>
    );
  };

  ReactDOM.render(<App />, document.getElementById("root"));
});
