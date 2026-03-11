const authStar = () =>
  new Promise((resolve) => {
    const requestId = crypto.randomUUID();

    const onMessage = (e) => {
      if (e.source !== window) return;
      if (e.data?.type !== "STAR_RESULT") return;
      if (e.data?.requestId !== requestId) return;

      window.removeEventListener("message", onMessage);
      resolve(e.data);
    };

    window.addEventListener("message", onMessage);

    window.postMessage(
      {
        type: "STAR",
        requestId,
        payload: {},
      },
      "*"
    );
  });