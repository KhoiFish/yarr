import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

var wasmModule;
var workerResourceCache;
var webRaytracer;

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

async function init(workerId, resourceMap) {
    // Load our web assembly module
    wasmModule = await import('../../target/web/pkg/web.js');
    await wasmModule.default();

    // Seed random
    const seedFunc = xmur3(makeid(32));
    await wasmModule.seed_rand(seedFunc());

    // Create resource cache
    workerResourceCache = wasmModule.create_empty_resource_cache();
    for (let [path, data] of resourceMap) {
        wasmModule.insert_resource(workerResourceCache, path, data);
    }
}

// --------------------------------------------------------------------------------------------------------------------

async function workerCreateRaytracer(sceneNum, imageWidth, imageHeight, samplesPerPixel, maxDepth, enableBvh) {
    webRaytracer = wasmModule.create_webraytracer(workerResourceCache, sceneNum, imageWidth, imageHeight, samplesPerPixel, maxDepth, enableBvh);
}

async function workerRenderRegion({ x, y, w, h }) {
    var rawImageData = await wasmModule.multi_sample_region(webRaytracer, x, y, w, h);
    return Comlink.transfer(rawImageData, [rawImageData.buffer]);
}

// --------------------------------------------------------------------------------------------------------------------
// Main entry

Comlink.expose({
    init,
    workerCreateRaytracer,
    workerRenderRegion
});
