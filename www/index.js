import { Graph } from "../pkg/wasm_rs";

const canvas = document.getElementById("gol");

let width = 32;
let height = 32;

let g = Graph.new(width, height);

setInterval(() => {
	canvas.innerText = g.render();
	g.tick();
}, 100)
