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
const resourceMap = new Map();
var previewImgData;

// --------------------------------------------------------------------------------------------------------------------

function updateTimeLabel(timeInMs) {
    var timeInSec = timeInMs / 1000;
    timeOutput.value = `${timeInSec.toFixed(2)} sec(s)`;
}

// --------------------------------------------------------------------------------------------------------------------

async function initializeWorkerPool(numThreads) {
    // Only re-initialze if thread count is different
    if (ManualWorkerPool.getNumWorkers() != numThreads) {
        await ManualWorkerPool.initWorkerPool(numThreads, resourceMap);
    }
}

// --------------------------------------------------------------------------------------------------------------------

function getRenderFunction(handler) {
    return async function () {
        setEnableRenderUI(false);
        previewImgData = new ImageData(width, height);
        ctx.putImageData(previewImgData, 0, 0);
        
        // Re-init worker pool (if necessary)
        const numThreads = parseInt(numThreadsOutput.value);
        await initializeWorkerPool(numThreads);

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

function previewCb(imageWidth, source, { x, y, w, h }) {
    ManualWorkerPool.copyScanLines(imageWidth, source, { x, y, w, h }, previewImgData.data);
}

function previewDraw() {
    ctx.putImageData(previewImgData, 0, 0);
}

function getPreviewFunction() {
    return async function () {            
        // Kick off preview drawing
        setEnableRenderUI(false);
        previewImgData = new ImageData(width, height);
        ctx.putImageData(previewImgData, 0, 0);
        const drawInteral = setInterval(previewDraw, 250);

        // Re-init worker pool (if necessary)
        const numThreads = parseInt(numThreadsOutput.value);
        await initializeWorkerPool(numThreads);

        // Kick off the progressive raytracing
        const sceneNum = parseInt(sceneNumOutput.value);
        const numSamples = parseInt(samplesNumOutput.value);
        const maxDepth = parseInt(maxDepthNumOutput.value);
        const enableBvh = (bvhEnableOutput.value === 'true');
        let { rawImageData, time } = await ManualWorkerPool.workerPoolRenderImage({ sceneNum, previewCb, width, height, numSamples, maxDepth, enableBvh });

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
        showDiv('parametersRow1');
        showDiv('parametersRow2');
        showDiv('renderButton');
    }
    else {
        showDiv('progress');
        hideDiv('parametersRow1');
        hideDiv('parametersRow2');
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

    // Try and detect num hardware threads, or default to 8
    var numThreads = navigator.hardwareConcurrency || 8;
    await initializeWorkerPool(numThreads);
    numThreadsOutput.value = `${numThreads}`;

    // Are rayon threads supported ?
    const threadsSupported = (await wasmHandlers.supportsThreads) ? true : false;
    if (threadsSupported == false) {
        // Hide rayon mode from list
        hideDiv('rayonDropDown');
    }

    // Map methods to table
    renderFunctionMap.set('single', getRenderFunction(wasmHandlers.singleThread));
    renderFunctionMap.set('rayon', getRenderFunction(wasmHandlers.multiThread));
    renderFunctionMap.set('workers', getPreviewFunction());

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
