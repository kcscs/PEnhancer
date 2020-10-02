function onResponse(response) {
  console.log("Received: " + response["result"]);
  
}

function onError(error) {
  console.log(`Error: ${error}`);
}

function onDisconnected(){
  console.log("Penhance disconnected - probably killed");
  isOn = false;
}

function startPEnhance(){
  console.log("Connecting to PEnhance");
  port = browser.runtime.connectNative("ping_pong");
  port.onMessage.addListener(onResponse);
  port.onDisconnect.addListener(onDisconnected);
  isOn = true;
}

function closePEnhance(){
  console.log("Disconnecting PEnhance");
  port.disconnect();
  isOn = false;
}

var isOn = false;
var port = null;

/*
On a click on the browser action, send the app a message.
*/
browser.browserAction.onClicked.addListener(() => {
  isOn = !isOn;

  if(isOn){
    startPEnhance();
  } else {
    closePEnhance();
  }

  /*
  console.log("IsOn: " + isOn);
  console.log("Sending:  ping");
  var sending = browser.runtime.sendNativeMessage(
    "ping_pong",
    "ping");
  sending.then(onResponse, onError);*/
});




function tick() {
  if (!isOn)
    return;


  browser.tabs.query({ active: true, windowId: browser.windows.WINDOW_ID_CURRENT })
    .then(tabs => browser.tabs.get(tabs[0].id))
    .then(tab => {
      console.info(tab);
      console.log("sending: " + tab.title);
      /*var sending = browser.runtime.sendNativeMessage(
        "ping_pong",
        tab.title
      );
      sending.then(onResponse, onError);*/
      port.postMessage(tab.title);
    });
}
window.setInterval(tick, 5000);


startPEnhance();