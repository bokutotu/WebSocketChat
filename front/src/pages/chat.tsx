import { useEffect, useState } from "react";

export default function chat() {
  
  const [socket, setSocket] = useState<WebSocket | null>(null);

  useEffect(() => {
    const s = new WebSocket("ws://localhost:1919/ws");
    setSocket(s);
  }, []);
  // const socket = new WebSocket('wss://socketsbay.com/wss/v2/2/demo/');
  if (socket) {
    socket.addEventListener('open', function (event) {
      socket.send('Hello Server!');
    });

    socket.addEventListener('message', function (event) {
        console.log('Message from server ', event.data);
    });


    setTimeout(() => {
        const obj = { hello: "world" };
        const blob = new Blob([JSON.stringify(obj, null, 2)], {
          type: "application/json",
        });
        console.log("Sending blob over websocket");
        socket.send(blob);
    }, 1000);

    setTimeout(() => {
        socket.send('About done here...');
        console.log("Sending close over websocket");
        socket.close(3000, "Crash and Burn!");
    }, 3000);
  }
  return (
    <div>
      chat
    </div>
  );
}
