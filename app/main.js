import init, { run_app } from '../pkg/project3.js';
async function main() {
   await init('/pkg/project3_bg.wasm');
   run_app();
}
main()