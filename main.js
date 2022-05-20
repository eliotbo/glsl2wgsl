import init, {greet, do_parse} from "./pkg/glsl2wgsl.js";

init()
    .then(() => {
        greet("WebAssembly")
        
    });

window.dum = function dum() {
    let t = document.getElementById("glslarea");
    // let tt = t.innerHTML.valueOf();
    let tt = t.value;
    let savior = do_parse(tt);

    document.getElementById("wgslarea").innerHTML = savior;
}