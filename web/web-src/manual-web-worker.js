import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

var wasmModule;

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

async function init(workerId) {
    // Load our web assembly module
    wasmModule = await import('../../target/web/pkg/web.js');
    await wasmModule.default();

    // Seed random
    const seedFunc = xmur3(makeid(32));
    await wasmModule.seed_rand(seedFunc());
}

// --------------------------------------------------------------------------------------------------------------------

async function workerRenderImage(imageWidth, imageHeight, samplesPerPixel, maxDepth, enableBvh) {
    var raytracer = wasmModule.create_webraytracer(imageWidth, imageHeight, samplesPerPixel, maxDepth, enableBvh);
    return await wasmModule.multi_sample_buffer(raytracer, false);
}

async function workerRenderImageProgressive(previewCb, imageWidth, imageHeight, samplesPerPixel, maxDepth, enableBvh, regionX, regionY, regionWidth, regionHeight)  {
    var raytracer = wasmModule.create_webraytracer(imageWidth, imageHeight, samplesPerPixel, maxDepth, enableBvh);
    for(var y = regionY; y < (regionY + regionHeight); y++) {
        for(var x = regionX; x < (regionX + regionWidth); x++) {
            var colorU32 = await wasmModule.multi_sample_point(raytracer, x, (imageHeight-y));
            await previewCb(x, y, colorU32);
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------
// Main entry

Comlink.expose({
    init,
    workerRenderImage,
    workerRenderImageProgressive
});
  