import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

const MAX_NUM_WORKERS = navigator.hardwareConcurrency || 8;
const workerPool = [];

// --------------------------------------------------------------------------------------------------------------------

async function initWorkerPool() {
    for (var i = 0; i < MAX_NUM_WORKERS; i++ ) {
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

function assert(condition, message) {
    if (!condition) {
        throw new Error(message || "Assertion failed");
    }
}

var rgbToHex = function (rgba) { 
    var hex = Number(rgba).toString(16);
    if (hex.length < 2) {
        hex = "0" + hex;
    }
    return hex;
};

function gammaCorrect(c) {
    return Math.max(0.0, Math.min(Math.sqrt(c), 0.999));
}

function convertToU8Range(c) {
    return (c * 256);
}

// --------------------------------------------------------------------------------------------------------------------

async function kickOffWorkerPoolRenderImage(imageWidth, imageHeight, samplesPerPixel, maxDepth) {
    // Distribute the sampling across the worker threads
    var numSamplesPerWorkerTable = [];
    var numWorkers = 0;
    {
        var numSamplesToDo = samplesPerPixel;
        var currentWorkerId = 0;
        do {
            if (numWorkers < MAX_NUM_WORKERS) {
                numSamplesPerWorkerTable.push(0);
                numWorkers++;
                currentWorkerId = (numSamplesPerWorkerTable.length - 1);
            } else {
                currentWorkerId = (currentWorkerId + 1) % MAX_NUM_WORKERS;
            }

            numSamplesPerWorkerTable[currentWorkerId]++;
            numSamplesToDo--;
        } while (numSamplesToDo > 0)
    }
    
    // Kick off the work for the worker threads
    var workerResults = []
    for (var workerId = 0; workerId < numSamplesPerWorkerTable.length; workerId++)  {
        workerResults.push(workerPool[workerId].workerRenderImage(imageWidth, imageHeight, numSamplesPerWorkerTable[workerId], maxDepth));
    }

    // Wait for results from all threads
    var resultsList = []
    for (var workerId = 0; workerId < workerResults.length; workerId++)  {
        resultsList.push(await workerResults[workerId]);
    }

    // Convert to final form
    var finalBufferSize = imageWidth * imageHeight * 4;
    var finalResults = new Uint8ClampedArray(finalBufferSize);
    var sumScale = 1.0 / samplesPerPixel;
    var componentCount = 0;
    for (var i = 0; i < finalBufferSize; i++) {
        // Skip alpha, it is always 1.0
        componentCount = (componentCount + 1) % 4;
        if (componentCount == 0) {
            finalResults[i] = 255;
            continue;
        }

        // Sum up results and normalize the color
        var sum = 0.0;
        for (var resultIndex = 0; resultIndex < resultsList.length; resultIndex++) {
            sum += resultsList[resultIndex][i];
        }
        finalResults[i] = convertToU8Range(gammaCorrect(sum * sumScale));
    }

    return finalResults;
}
  
async function workerPoolRenderImage({ width, height, numSamples, maxDepth }) {
    const start = performance.now();
    var rawImageData = await kickOffWorkerPoolRenderImage(width, height, numSamples, maxDepth);
    const time = performance.now() - start;
    return {
        rawImageData: Comlink.transfer(rawImageData, [rawImageData.buffer]),
        time
    };
}

// --------------------------------------------------------------------------------------------------------------------

async function kickOffWorkerPoolRenderImageProgressive(previewCb, imageWidth, imageHeight, samplesPerPixel, maxDepth) {
    // // Allocate final buffer upfront
    // var finalBufferSize = imageWidth * imageHeight * 4;
    // var finalResults = new Uint8ClampedArray(finalBufferSize);

    // // Define our own callback
    // var ourCb = function (x, y, colorU32) {
    //     // Write to our final results buffer
    //     // Note we intentionally skip alpha (i = 4)
    //     var offset = ((y * imageWidth) + x) * 4;
    //     for (var i = 0; i < 3; i++) {
    //         // Shift and mask off the channel value
    //         finalResults[offset + i] = (colorU32 >> (8 * (3 - i))) & 0xff;
    //     }
    //     finalResults[offset + 3] = 255;

    //     // Call user's callback
    //     userCb(x, y, colorU32);
    // }

    // Divide into regions
    const maxRegionWidth = Math.floor(imageWidth / MAX_NUM_WORKERS);
    const workersList = [];
    var numWorkers = 0;
    for (var x = 0; x < imageWidth; x += maxRegionWidth) {
        var regionWidth = Math.min(maxRegionWidth, (imageWidth - x));
        var regionHeight = imageHeight;
        workersList.push(workerPool[numWorkers++].workerRenderImageProgressive(Comlink.proxy(previewCb), imageWidth, imageHeight, samplesPerPixel, maxDepth, x, 0, regionWidth, regionHeight));
    }

    // Wait for everything to come back
    for (var i = 0; i < workersList.length; i++) {
        await workersList[i];
    }

    // // Done
    // return finalResults;
}
  
async function workerPoolRenderImageProgressive({ previewCb, width, height, numSamples, maxDepth }) {
    const start = performance.now();
    await kickOffWorkerPoolRenderImageProgressive(previewCb, width, height, numSamples, maxDepth);
    const time = performance.now() - start;
    return {
        time
    };
}

// --------------------------------------------------------------------------------------------------------------------

export { MAX_NUM_WORKERS, initWorkerPool, workerPoolRenderImage, workerPoolRenderImageProgressive };