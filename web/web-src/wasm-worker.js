import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

function wrapRenderImageFunc({ render_image }) {
  return ({ width, height, numSamples, maxDepth }) => {
    const start = performance.now();
    const rawImageData = render_image(width, height, numSamples, maxDepth);
    const time = performance.now() - start;
    return {
      rawImageData: Comlink.transfer(rawImageData, [rawImageData.buffer]),
      time
    };
  };
}

// --------------------------------------------------------------------------------------------------------------------

async function initHandlers() {
  let [singleThreadExports, multiThreadExports] = await Promise.all(
    [
      // Single-thread
      (async () => {
        const singleThreadImport = await import('../../target/web/pkg/web.js');
        await singleThreadImport.default();

        return Comlink.proxy({
          renderImage: wrapRenderImageFunc(singleThreadImport)
        });
      })(),

      // Multi-thread
      (async () => {
        // Check to see if threads are supported
        if (!(await threads())) {
          return;
        }
        const multiThreadImport = await import('../../target/web/pkg-parallel/web.js');
        await multiThreadImport.default();
        await multiThreadImport.seed_rand(Date.now());
        await multiThreadImport.initThreadPool(navigator.hardwareConcurrency);

        return Comlink.proxy({
          renderImage: wrapRenderImageFunc(multiThreadImport)
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
  handlers: initHandlers()
});
