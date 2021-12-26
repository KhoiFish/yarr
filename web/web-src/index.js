import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

const numSamples = 16;
const maxDepth = 64;
const canvas = document.getElementById('canvas');
const { width, height } = canvas;
const ctx = canvas.getContext('2d');
const timeOutput = document.getElementById('time');

// --------------------------------------------------------------------------------------------------------------------

(async function init() {
  // Create a separate thread from wasm-worker.js and get a proxy to its handlers.
  let handlers = await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;

  function setupBtn(button, handler) {
    Object.assign(button, {
      async onclick() {
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

  setupBtn(document.getElementById('singleThread'), handlers['singleThread']);
  if (await handlers.supportsThreads) {
    setupBtn(document.getElementById('multiThread'), handlers['multiThread']);
  }
})();
