import Julia from './julia';

const loadImage = async () => {
  const output = document.getElementById('output');

  const lib = await Julia;
  const data = { width: 400, height: 400, realPart: -0.8, imaginaryPart: 0.156};
  const base64 = await lib.generate(data);
  const imgPath = 'data:image/png;base64,' + base64;

  output.setAttribute('src', imgPath);
};

window.addEventListener('DOMContentLoaded', loadImage, false);
