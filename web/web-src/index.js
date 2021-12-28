import * as Comlink from 'comlink';
import * as ManualWorkerPool from './manual-worker-pool.js';

// --------------------------------------------------------------------------------------------------------------------

const numSamplesSlider = document.getElementById('numSamplesSlider');
const maxDepthSlider = document.getElementById('maxDepthSlider');
const canvas = document.getElementById('canvas');
const { width, height } = canvas;
const ctx = canvas.getContext('2d');
const timeOutput = document.getElementById('time');
const numThreadsOutput = document.getElementById('numThreads');
const previewImgData = new ImageData(width, height);

// --------------------------------------------------------------------------------------------------------------------

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

// --------------------------------------------------------------------------------------------------------------------

function previewCb(x, y, colorU32) {
  // Note we intentionally skip alpha (i = 4)
  var offset = ((y * width) + x) * 4;
  for (var i = 0; i < 3; i++) {
      // Shift and mask off the channel value
      previewImgData.data[offset + i] = (colorU32 >> (8 * (3 - i))) & 0xff;
  }
  previewImgData.data[offset + 3] = 255;

  // Update canvas
  ctx.putImageData(previewImgData, 0, 0);
}

// --------------------------------------------------------------------------------------------------------------------

function setupPreviewRenderBtn(button) {
  Object.assign(button, {
    async onclick() {
      const numSamples = parseInt(numSamplesSlider.value);
      const maxDepth = parseInt(maxDepthSlider.value);
      let { time } = await ManualWorkerPool.workerPoolRenderImageProgressive( { previewCb, width, height, numSamples, maxDepth });
      timeOutput.value = `${time.toFixed(2)} ms`;
    },
    disabled: false
  });
}

// --------------------------------------------------------------------------------------------------------------------

(async function init() {
  // Get handlers to rust wasm api
  let wasmHandlers = await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;

  // Spin up our manual web worker pool
  await ManualWorkerPool.initWorkerPool();

  // Set label to how many threads detected
  numThreadsOutput.value = `Num threads: ${ManualWorkerPool.MAX_NUM_WORKERS}`;

  
  // Setup buttons
  setupRenderBtn(document.getElementById('singleThreadBtn'), wasmHandlers.singleThread);
  if (await wasmHandlers.supportsThreads) {
    setupRenderBtn(document.getElementById('multiThreadBtn'), wasmHandlers.multiThread);
  }
  setupRenderBtn(document.getElementById('manualWebWorkers'), { renderImage: ManualWorkerPool.workerPoolRenderImage });
  setupPreviewRenderBtn(document.getElementById('manualWebWorkersPreview'));

})();
