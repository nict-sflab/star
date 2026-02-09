import init, { init_user, request_register, receive_credential, request_authenticate } from "./wasm/star_client_wasm.js";

const API_HOST = "http://localhost:8080"

const one = BigInt(1)

const initStar = async () => {
  const wasmUrl = chrome.runtime.getURL("wasm/star_client_wasm_bg.wasm");
  await init(wasmUrl)

  const pkResp = await fetch(`${API_HOST}/star/public_key`)
  const publicKey = await pkResp.text()
  console.log(publicKey)

  await init_user(publicKey)

}

chrome.runtime.onStartup.addListener(
  async () => {
    await initStar();
  }
)

chrome.runtime.onInstalled.addListener(async () => {
  await initStar();

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

  const credential = receive_credential(credCipher, commonKey)
  console.log(credential)

  const counter = 0;
  const date = Date.now();
  
  await chrome.storage.local.set({ credential })
  await chrome.storage.local.set({ counter })
  await chrome.storage.local.set({ date })

});

chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    (async () => {
        const { credential } = await chrome.storage.local.get(["credential"]);
        if (!credential) {
            throw new Error("Missing credential in storage");
        }

        const { counter: rawCounter } = await chrome.storage.local.get(["counter"]);
        const counter = BigInt(rawCounter ?? 0);

        const proof = await request_authenticate(counter, credential);
        const next = counter + one;
        await chrome.storage.local.set({ counter: next });

        sendResponse( proof );
    })().catch((err) => {
        console.error("auth message failed", err);
        sendResponse({ err });
    });

    return true;
});
