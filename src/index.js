console.log('Hello World!');

const wasm = import('../pkg/index.js');

const loadImage = () => {
  const output = document.getElementById('output');

  wasm.then(lib => {
    console.log('WASM scripts loaded');
  
    const t0 = performance.now();
    const base64 = lib.julia_base64(400, 400, -0.8, 0.156);
    const t1 = performance.now();
    console.log(`Call to generate image took ${t1 - t0} milliseconds.`);
    const imgPath = 'data:image/png;base64,' + base64;

    output.setAttribute('src', imgPath);
  });
};

window.addEventListener('DOMContentLoaded', loadImage, false);
