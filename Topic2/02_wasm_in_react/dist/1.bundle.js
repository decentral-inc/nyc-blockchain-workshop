(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "./build/react_rust_wasm.js":
/*!**********************************!*\
  !*** ./build/react_rust_wasm.js ***!
  \**********************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

"use strict";
eval("\n\nObject.defineProperty(exports, \"__esModule\", {\n    value: true\n});\nexports.__wbg_alert_42c5391edfd46cdc = __wbg_alert_42c5391edfd46cdc;\nexports.fibonacci = fibonacci;\nexports.__wbindgen_throw = __wbindgen_throw;\n\nvar _react_rust_wasm_bg = __webpack_require__(/*! ./react_rust_wasm_bg */ \"./build/react_rust_wasm_bg.wasm\");\n\nvar wasm = _interopRequireWildcard(_react_rust_wasm_bg);\n\nfunction _interopRequireWildcard(obj) { if (obj && obj.__esModule) { return obj; } else { var newObj = {}; if (obj != null) { for (var key in obj) { if (Object.prototype.hasOwnProperty.call(obj, key)) newObj[key] = obj[key]; } } newObj.default = obj; return newObj; } }\n\nvar lTextDecoder = typeof TextDecoder === 'undefined' ? __webpack_require__(/*! util */ \"./node_modules/util/util.js\").TextDecoder : TextDecoder; /* tslint:disable */\n\n\nvar cachedTextDecoder = new lTextDecoder('utf-8');\n\nvar cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {\n        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nfunction __wbg_alert_42c5391edfd46cdc(arg0, arg1) {\n    var varg0 = getStringFromWasm(arg0, arg1);\n    alert(varg0);\n}\n/**\n* @param {number} arg0\n* @returns {void}\n*/\nfunction fibonacci(arg0) {\n    return wasm.fibonacci(arg0);\n}\n\nfunction __wbindgen_throw(ptr, len) {\n    throw new Error(getStringFromWasm(ptr, len));\n}\n\n//# sourceURL=webpack:///./build/react_rust_wasm.js?");

/***/ }),

/***/ "./build/react_rust_wasm_bg.wasm":
/*!***************************************!*\
  !*** ./build/react_rust_wasm_bg.wasm ***!
  \***************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./build/react_rust_wasm_bg.wasm?");

/***/ })

}]);