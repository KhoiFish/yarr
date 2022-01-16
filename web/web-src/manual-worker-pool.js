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
    for (var y1 = imageHeight; y1 > 0; y1 -= maxScanLineHeight) {
        var y0 = Math.max(0, y1 - maxScanLineHeight);
        var h = y1 - y0;
        scanLines.push({ x: 0, y: y0, w: imageWidth, h });
    }

    return scanLines;
}

function copyScanLines(imageWidth, source, { x, y, w, h }, destBuffer) {
    const oneScanLineSize = (imageWidth * 4);
    const offset = oneScanLineSize * y;
    destBuffer.set(source, offset);
}

async function scanLineWorkerFunc(previewCb, worker, scanLines, imageWidth, imageHeight, finalResults) {
    while (scanLines.length > 0) {
        var scanLine = scanLines.pop();
        var results = await worker.workerRenderRegion(scanLine);
        if (previewCb != null) {
            previewCb(imageWidth, results, scanLine);
        } else {
            copyScanLines(imageWidth, results, scanLine, finalResults);
        }
    }
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

    // Kick off all the workers
    var workerPromises = [];
    for (var workerId = 0; workerId < workerPool.length; workerId++) {
        workerPromises.push(scanLineWorkerFunc(previewCb, workerPool[workerId], scanLines, imageWidth, imageHeight, finalResults));
    }

    // Wait till all workers are done
    for (var workerId = 0; workerId < workerPromises.length; workerId++) {
        await workerPromises[workerId];
    }

    return finalResults;
}

// --------------------------------------------------------------------------------------------------------------------
  
async function workerPoolRenderImage({ sceneNum, previewCb, width, height, numSamples, maxDepth, enableBvh }) {
    const start = performance.now();
    const maxScanLineHeight = 4;
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
    return await workerPoolRenderImage({ sceneNum, previewCb: null, width, height, numSamples, maxDepth, enableBvh });
}

// --------------------------------------------------------------------------------------------------------------------

export { MAX_NUM_WORKERS, initWorkerPool, workerPoolRenderImage, workerPoolRenderImageNoPreview, copyScanLines };