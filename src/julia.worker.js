import { expose } from 'comlink';

const wasm = import('../pkg/index.js');

export default class WasmWorker {
  generate(data) {
    const { width, height, realPart, imaginaryPart } = data;

    return new Promise(async (resolve) => {
      const lib = await wasm;
      const base64 = lib.julia_base64(width, height, realPart, imaginaryPart);
      resolve(base64);
    });
  }
}

expose(WasmWorker);
