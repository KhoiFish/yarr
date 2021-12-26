import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

var wasmModule;
var numWorkers = navigator.hardwareConcurrency;
var workerPool = [];
var numSamplesToProcess = [];

// --------------------------------------------------------------------------------------------------------------------

async function __init() {
    // Load our web assembly module
    wasmModule = await import('../../target/web/pkg/web.js');
    await wasmModule.default();
}

async function __initWorkerPool() {
    for (var i = 0; i < numWorkers; i++ ) {
        let worker = await Comlink.wrap(
            new Worker(new URL('./manual-web-worker.js', import.meta.url), {
            type: 'module'
            })
        );
        await worker.init();
        workerPool.push(worker);
        numSamplesToProcess.push(0);
    }
}

// --------------------------------------------------------------------------------------------------------------------

function __workerRenderImage(image_width, image_height, samples_per_pixel, max_depth) {
    return wasmModule.render_image(image_width, image_height, samples_per_pixel, max_depth);
}

async function __startWorkerPool(image_width, image_height, samples_per_pixel, max_depth) {
    var bufferSize = image_width * image_height * 4;
    var finalResult = workerPool[0].workerRenderImage(image_width, image_height, samples_per_pixel, max_depth);

    //var finalResult = new Uint8ClampedArray(bufferSize);
    //var numSamplesPerWorker = samples_per_pixel / numWorkers;
    //var remainderSamples = samples_per_pixel % numWorkers;
    // var numSamplesPerWorker = 1;
    // var remainderSamples = 0;
    // for (var i = 0; i < numWorkers; i++ ) {
    //     let worker = workerPool[i];

    //     // Render image
    //     var workerResults = await worker.workerRenderImage(image_width, image_height, numWorkerSamples, max_depth);

    //     // Combine results
    //     // for (var j = 0; j < bufferSize; j++) {
    //     //     finalResult[j] += workerResults[j];
    //     // }
    //     finalResult = workerResults;
    // }

    // // Average out to num samples
    // var scale = 1.0 / samples_per_pixel;
    // for (j = 0; j < bufferSize; j++) {
    //     finalResult[j] *= scale; 
    // }

    return finalResult;
}

// --------------------------------------------------------------------------------------------------------------------
  
function render_image(image_width, image_height, samples_per_pixel, max_depth) {
    wasmModule.wasm_alert("hello");
    return __startWorkerPool(image_width, image_height, samples_per_pixel, max_depth);
}

// --------------------------------------------------------------------------------------------------------------------
// Main entry

Comlink.expose({
    init: __init,
    initWorkerPool: __initWorkerPool,
    workerRenderImage: __workerRenderImage,
    render_image,
});
  