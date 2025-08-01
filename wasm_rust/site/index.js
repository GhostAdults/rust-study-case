import init, * as wasm  from "wasm_rust_simd";
// greet("STAO")

async function run() {
  await init(); // 初始化 wasm 模块


  WebAssembly.instantiateStreaming(fetch("memory.wasm"), {
    js: { mem: memory },
  }).then((obj) => {
    var i32 = new Uint32Array(memory.buffer);
    for (var i = 0; i < 10; i++) {
      i32[i] = i;
    }
    var sum = obj.instance.exports.accumulate(0, 10);
    console.log(sum);
  });


  const SIZE = 10_000_000;
  const ptr = wasm.alloc(SIZE); // 👈 从命名导出中访问函数
  const wasmArray = new Float64Array(memory.buffer, ptr, SIZE);
  wasmArray.fill(1);

  console.time("🧩 WASM 求和");
  const result = wasm.sum_array(ptr, SIZE);
  console.timeEnd("🧩 WASM 求和");

  console.log("WASM 求和结果：", result);
}

run();



