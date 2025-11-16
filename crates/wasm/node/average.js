import { avg } from "../pkg/wasm_avg.js";

const run =  () => {
  console.log(avg([10, 20, 30])); // → 20
};

run();
