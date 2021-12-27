import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

var wasmModule;
var numWorkers = navigator.hardwareConcurrency;
var workerPool = [];

// --------------------------------------------------------------------------------------------------------------------

// https://stackoverflow.com/questions/1349404/generate-random-string-characters-in-javascript
function makeid(length) {
    var result = '';
    var characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    var charactersLength = characters.length;
    for ( var i = 0; i < length; i++ ) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
}

// https://stackoverflow.com/questions/521295/seeding-the-random-number-generator-in-javascript
function xmur3(str) {
    for(var i = 0, h = 1779033703 ^ str.length; i < str.length; i++)
        h = Math.imul(h ^ str.charCodeAt(i), 3432918353),
        h = h << 13 | h >>> 19;
    return function() {
        h = Math.imul(h ^ h >>> 16, 2246822507);
        h = Math.imul(h ^ h >>> 13, 3266489909);
        return (h ^= h >>> 16) >>> 0;
    }
}

// --------------------------------------------------------------------------------------------------------------------

async function __init(workerId) {
    // Load our web assembly module
    wasmModule = await import('../../target/web/pkg/web.js');
    await wasmModule.default();

    // Seed random
    const seedFunc = xmur3(makeid(32));
    await wasmModule.seed_rand(seedFunc());
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
    for (var workerId = 0; workerId < numWorkers; workerId++)  {
        workerResults.push(workerPool[workerId].workerRenderImage(image_width, image_height, samples_per_pixel, max_depth));
    }

    // Wait for results from all threads
    var resultsList = []
    for (var workerId = 0; workerId < numWorkers; workerId++)  {
        resultsList.push(await workerResults[workerId]);
    }

    // Convert to final form
    var finalBufferSize = image_width * image_height * 4;
    var finalResults = new Uint8ClampedArray(finalBufferSize);
    var scale = 1.0 / 32.0;
    for (var i = 0; i < finalBufferSize; i++) {
        var sum = 0.0;
        for (var workerId = 0; workerId < numWorkers; workerId++) {
            sum += resultsList[workerId][i];
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
  