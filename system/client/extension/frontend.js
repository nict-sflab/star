const API_HOST = "http://localhost:8080";

const ROUTE_MAP = {
  0: "8081", 
  1: "8081" 
}

const STORAGE_KEYS = {
  cred: "cred",
  counter: "counter",
  period: "period",
  installMarker: "star_install_marker_v1",
};

const wasm = {
  init: null,
  init_user: null,
  request_register: null,
  receive_credential: null,
  request_authenticate: null,
  route: null
};

let initPromise;
let registerPromise;

const getTimestamp = (date = new Date()) =>  {
    const secs = date.getTime() / 1000 / 60
    const floored = Math.floor(secs);

    return BigInt(floored)
}

const getStorage = async (key) => {
  const result = await chrome.storage.local.get(key);
  const value = result?.[key];
  return typeof value === "undefined" ? null : value;
};
const setStorage = (key, value) => chrome.storage.local.set({ [key]: value });
const readCounter = async () => {
  const raw = await getStorage(STORAGE_KEYS.counter);
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
    wasm.route = wasmModule.route;
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

  await setStorage(STORAGE_KEYS.cred, cred);
  await setStorage(STORAGE_KEYS.counter, "0");
  await setStorage(STORAGE_KEYS.period, getTimestamp().toString());
};

const ensureRegistered = async () => {
  await initStar();

  const existingCred = await getStorage(STORAGE_KEYS.cred);
  const existingCounter = await getStorage(STORAGE_KEYS.counter);
  if (existingCred && existingCounter !== null) return;

  registerPromise ??= runRegister();
  return registerPromise;
};

const isFirstInstall = async () =>
  (await getStorage(STORAGE_KEYS.installMarker)) !== "1";

const injectLibrary = () => {
  if (document.getElementById("star-extension-lib")) return;
  const s = document.createElement("script");
  s.id = "star-extension-lib";
  s.src = chrome.runtime.getURL("lib/star.js");
  s.onload = () => s.remove();
  (document.head || document.documentElement).append(s);
};

const boot = async () => {
  try {
    injectLibrary();
    if (await isFirstInstall()) {
      await ensureRegistered();
      await setStorage(STORAGE_KEYS.installMarker, "1");
    }
  } catch (err) {
    console.error("STAR init/register failed:", err);
  }
};

if (document.readyState === "loading") {
  window.addEventListener("DOMContentLoaded", boot, { once: true });
} else {
  boot();
}

window.addEventListener("message", async (e) => {
  if (e.source !== window) return;
  if (e.data?.type !== "STAR") return;

  try {
    await initStar();

    const period = getTimestamp();

    const cred = await getStorage(STORAGE_KEYS.cred);
    const last = await getStorage(STORAGE_KEYS.period);
    let counter = 0n


    console.log(`period: ${last} = ${period}`)

    if (last == period) {
      counter = await readCounter()
      console.log(`counter: ${counter}`)
    }

    const proof = await wasm.request_authenticate(counter, period, cred);
    const routeIndex = await wasm.route(counter, period, cred, 1);
    const route = ROUTE_MAP[routeIndex];
    
    await setStorage(STORAGE_KEYS.counter, (counter + 1n).toString());
    await setStorage(STORAGE_KEYS.period, period.toString());

    // console.log("extension:", proof, counter, period, cred);

    window.postMessage({ type: "STAR_RESULT", proof: proof, route: route }, "*");
  } catch (err) {
    console.error("STAR auth failed:", err);
    window.postMessage(
      { type: "STAR_RESULT", error: true, message: String(err?.message ?? err) },
      "*"
    );
  }
});
