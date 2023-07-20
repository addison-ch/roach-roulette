import { useEffect, useRef, useState } from "react";

const useWebSocket = (socketUrl: any, shouldConnect: any) => {
  const socketRef = useRef<WebSocket | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [message, setMessage] = useState(null);
  const [socketError, setSocketError] = useState<string | null>(null);

  useEffect(() => {
    console.log("chilling");
    if (shouldConnect) {
      console.log("Connecting to socket");
      const newSocket = new WebSocket(socketUrl);

      newSocket.onopen = () => {
        setIsConnected(true);
        console.log("WebSocket connection established");
      };

      newSocket.onmessage = (event) => {
        setMessage(event.data);
      };

      newSocket.onclose = () => {
        setIsConnected(false);
        console.log("WebSocket connection closed");
      };

      newSocket.onerror = (event) => {
        setSocketError("WebSocket connection error");
        console.error("WebSocket connection error:", event);
      };

      socketRef.current = newSocket;

      return () => {
        newSocket.close();
      };
    }
  }, [socketUrl, shouldConnect]);

  const sendWebSocketMessage = (message: any) => {
    if (socketRef.current && socketRef.current.readyState === WebSocket.OPEN) {
      socketRef.current.send(message);
      console.log("Sent message:", message);
    } else {
      console.log("WebSocket connection not open");
    }
  };

  return { isConnected, message, socketError, sendWebSocketMessage };
};

export default useWebSocket;
