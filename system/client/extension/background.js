// // wasm-pack が生成した JS グルーを ESModule として import
import init, { add, fib, greet } from "./wasm/star_client_wasm.js";

const HOST = "http://localhost:3000"
// Service Worker はアイドルで停止→再起動が起きるので、都度 init できるようにする
let initPromise = null;

function ensureWasmReady() {
  if (!initPromise) {
    // .wasm の URL は chrome.runtime.getURL で作る
    const wasmUrl = chrome.runtime.getURL("wasm/star_client_wasm_bg.wasm");
    initPromise = init(wasmUrl);
  }
  return initPromise;
}

chrome.runtime.onInstalled.addListener(async () => {
  await ensureWasmReady();
  const resp = await fetch(`${HOST}/star/public_key`)
  console.log(resp.text)
  init(resp.text)


  // console.log("add(2,3) =", add(2, 3));
  // console.log("fib(10)  =", fib(10));
});

// // 例: 他の extension page / content script から呼び出せるようにする
// chrome.runtime.onMessage.addListener((msg, _sender, sendResponse) => {
//   (async () => {
//     await ensureWasmReady();

//     if (msg?.type === "ADD") {
//       sendResponse({ ok: true, result: add(msg.a | 0, msg.b | 0) });
//       return;
//     }
//     if (msg?.type === "FIB") {
//       sendResponse({ ok: true, result: fib(msg.n >>> 0) });
//       return;
//     }
//     if (msg?.type === "GREET") {
//       sendResponse({ ok: true, result: greet(String(msg.name ?? "")) });
//       return;
//     }

//     sendResponse({ ok: false, error: "Unknown message type" });
//   })();

//   // 非同期で sendResponse するので true を返す
//   return true;
// });