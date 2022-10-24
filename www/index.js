import * as wasm from "wasm-city";

const universe = wasm.Universe.new('canvas-city');

window.universe = universe;

const renderLoop = () => {
    universe.tick();
    requestAnimationFrame(renderLoop);
};

renderLoop();
