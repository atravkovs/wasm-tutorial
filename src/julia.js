import { wrap } from 'comlink';

const instance = new Worker(new URL('./julia.worker', import.meta.url));
const Wrapper = wrap(instance);

const getWorker = async () => {
  const JuliaWorker = await new Wrapper();
  return JuliaWorker;
}

export default getWorker();
