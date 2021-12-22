import init, { greet, WebRaytracer, WebColor } from './web.js';

// --------------------------------------------------------------------------------------------------------------------

function check_for_shared_memory_feature() {
    let msg = 'This demo requires a current version of Firefox (e.g., 79.0)';
    if (typeof SharedArrayBuffer !== 'function') {
        alert('this browser does not have SharedArrayBuffer support enabled' + '\n\n' + msg);
        return
    }
    
    // Test for bulk memory operations with passive data segments
    //  (module (memory 1) (data passive ""))
    const buf = new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
        0x05, 0x03, 0x01, 0x00, 0x01, 0x0b, 0x03, 0x01, 0x01, 0x00]);
    if (!WebAssembly.validate(buf)) {
        alert('this browser does not support passive wasm memory, demo does not work' + '\n\n' + msg);
        return
    }
}

// --------------------------------------------------------------------------------------------------------------------

async function init_wasm() {
    await init();
    greet("Welcome, friend! Custom wasm module has been loaded.");
}

// --------------------------------------------------------------------------------------------------------------------

function main_function() {
    var button = document.getElementById('render');
    var canvas = document.getElementById('canvas');
    var ctx = canvas.getContext('2d');

    var render = function() {
        const render_width = 320;
        const render_height = 180;
        const myImageData = ctx.createImageData(render_width, render_height);
        const data = myImageData.data;
        const raytracer = new WebRaytracer();

        // Raytrace
        for (var y = 0; y < render_height; y++) {
            for (var x = 0; x < render_width; x++) {
                const color = raytracer.sample(x, y);
                const offset = (((render_height-y) * render_width) + x) * 4;
                data[offset + 0] = color.r(); 
                data[offset + 1] = color.g(); 
                data[offset + 2] = color.b(); 
                data[offset + 3] = 255; 
            }
        }

        ctx.putImageData(myImageData, 0, 0);
    };

    button.onclick = function() {
        render();

        // Something to try later with using worker threads
        //await new Worker('wasm-worker.js', { type: 'module'});
    };
}

// --------------------------------------------------------------------------------------------------------------------
// Main entry

init_wasm();
main_function();
