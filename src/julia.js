import { wrap } from "comlink";

const instance = new Worker(new URL("./julia.worker", import.meta.url));
const Wrapper = wrap(instance);

instance.addEventListener("message", (m) => {
  if (m.data?.type === "update-percentage") {
    document.dispatchEvent(
      new CustomEvent("update-progress", {
        detail: { message: () => m.data.message },
      })
    );
  }
});

const getWorker = async () => {
  const JuliaWorker = await new Wrapper();
  return JuliaWorker;
};

export default getWorker();
