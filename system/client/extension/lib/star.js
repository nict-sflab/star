const authStar = () => {
  return new Promise((resolve, reject) => {
    const onMessage = (e) => {
      if (e.source !== window) return;
      if (e.data?.type !== "STAR_RESULT") return;

      console.log("star: ", e.data);
      resolve(e.data);
    };

    window.addEventListener("message", onMessage, { once: true });
    window.postMessage({ type: "STAR", payload: {} }, "*");
  });
};
