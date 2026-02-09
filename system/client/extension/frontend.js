const API_HOST = "http://localhost:8080";
const STORAGE_KEYS = {
  cred: "cred",
  counter: "counter",
  period: "period",
};

const wasm = {
  init: null,
  init_user: null,
  request_register: null,
  receive_credential: null,
  request_authenticate: null,
};

let initPromise;
let registerPromise;

const getStorage = (key) => localStorage.getItem(key);
const setStorage = (key, value) => localStorage.setItem(key, value);
const readCounter = () => {
  const raw = getStorage(STORAGE_KEYS.counter);
  return raw === null ? 0n : BigInt(raw);
};

const runInit = async () => {
  if (!wasm.init) {
    const wasmModuleUrl = chrome.runtime.getURL("wasm/star_client_wasm.js");
    const wasmModule = await import(wasmModuleUrl);
    wasm.init = wasmModule.default;
    wasm.init_user = wasmModule.init_user;
    wasm.request_register = wasmModule.request_register;
    wasm.receive_credential = wasmModule.receive_credential;
    wasm.request_authenticate = wasmModule.request_authenticate;
  }

  const wasmUrl = chrome.runtime.getURL("wasm/star_client_wasm_bg.wasm");
  const wasmBytes = await (await fetch(wasmUrl)).arrayBuffer();
  await wasm.init({ module_or_path: wasmBytes });

  const publicKey = await (await fetch(`${API_HOST}/star/public_key`)).text();
  await wasm.init_user(publicKey);
};

const initStar = () => {
  initPromise ??= runInit();
  return initPromise;
};

const runRegister = async () => {
  const [request, commonKey] = wasm.request_register().split(":");
  const regResp = await fetch(`${API_HOST}/star/register`, {
    method: "POST",
    body: request,
  });

  const credCipher = await regResp.text();
  const cred = wasm.receive_credential(credCipher, commonKey);

  setStorage(STORAGE_KEYS.cred, cred);
  setStorage(STORAGE_KEYS.counter, "0");
  setStorage(STORAGE_KEYS.period, Date.now().toString());
};

const ensureRegistered = async () => {
  await initStar();

  const existingCred = getStorage(STORAGE_KEYS.cred);
  const existingCounter = getStorage(STORAGE_KEYS.counter);
  if (existingCred && existingCounter !== null) return;

  registerPromise ??= runRegister();
  return registerPromise;
};

window.addEventListener("load", async () => {
  try {
    await ensureRegistered();
  } catch (err) {
    console.error("STAR init/register failed:", err);
  }
});

// call like here: 
// window.addEventListener("message", (e) => {
//   if (e.source !== window) return;
//   if (e.data?.type !== "STAR_RESULT") return;
// 
//   console.log("result!: ", e.data.payload);
// });
// 
// window.postMessage({ type: "STAR", payload: {} }, "*");
window.addEventListener("message", async (e) => {
  if (e.source !== window) return;
  if (e.data?.type !== "STAR") return;

  try {
    await initStar();

    const cred = getStorage(STORAGE_KEYS.cred);
    const period = getStorage(STORAGE_KEYS.period);
    const counter = readCounter();

    if (!cred) {
      throw new Error("STAR credential missing");
    }

    const proof = await wasm.request_authenticate(counter, cred);
    setStorage(STORAGE_KEYS.counter, (counter + 1n).toString());

    console.log("extension:", proof, counter, period, cred);

    window.postMessage({ type: "STAR_RESULT", payload: proof }, "*");
  } catch (err) {
    console.error("STAR auth failed:", err);
    window.postMessage(
      { type: "STAR_RESULT", error: true, message: String(err?.message ?? err) },
      "*"
    );
  }
});
