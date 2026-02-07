import init, { init_user, request_register, receive_credential } from "./wasm/star_client_wasm.js";

const PROXY_HOST = "http://localhost:8081"
const API_HOST = "http://localhost:8080"

let initPromise = null;

var a = null

function ensureWasmReady() {
  if (!initPromise) {
    const wasmUrl = chrome.runtime.getURL("wasm/star_client_wasm_bg.wasm");
    initPromise = init(wasmUrl);
  }
  return initPromise;
}

chrome.runtime.onInstalled.addListener(async () => {
  await ensureWasmReady();
  const pkResp = await fetch(`${API_HOST}/star/public_key`)
  init()

  let publicKey = await pkResp.text()
  console.log(publicKey)

  init_user(publicKey)

  let r = request_register()
  let [request, commonKey] = r.split(":")
  console.log(request)
  console.log(commonKey)

  const regResp = await fetch(`${API_HOST}/star/register`, {
    method : "POST",
    body: request
  })

  let credCipher = await regResp.text()
  console.log(credCipher)

  let cred = receive_credential(credCipher, commonKey)
  console.log(cred)
  await chrome.storage.local.set({ cred })
  await chrome.storage.local.set({ publicKey })

  let result = await chrome.storage.local.get(["cred"])
  console.log("Value is " + result.cred);
});

