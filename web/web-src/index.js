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
const buttonAvailableMap = new Map();
var previewImgData;

// --------------------------------------------------------------------------------------------------------------------

function updateTimeLabel(timeInMs) {
    var timeInSec = timeInMs / 1000;
    timeOutput.value = `${timeInSec.toFixed(2)} sec(s)`;
}

// --------------------------------------------------------------------------------------------------------------------

function setupRenderBtn(button, handler) {
    Object.assign(button, {
        async onclick() {
            setEnableAvaialbleButtons(false);
            const numSamples = parseInt(numSamplesSlider.value);
            const maxDepth = parseInt(maxDepthSlider.value);
            let { rawImageData, time } = await handler.renderImage({
                width,
                height,
                numSamples,
                maxDepth
            });
            updateTimeLabel(time);
            ctx.putImageData(new ImageData(rawImageData, width, height), 0, 0);
            setEnableAvaialbleButtons(true);
        }
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
}

function previewDraw() {
    ctx.putImageData(previewImgData, 0, 0);
}

function setupPreviewRenderBtn(button) {
    Object.assign(button, {
        async onclick() {            
            // Kick off preview drawing
            setEnableAvaialbleButtons(false);
            previewImgData = new ImageData(width, height);
            const drawInteral = setInterval(previewDraw, 250);

            // Kick off the progressive raytracing
            const numSamples = parseInt(numSamplesSlider.value);
            const maxDepth = parseInt(maxDepthSlider.value);
            let { time } = await ManualWorkerPool.workerPoolRenderImageProgressive({ previewCb, width, height, numSamples, maxDepth });

            // Done rendering
            updateTimeLabel(time);
            clearInterval(drawInteral);
            ctx.putImageData(previewImgData, 0, 0);
            setEnableAvaialbleButtons(true);
        }
    });
}

// --------------------------------------------------------------------------------------------------------------------

function setEnableAvaialbleButtons(enable) {
    for (let [buttonId, isAvailable] of buttonAvailableMap) {
        var buttonEnabled = isAvailable && enable;
        document.getElementById(buttonId).disabled = !buttonEnabled;
    }
}

// --------------------------------------------------------------------------------------------------------------------

async function init() {
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

    // Are threads supported ?
    const threadsSupported = (await wasmHandlers.supportsThreads) ? true : false;

    // Map which buttons are available
    buttonAvailableMap.set('singleThreadBtn', true);
    buttonAvailableMap.set('multiThreadBtn', threadsSupported);
    buttonAvailableMap.set('manualWebWorkers', true);
    buttonAvailableMap.set('manualWebWorkersPreview', true);

    // Setup buttons, they are disabled by default
    setupRenderBtn(document.getElementById('singleThreadBtn'), wasmHandlers.singleThread);
    setupRenderBtn(document.getElementById('multiThreadBtn'), wasmHandlers.multiThread);
    setupRenderBtn(document.getElementById('manualWebWorkers'), { renderImage: ManualWorkerPool.workerPoolRenderImage });
    setupPreviewRenderBtn(document.getElementById('manualWebWorkersPreview'));

    // Now enable buttons if they are available
    setEnableAvaialbleButtons(true);
}

// --------------------------------------------------------------------------------------------------------------------
// Main entry

init();
