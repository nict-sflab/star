const starAuth = () => {
    return new Promise((resolve, reject) => {
        chrome.runtime.sendMessage({  }, function (response) {
            resolve(response)
        })
    })

}


(async ()=> {
    const data = await starAuth()
    console.log(data)
})()