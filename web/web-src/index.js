import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

const numSamplesSlider = document.getElementById('numSamplesSlider');
const maxDepthSlider = document.getElementById('maxDepthSlider');
const canvas = document.getElementById('canvas');
const { width, height } = canvas;
const ctx = canvas.getContext('2d');
const timeOutput = document.getElementById('time');

// --------------------------------------------------------------------------------------------------------------------

(async function init() {
  // Spin up a web worker to get our exports
  let handlers = await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;

  function setupRenderBtn(button, handler) {
    Object.assign(button, {
      async onclick() {
        const numSamples = parseInt(numSamplesSlider.value);
        const maxDepth = parseInt(maxDepthSlider.value);
        let { rawImageData, time } = await handler.renderImage({
          width,
          height,
          numSamples,
          maxDepth
        });
        timeOutput.value = `${time.toFixed(2)} ms`;
        const imgData = new ImageData(rawImageData, width, height);
        ctx.putImageData(imgData, 0, 0);
      },
      disabled: false
    });
  }

  setupRenderBtn(document.getElementById('singleThreadBtn'), handlers.singleThread);
  setupRenderBtn(document.getElementById('manualWebWorkers'), handlers.manualWebWorkers);
  if (await handlers.supportsThreads) {
    setupRenderBtn(document.getElementById('multiThreadBtn'), handlers.multiThread);
  }
})();
