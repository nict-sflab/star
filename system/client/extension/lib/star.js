const authStar = () => {
    return new Promise((resolve, reject) => {
      window.addEventListener("message", (e) => {
        if (e.source !== window) return;
        if (e.data?.type !== "STAR_RESULT") return;

        const result = e.data.payload
        console.log("star: ", result);
        resolve(result)
      });

      window.postMessage({ type: "STAR", payload: {} }, "*")
  })
}