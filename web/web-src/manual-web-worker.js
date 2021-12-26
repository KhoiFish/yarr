import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

var wasmModule;
var numWorkers = navigator.hardwareConcurrency;
var workerPool = [];

// --------------------------------------------------------------------------------------------------------------------

async function __init(workerId) {
    // Load our web assembly module
    wasmModule = await import('../../target/web/pkg/web.js');
    await wasmModule.default();
    //await wasmModule.seed_rand(workerId);
}

async function __initWorkerPool() {
    for (var i = 0; i < numWorkers; i++ ) {
        let worker = await Comlink.wrap(
            new Worker(new URL('./manual-web-worker.js', import.meta.url), {
            type: 'module'
            })
        );
        await worker.init(i);
        workerPool.push(worker);
    }
}

// --------------------------------------------------------------------------------------------------------------------

async function __workerRenderImage(image_width, image_height, samples_per_pixel, max_depth) {
    return await wasmModule.multi_sample_image(false, image_width, image_height, samples_per_pixel, max_depth);
}

async function __startWorkerPool(image_width, image_height, samples_per_pixel, max_depth) {
    var numSamplesToProcess = [];

    samples_per_pixel = 1;

    // Kick off the work for the worker threads
    var workerResults = []
    for (var i = 0; i < 32; i++) {
        workerResults.push(workerPool[i].workerRenderImage(image_width, image_height, samples_per_pixel, max_depth));
    }

    // Get final list of results
    var resultsList = []
    for (var i = 0; i < 32; i++) {
        var result = await workerResults[i];
        resultsList.push(result);
    }

    // Convert to final form
    var finalBufferSize = image_width * image_height * 4;
    var finalResults = new Uint8ClampedArray(finalBufferSize);
    var scale = 1.0 / 32.0;
    for (var i = 0; i < finalBufferSize; i++) {
        var sum = 0.0;
        for (var workerId = 0; workerId < numWorkers; workerId++) {
            var workerResult = resultsList[workerId];
            sum += workerResult[i];
        }
        finalResults[i] = (sum * scale) * 256.0;
    }

    return finalResults;
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
  