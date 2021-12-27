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

async function kickOffWorkerPoolRenderImageProgressive(progressiveCb, imageWidth, imageHeight, samplesPerPixel, maxDepth) {
    
}
  
async function workerPoolRenderImageProgressive({ progressiveCb, width, height, numSamples, maxDepth }) {
    const start = performance.now();
    var rawImageData = await kickOffWorkerPoolRenderImage(width, height, numSamples, maxDepth);
    const time = performance.now() - start;
    return {
        rawImageData: Comlink.transfer(rawImageData, [rawImageData.buffer]),
        time
    };
}

// --------------------------------------------------------------------------------------------------------------------

export { MAX_NUM_WORKERS, initWorkerPool, workerPoolRenderImage, workerPoolRenderImageProgressive };