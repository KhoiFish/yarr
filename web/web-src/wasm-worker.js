import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

function createResourceCache(importedHandler, resourceMap) {
    var resourceCache = importedHandler.create_empty_resource_cache();
    for (let [path, data] of resourceMap) {
        importedHandler.insert_resource(resourceCache, path, data);
    }

    return resourceCache;
}

// --------------------------------------------------------------------------------------------------------------------

function wrapRenderImageFunc(importedHandler, resourceCache) {
    return ({ sceneNum, width, height, numSamples, maxDepth, enableBvh }) =>
    {
        // Render
        const start = performance.now();
        const raytracer = importedHandler.create_webraytracer(resourceCache, sceneNum, width, height, numSamples, maxDepth, enableBvh );
        const rawImageData = importedHandler.render_image(raytracer);
        const time = performance.now() - start;
        return {
            rawImageData: Comlink.transfer(rawImageData, [rawImageData.buffer]),
            time
        };
    };
}

// --------------------------------------------------------------------------------------------------------------------

async function initHandlers(resourceMap) {
    let [singleThreadExports, multiThreadExports] = await Promise.all(
        [
            // Single-thread
            (async () =>
            {
                const singleThreadImport = await import('../../target/web/pkg/web.js');
                await singleThreadImport.default();

                return Comlink.proxy({
                    renderImage: wrapRenderImageFunc(singleThreadImport, createResourceCache(singleThreadImport, resourceMap))
                });
            })(),

            // Multi-thread
            (async () =>
            {
                // Check to see if threads are supported
                if (!(await threads()))
                {
                    return;
                }
                const multiThreadImport = await import('../../target/web/pkg-parallel/web.js');
                await multiThreadImport.default();
                await multiThreadImport.initThreadPool(navigator.hardwareConcurrency);

                return Comlink.proxy({
                    renderImage: wrapRenderImageFunc(multiThreadImport, createResourceCache(multiThreadImport, resourceMap))
                });
            })(),
        ]);

    return Comlink.proxy({
        singleThread: singleThreadExports,
        multiThread: multiThreadExports,
        supportsThreads: !!multiThreadExports,
    });
}

// --------------------------------------------------------------------------------------------------------------------
// Main entry

Comlink.expose({
    initHandlers
});
