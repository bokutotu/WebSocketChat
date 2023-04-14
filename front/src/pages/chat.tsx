import { useEffect, useState } from "react";

export default function chat() {
  const [messages, setMessages] = useState<string[]>([]);
  const [socket, setSocket] = useState<WebSocket | null>(null);

  useEffect(() => {
    const s = new WebSocket("ws://localhost:1919/ws");
    setSocket(s);
  }, []);

  if (socket) {
    socket.addEventListener('message', function (event) {
      setMessages([...messages, event.data]);
      console.log('Message from server ', event.data);
    });
  }

  const [ message, setMessage ] = useState<string>('');

  return (
    <div>
      chat
      <input
        type="text"
        onChange={(e) => setMessage(e.target.value)}
        value={message}
      />
      <button
        onClick={() => {
          if (socket) {
          socket.send(message);
          }
          setMessage('');
        }}
      >
      送信
      </button>

      {messages.map((message) => (
        <div>{message}</div>
      ))}
      
    </div>
  );
}
