import * as Comlink from 'comlink';
import * as ManualWorkerPool from './manual-worker-pool.js';
import * as WasmModule from '../../target/web/pkg/web.js';

// --------------------------------------------------------------------------------------------------------------------

const canvas = document.getElementById('canvas');
const { width, height } = canvas;
const ctx = canvas.getContext('2d');
const timeOutput = document.getElementById('time');
const renderButton =  document.getElementById('renderButton');
const numThreadsOutput = document.getElementById('numThreads');
const renderFunctionMap = new Map();
const sceneNumOutput = document.getElementById('sceneNum');
const samplesNumOutput = document.getElementById('samplesNum');
const maxDepthNumOutput = document.getElementById('maxDepthNum');
const bvhEnableOutput = document.getElementById('bvhEnable');
const resolutionOutput = document.getElementById('resolution');
const threadsModeOutput = document.getElementById('threadsMode');
var previewImgData;

// --------------------------------------------------------------------------------------------------------------------

function updateTimeLabel(timeInMs) {
    var timeInSec = timeInMs / 1000;
    timeOutput.value = `${timeInSec.toFixed(2)} sec(s)`;
}

// --------------------------------------------------------------------------------------------------------------------

function getRenderFunction(handler) {
    return async function () {
        setEnableRenderUI(false);
        previewImgData = new ImageData(width, height);
        ctx.putImageData(previewImgData, 0, 0);

        const sceneNum = parseInt(sceneNumOutput.value);
        const numSamples = parseInt(samplesNumOutput.value);
        const maxDepth = parseInt(maxDepthNumOutput.value);
        const enableBvh = (bvhEnableOutput.value === 'true');
        let { rawImageData, time } = await handler.renderImage({
            sceneNum,
            width,
            height,
            numSamples,
            maxDepth,
            enableBvh
        });

        updateTimeLabel(time);
        ctx.putImageData(new ImageData(rawImageData, width, height), 0, 0);
        setEnableRenderUI(true);
    };
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

function getPreviewFunction() {
    return async function () {            
        // Kick off preview drawing
        setEnableRenderUI(false);
        previewImgData = new ImageData(width, height);
        const drawInteral = setInterval(previewDraw, 250);

        // Kick off the progressive raytracing
        const sceneNum = parseInt(sceneNumOutput.value);
        const numSamples = parseInt(samplesNumOutput.value);
        const maxDepth = parseInt(maxDepthNumOutput.value);
        const enableBvh = (bvhEnableOutput.value === 'true');
        let { time } = await ManualWorkerPool.workerPoolRenderImageProgressive({ sceneNum, previewCb, width, height, numSamples, maxDepth, enableBvh });

        // Done rendering
        updateTimeLabel(time);
        clearInterval(drawInteral);
        ctx.putImageData(previewImgData, 0, 0);
        setEnableRenderUI(true);
    };
}

// --------------------------------------------------------------------------------------------------------------------

function setEnableRenderUI (enable){
    if (enable) {
        hideDiv('progress');
        showDiv('parameters');
        showDiv('renderButton');
    }
    else {
        showDiv('progress');
        hideDiv('parameters');
        hideDiv('renderButton');
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
    numThreadsOutput.value = `${ManualWorkerPool.MAX_NUM_WORKERS}`;

    // Are rayon threads supported ?
    const threadsSupported = (await wasmHandlers.supportsThreads) ? true : false;
    if (threadsSupported == false) {
        // Hide rayon mode from list
        hideDiv('rayonDropDown');
    }

    // Map methods to table
    renderFunctionMap.set('single', getRenderFunction(wasmHandlers.singleThread));
    renderFunctionMap.set('rayon', getRenderFunction(wasmHandlers.multiThread));
    renderFunctionMap.set('workers', getRenderFunction({ renderImage: ManualWorkerPool.workerPoolRenderImage }));
    renderFunctionMap.set('preview', getPreviewFunction());

    // Set up render button click event
    Object.assign(renderButton, {
        async onclick() {
            const threadMode = threadsModeOutput.value;
            renderFunctionMap.get(threadMode)();
        }
    });

    // Update resolution output
    resolutionOutput.value = `${width}x${height}`;

    // Everything loaded, hide loading screen
    hideDiv('loading');
    showDiv('paramInfo');

    // Enable render UI
    setEnableRenderUI(true);
}

// --------------------------------------------------------------------------------------------------------------------
// Main entry

init();
