// --------------------------------------------------------------------------------------------------------------------
// Load wasm first
function loadWasm() {
    wasm_bindgen('./web_bg.wasm')
        .then(run)
        .catch(console.error);
}
loadWasm();

// --------------------------------------------------------------------------------------------------------------------

function run()
{
    const { WebRaytracer, WebColor } = wasm_bindgen;
    const button = document.getElementById('render');

    wasm_bindgen.greet();

    // Cache canvas resources
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
        // const raytracer = new WebRaytracer();
        // raytracer.multi_threaded_render();
    };
}