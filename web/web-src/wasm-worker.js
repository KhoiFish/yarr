import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

// --------------------------------------------------------------------------------------------------------------------

function wrapExports(handler) {
  return Comlink.proxy({
    renderImage: wrapRenderImageFunc(handler)
  });
}

function wrapRenderImageFunc({ render_image }) {
  return ({ width, height, numSamples, maxDepth }) => {
    const start = performance.now();
    const rawImageData = render_image(width, height, numSamples, maxDepth);
    const time = performance.now() - start;
    return {
      // Little perf boost to transfer data to the main thread w/o copying.
      rawImageData: Comlink.transfer(rawImageData, [rawImageData.buffer]),
      time
    };
  };
}

// --------------------------------------------------------------------------------------------------------------------

async function initHandlers() {
  let [singleThread, multiThread] = await Promise.all(
    [
      // Single-thread handler
      (async () => {
        const singleThread = await import('../../target/web/pkg/web.js');
        await singleThread.default();
        return wrapExports(singleThread);
      })(),

      // Multi-thread handler
      (async () => {
        if (!(await threads())) {
          // Threads un-supported, skip
          return;
        }
        const multiThread = await import('../../target/web/pkg-parallel/web.js');
        await multiThread.default();
        await multiThread.initThreadPool(navigator.hardwareConcurrency);
        return wrapExports(multiThread);
      })(),
  ]);

  return Comlink.proxy({
    singleThread,
    supportsThreads: !!multiThread,
    multiThread,
  });
}

// --------------------------------------------------------------------------------------------------------------------
// Main entry

Comlink.expose({
  handlers: initHandlers()
});
