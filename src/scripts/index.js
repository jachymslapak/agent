const { invoke } = window.__TAURI__.tauri;
function swiftchsite() {
    window.location.href = "/sites/main.html";
};
invoke('get_updates').then((message) => {
    if (message == "done") {
        setTimeout(swiftchsite, 3500); 
    } else if (message == "!download"){
        invoke('disconnected');
        setTimeout(swiftchsite, 4200); 
    } 
    else {
        document.getElementsByClassName("footer")[0].innerHTML = "failed to get output"
    }
},setTimeout(() => {
    document.getElementsByClassName("footer")[0].innerHTML = "network timeout";
    invoke('disconnected');
    setTimeout(swiftchsite, 1000); 
}, 10000));