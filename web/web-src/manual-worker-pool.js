import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

const MAX_NUM_WORKERS = navigator.hardwareConcurrency || 8;
const workerPool = [];

// --------------------------------------------------------------------------------------------------------------------

async function initWorkerPool(resourceMap) {
    for (var i = 0; i < MAX_NUM_WORKERS; i++ ) {
        let worker = await Comlink.wrap(
            new Worker(new URL('./manual-web-worker.js', import.meta.url), {
                type: 'module'
            })
        );
        await worker.init(i, resourceMap);
        workerPool.push(worker);
    }
}

// --------------------------------------------------------------------------------------------------------------------

function assert(condition, message) {
    if (!condition) {
        throw new Error(message || "Assertion failed");
    }
}

// --------------------------------------------------------------------------------------------------------------------

function buildScanLines(imageWidth, imageHeight, maxScanLineHeight) {
    // Sanity check
    assert(imageHeight >= maxScanLineHeight);

    // Create list of tiles
    var scanLines = [];
    for (var y = 0; y < imageHeight; y += maxScanLineHeight) {
        var h = Math.min(maxScanLineHeight, imageHeight - y);
        scanLines.push({ x: 0, y, w: imageWidth, h });
    }

    return scanLines;
}

function copyScanLines(imageWidth, source, { x, y, w, h }, destBuffer) {
    const oneScanLineSize = (imageWidth * 4);
    const offset = oneScanLineSize * y;
    destBuffer.set(source, offset);
}

async function renderImageScanlines(previewCb, scanLines, sceneNum, imageWidth, imageHeight, samplesPerPixel, maxDepth, enableBvh) {
    // Create buffer to store final results
    var finalBufferSize = 0;
    var finalResults = null;
    if (previewCb == null) {
        finalBufferSize = imageWidth * imageHeight * 4;
        finalResults = new Uint8ClampedArray(finalBufferSize);
    }

    // Have our workers create the raytracer objects
    for (var workerId = 0; workerId < workerPool.length; workerId++) {
        await workerPool[workerId].workerCreateRaytracer(sceneNum, imageWidth, imageHeight, samplesPerPixel, maxDepth, enableBvh);
    }

    // Form scanline workers with extra meta data
    var scanLineWorkers = []
    for (var workerId = 0; workerId < workerPool.length; workerId++) {
        scanLineWorkers.push({ worker: workerPool[workerId], scanLine: null, isWorking: false, resultsPromise: null });
    }

    // Raytrace the tiles
    while (scanLines.length > 0) {
        // Find a free worker
        var workerFound = false;
        for (var workerId = 0; workerId < scanLineWorkers.length; workerId++) {
            // If we find a free worker, pop off the scanline and kick off the worker
            if (scanLineWorkers[workerId].isWorking == false) {
                var scanLine = scanLines.pop();
                scanLineWorkers[workerId].scanLine = scanLine;
                scanLineWorkers[workerId].resultsPromise = scanLineWorkers[workerId].worker.workerRenderRegion(scanLine);
                scanLineWorkers[workerId].isWorking = true;
                workerFound = true;
                break;
            }
        }

        // All workers are busy, we must wait for at least one to be frees
        if (workerFound == false) {
            for (var workerId = 0; workerId < scanLineWorkers.length; workerId++) {
                if (scanLineWorkers[workerId].isWorking == true) {
                    // Copy results to final buffer
                    var results = await scanLineWorkers[workerId].resultsPromise;
                    scanLineWorkers[workerId].isWorking = false;
                    if (previewCb != null) {
                        previewCb(imageWidth, results, scanLineWorkers[workerId].scanLine);
                    } else {
                        copyScanLines(imageWidth, results, scanLineWorkers[workerId].scanLine, finalResults);
                    }

                    // This worker is free now, get it on the next iteration
                    break;
                }
            }
        }
    }

    // Wait for any last workers to come back
    for (var workerId = 0; workerId < scanLineWorkers.length; workerId++) {
        if (scanLineWorkers[workerId].isWorking == true) {
            // Copy results to final buffer
            var results = await scanLineWorkers[workerId].resultsPromise;
            scanLineWorkers[workerId].isWorking = false;
            if (previewCb != null) {
                previewCb(imageWidth, results, scanLineWorkers[workerId].scanLine);
            } else {
                copyScanLines(imageWidth, results, scanLineWorkers[workerId].scanLine, finalResults);
            }
        }
    }

    return finalResults;
}

// --------------------------------------------------------------------------------------------------------------------
  
async function workerPoolRenderImage({ sceneNum, previewCb, width, height, numSamples, maxDepth, enableBvh }) {
    const start = performance.now();
    const maxScanLineHeight = Math.ceil(height / MAX_NUM_WORKERS);
    const scanLines = buildScanLines(width, height, maxScanLineHeight);
    const rawImageData =  await renderImageScanlines(previewCb, scanLines, sceneNum, width, height, numSamples, maxDepth, enableBvh);
    const time = performance.now() - start;

    if (previewCb != null) {
        return {
            rawImageData: null,
            time
        };
    } else {    
        return {
            rawImageData: Comlink.transfer(rawImageData, [rawImageData.buffer]),
            time
        };
    }
}

async function workerPoolRenderImageNoPreview({ sceneNum, width, height, numSamples, maxDepth, enableBvh }) {
    return await workerPoolRenderImage({ sceneNum,previewCb: null, width, height, numSamples, maxDepth, enableBvh });
}

// --------------------------------------------------------------------------------------------------------------------

export { MAX_NUM_WORKERS, initWorkerPool, workerPoolRenderImage, workerPoolRenderImageNoPreview, copyScanLines };