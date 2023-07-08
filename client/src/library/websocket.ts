import { useEffect, useRef } from "react";

type OnMessageCallback = (data: any) => void;

export default function useWebsocket(onMessage: OnMessageCallback) {
  const ws = useRef<WebSocket | null>(null);

  useEffect(() => {
    if (ws.current !== null) return;
    const wsUri = "ws://localhost:8080/ws";
    ws.current = new WebSocket(wsUri);
    ws.current.onopen = () => console.log("ws opened");
    ws.current.onclose = () => console.log("ws closed");
    const wsCurrent = ws.current;
    return () => {
      wsCurrent.close();
    };
  }, []);

  useEffect(() => {
    if (!ws.current) return;
    ws.current.onmessage = (e) => {
      onMessage(e.data);
    };
  }, []);

  const sendMessage = (msg: string) => {
    if (!ws.current) return;
    ws.current.send(msg);
  };

  return sendMessage;
}
