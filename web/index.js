import * as Comlink from 'comlink';

const maxIterations = 1000;

const canvas = document.getElementById('canvas');
const { width, height } = canvas;
const ctx = canvas.getContext('2d');
const timeOutput = document.getElementById('time');

(async function init() {
  // Create a separate thread from wasm-worker.js and get a proxy to its handlers.
  let handlers = await Comlink.wrap(
    new Worker(new URL('./wasm-worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;

  function setupBtn(id) {
    // Handlers are named in the same way as buttons.
    let handler = handlers[id];
    // If handler doesn't exist, it's not supported.
    if (!handler) return;
    // Assign onclick handler + enable the button.
    Object.assign(document.getElementById(id), {
      async onclick() {
        let { rawImageData, time } = await handler({
          width,
          height,
          maxIterations
        });
        timeOutput.value = `${time.toFixed(2)} ms`;
        const imgData = new ImageData(rawImageData, width, height);
        ctx.putImageData(imgData, 0, 0);
      },
      disabled: false
    });
  }

  setupBtn('singleThread');
  if (await handlers.supportsThreads) {
    setupBtn('multiThread');
  }
})();

// import init, { greet, WebRaytracer, WebColor } from './web.js';

// // --------------------------------------------------------------------------------------------------------------------

// function check_for_shared_memory_feature() {
//     let msg = 'This demo requires a current version of Firefox (e.g., 79.0)';
//     if (typeof SharedArrayBuffer !== 'function') {
//         alert('this browser does not have SharedArrayBuffer support enabled' + '\n\n' + msg);
//         return
//     }
    
//     // Test for bulk memory operations with passive data segments
//     //  (module (memory 1) (data passive ""))
//     const buf = new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
//         0x05, 0x03, 0x01, 0x00, 0x01, 0x0b, 0x03, 0x01, 0x01, 0x00]);
//     if (!WebAssembly.validate(buf)) {
//         alert('this browser does not support passive wasm memory, demo does not work' + '\n\n' + msg);
//         return
//     }
// }

// // --------------------------------------------------------------------------------------------------------------------

// async function init_wasm() {
//     await init();
//     greet("Welcome, friend! Custom wasm module has been loaded.");
// }

// // --------------------------------------------------------------------------------------------------------------------

// function wasmWorker(modulePath) {
 
//     // Create an object to later interact with 
//     const proxy = {};
 
//     // Keep track of the messages being sent
//     // so we can resolve them correctly
//     let id = 0;
//     let idPromises = {};
 
//     return new Promise((resolve, reject) => {
//         const worker = new Worker('worker.js');
//         worker.postMessage({eventType: "INITIALISE", eventData: modulePath});
//         worker.addEventListener('message', function(event) {
 
//             const { eventType, eventData, eventId } = event.data;
 
//             if (eventType === "INITIALISED") {
//                 const methods = event.data.eventData;
//                 methods.forEach((method) => {
//                     proxy[method] = function() {
//                         return new Promise((resolve, reject) => {
//                             worker.postMessage({
//                                 eventType: "CALL",
//                                 eventData: {
//                                     method: method,
//                                     arguments: Array.from(arguments) // arguments is not an array
//                                 },
//                                 eventId: id
//                             });
 
//                             idPromises[id] = { resolve, reject };
//                             id++
//                         });
//                     }
//                 });
//                 resolve(proxy);
//                 return;
//             } else if (eventType === "RESULT") {
//                 if (eventId !== undefined && idPromises[eventId]) {
//                     idPromises[eventId].resolve(eventData);
//                     delete idPromises[eventId];
//                 }
//             } else if (eventType === "ERROR") {
//                 if (eventId !== undefined && idPromises[eventId]) {
//                     idPromises[eventId].reject(event.data.eventData);
//                     delete idPromises[eventId];
//                 }
//             }
             
//         });
 
//         worker.addEventListener("error", function(error) {
//             reject(error);
//         });
//     })
 
// }

// // --------------------------------------------------------------------------------------------------------------------

// function main_function() {
//     var button = document.getElementById('render');
//     var canvas = document.getElementById('canvas');
//     var ctx = canvas.getContext('2d');

//     var render = function() {
//         const render_width = 320;
//         const render_height = 180;
//         const myImageData = ctx.createImageData(render_width, render_height);
//         const data = myImageData.data;
//         const raytracer = new WebRaytracer();

//         // Raytrace
//         for (var y = 0; y < render_height; y++) {
//             for (var x = 0; x < render_width; x++) {
//                 const color = raytracer.multi_sample(x, y);
//                 const offset = (((render_height-y) * render_width) + x) * 4;
//                 data[offset + 0] = color.r(); 
//                 data[offset + 1] = color.g(); 
//                 data[offset + 2] = color.b(); 
//                 data[offset + 3] = 255; 
//             }
//         }

//         ctx.putImageData(myImageData, 0, 0);
//     };

//     button.onclick = function() {
//         //render();
//         // Something to try later with using worker threads
//         //await new Worker('wasm-worker.js', { type: 'module'});

//         wasmWorker("./calculator.wasm").then((wasmProxyInstance) => {
//             wasmProxyInstance.multi_sample
//             wasmProxyInstance.add(2, 3)
//                 .then((result) => {
//                     console.log(result); // 5
//                 })
//                 .catch((error) => {
//                     console.error(error);
//                 });
         
//             wasmProxyInstance.divide(100, 10)
//                 .then((result) => {
//                     console.log(result); // 10
//                 })
//                 .catch((error) => {
//                     console.error(error);
//                 });
//         });
//     };
// }

// // --------------------------------------------------------------------------------------------------------------------
// // Main entry

// init_wasm();
// main_function();
