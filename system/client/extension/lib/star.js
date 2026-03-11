let hasCalled = false;

const authStar = () => {
  return new Promise((resolve, reject) => {
    const onMessage = (e) => {
      if (e.source !== window) return;
      if (e.data?.type !== "STAR_RESULT") return;
      console.log("star: ", e.data);
      resolve(e.data);
    };

    if (!hasCalled) {
      window.addEventListener("message", onMessage);
      hasCalled = true;
    }

    window.postMessage({ type: "STAR", payload: {} }, "*");
  });
};
