const API_HOST = "http://localhost:8080"

const one = BigInt(1)

let init;
let init_user;
let request_register;
let receive_credential;
let request_authenticate;
let initPromise;
let registerPromise;

const initStar = async () => {
  if (initPromise) return 

  initPromise = (async () => {
  if (!init) {
    const wasmModuleUrl = chrome.runtime.getURL("wasm/star_client_wasm.js");
    const wasmModule = await import(wasmModuleUrl);
    ({ default: init, init_user, request_register, receive_credential, request_authenticate } = wasmModule);
    }

    const wasmUrl = chrome.runtime.getURL("wasm/star_client_wasm_bg.wasm");
    const wasmResponse = await fetch(wasmUrl);
    const wasmBytes = await wasmResponse.arrayBuffer();
    await init({ module_or_path: wasmBytes })

    const pkResp = await fetch(`${API_HOST}/star/public_key`)
    const publicKey = await pkResp.text()
    console.log(publicKey)

    await init_user(publicKey)
  })();


    return initPromise;
  }

const register = async () => {
  const existingCred = localStorage.getItem("cred");
  const existingCounter = localStorage.getItem("counter");
  if (existingCred && existingCounter !== null) {
    return;
  }

  if (registerPromise) { return }
  
  registerPromise = (async () => {
    const r = request_register()
    const [request, commonKey] = r.split(":")
    console.log(request)
    console.log(commonKey)

    const regResp = await fetch(`${API_HOST}/star/register`, {
    method : "POST",
    body: request
    })

    const credCipher = await regResp.text()
    console.log(credCipher)

    const cred = receive_credential(credCipher, commonKey)
    console.log(cred)

    const counter = 0n;
    const date = Date.now().toString();

    localStorage.setItem("cred", cred)
    localStorage.setItem("counter", counter.toString())
    localStorage.setItem("period", date)
    })();
   return registerPromise;
}


window.addEventListener("load", async(e) => {
  try {
    await initStar();
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
window.addEventListener("message", (e) => {
    if (e.source !== window) return;
    if (e.data?.type !== "STAR") return;

    (async () => {
        try {
          await initStar();

          const cred = localStorage.getItem("cred")
          const period = localStorage.getItem("period")
          let counter = localStorage.getItem("counter")
          counter = BigInt(counter ?? 0);

          if (!cred) {
            throw new Error("STAR credential missing");
          }

          const proof = await request_authenticate(counter, cred);
          localStorage.setItem("counter", (counter + 1n).toString())

          console.log("extension:", proof, counter, period, cred)

          window.postMessage({ type: "STAR_RESULT", payload: proof }, "*")
        } catch (err) {
          console.error("STAR auth failed:", err);
          window.postMessage({ type: "STAR_RESULT", error: true, message: String(err?.message ?? err) }, "*")
        }
    })()

    return true;
});
