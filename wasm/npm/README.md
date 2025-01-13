# Pyrin WASM SDK

An integration wrapper around [`pyrin-wasm`](https://www.npmjs.com/package/pyrin-wasm) module that uses [`websocket`](https://www.npmjs.com/package/websocket) W3C adaptor for WebSocket communication.

This is a Node.js module that provides bindings to the Pyrin WASM SDK strictly for use in the Node.js environment. The web browser version of the SDK is available as part of official SDK releases at [https://github.com/Pyrinpyi/pyrin/releases](https://github.com/Pyrinpyi/pyrin/releases)

## Usage

Pyrin NPM module exports include all WASM32 bindings.
```javascript
const pyrin = require('pyrin');
console.log(pyrin.version());
```

## Documentation

Documentation is available at [https://pyrin.aspectron.org/docs/](https://pyrin.aspectron.org/docs/)


## Building from source & Examples

SDK examples as well as information on building the project from source can be found at [https://github.com/Pyrinpyi/pyrin/tree/master/wasm](https://github.com/Pyrinpyi/pyrin/tree/master/wasm)

## Releases

Official releases as well as releases for Web Browsers are available at [https://github.com/Pyrinpyi/pyrin/releases](https://github.com/Pyrinpyi/pyrin/releases).

Nightly / developer builds are available at: [https://aspectron.org/en/projects/pyrin-wasm.html](https://aspectron.org/en/projects/pyrin-wasm.html)

