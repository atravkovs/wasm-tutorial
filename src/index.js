import Julia from "./julia";

const loadImage = async () => {
  const output = document.getElementById("output");
  const submit = document.getElementById("submit");
  const progress = document.getElementById("progress");

  const widthElement = document.getElementById("width");
  const heightElement = document.getElementById("height");
  const realElement = document.getElementById("real");
  const imaginaryElement = document.getElementById("imaginary");

  const lib = await Julia;

  submit.addEventListener("click", async (event) => {
    event.preventDefault();

    if (
      !widthElement.value ||
      !heightElement.value ||
      !realElement.value ||
      !imaginaryElement.value
    ) {
      return;
    }

    const data = {
      width: +widthElement.value,
      height: +heightElement.value,
      realPart: +realElement.value,
      imaginaryPart: +imaginaryElement.value,
    };
    const base64 = await lib.generate(data);
    const imgPath = "data:image/png;base64," + base64;

    output.setAttribute("src", imgPath);
  });

  document.addEventListener("update-progress", (event) => {
    progress.value = event.detail.message();
  });
};

window.addEventListener("DOMContentLoaded", loadImage, false);
