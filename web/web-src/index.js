import * as Comlink from 'comlink';
import * as ManualWorkerPool from './manual-worker-pool.js';
import * as WasmModule from '../../target/web/pkg/web.js';

// --------------------------------------------------------------------------------------------------------------------

const numSamplesSlider = document.getElementById('numSamplesSlider');
const maxDepthSlider = document.getElementById('maxDepthSlider');
const canvas = document.getElementById('canvas');
const { width, height } = canvas;
const ctx = canvas.getContext('2d');
const timeOutput = document.getElementById('time');
const numThreadsOutput = document.getElementById('numThreads');
const buttonAvailableMap = new Map();
const enableBvhCheckbox = document.getElementById('bvhCheckbox');
const sceneNumOutput = document.getElementById('sceneNum');
var previewImgData;

// --------------------------------------------------------------------------------------------------------------------

function updateTimeLabel(timeInMs) {
    var timeInSec = timeInMs / 1000;
    timeOutput.value = `${timeInSec.toFixed(2)} sec(s)`;
}

// --------------------------------------------------------------------------------------------------------------------

function setupRenderBtn(buttonId, handler) {
    var button = document.getElementById(buttonId);
    Object.assign(button, {
        async onclick() {
            setEnableAvaialbleButtons(false);
            const sceneNum = parseInt(sceneNumOutput.value);
            const numSamples = parseInt(numSamplesSlider.value);
            const maxDepth = parseInt(maxDepthSlider.value);
            let { rawImageData, time } = await handler.renderImage({
                sceneNum,
                width,
                height,
                numSamples,
                maxDepth,
                enableBvh: enableBvhCheckbox.checked
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

function setupPreviewRenderBtn(buttonId) {
    var button = document.getElementById(buttonId);
    Object.assign(button, {
        async onclick() {            
            // Kick off preview drawing
            setEnableAvaialbleButtons(false);
            previewImgData = new ImageData(width, height);
            const drawInteral = setInterval(previewDraw, 250);

            // Kick off the progressive raytracing
            const sceneNum = parseInt(sceneNumOutput.value);
            const numSamples = parseInt(numSamplesSlider.value);
            const maxDepth = parseInt(maxDepthSlider.value);
            let { time } = await ManualWorkerPool.workerPoolRenderImageProgressive({ sceneNum, previewCb, width, height, numSamples, maxDepth, enableBvh: enableBvhCheckbox.checked });

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
    // Load wasm module and load our resources (images)
    const resourceInfo = [ { path: "earthmap.jpeg" } ];
    await WasmModule.default();
    const resourceCache = await WasmModule.create_and_load_resource_cache(resourceInfo);

    // Convert to a JS map, this allows for structured cloning (deep copies) of the
    // loaded resources when we pass the map to other web workers.
    const resourceMap = new Map();
    for (var i = 0; i < resourceInfo.length; i++) {
        var resource = resourceInfo[i].path;
        resourceMap.set(resource, WasmModule.get_resource(resourceCache, resource));
    }

    // Get handlers to rust wasm api
    let wasmHandlers = await Comlink.wrap(
        new Worker(new URL('./wasm-worker.js', import.meta.url), {
            type: 'module'
        })
    ).initHandlers(resourceMap);

    // Spin up our manual web worker pool
    await ManualWorkerPool.initWorkerPool(resourceMap);

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
    setupRenderBtn('singleThreadBtn', wasmHandlers.singleThread);
    setupRenderBtn('multiThreadBtn', wasmHandlers.multiThread);
    setupRenderBtn('manualWebWorkers', { renderImage: ManualWorkerPool.workerPoolRenderImage });
    setupPreviewRenderBtn('manualWebWorkersPreview');

    // Now enable buttons if they are available
    setEnableAvaialbleButtons(true);
}

// --------------------------------------------------------------------------------------------------------------------
// Main entry

init();
