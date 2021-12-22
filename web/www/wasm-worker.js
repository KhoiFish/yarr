const multiThread = await import(
    './web.js'
);
await multiThread.default();
await multiThread.initThreadPool(navigator.hardwareConcurrency);