import React, { useState, useEffect, useRef } from "react";
import axios, { AxiosError } from "axios";
import useWebSocket from "../utils/useWebSocket";

interface ApiResponse {
  code: string;
}

const RoomCreate: React.FC = () => {
  const [data, setData] = useState<ApiResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [roomCode, setRoomCode] = useState<string>("");

  const socketUrl = `ws://127.0.0.1:3005/start_connection/${roomCode}`;
  const [creatingRoom, setCreatingRoom] = useState<boolean>(false);

  // websocket connection
  const { isConnected, message, socketError, sendWebSocketMessage } =
    useWebSocket(socketUrl, creatingRoom);

  const ws_start = (code: string) => {
    if (code === null) {
      throw new Error("Room code cant be null");
    }
    setCreatingRoom(true);
    setRoomCode(code);
  };

  // Handle received messages
  useEffect(() => {
    if (message) {
      console.log("Received message:", message);
    }
  }, [message]);

  const handleClick = async () => {
    try {
      const res = await axios.get<ApiResponse>(
        "http://127.0.0.1:3005/create_room"
      );
      console.log(res.data.code);
      ws_start(res.data.code);
      setData(res.data);
    } catch (error) {
      const axiosError = error as AxiosError;
      // Handle the error based on the response from the server
      if (axiosError.response) {
        setError(
          `Request failed with status code ${axiosError.response.status}`
        );
      } else if (axiosError.request) {
        setError("No response was received");
      } else {
        setError(axiosError.message);
      }
    }
  };
  return (
    <div>
      <button onClick={handleClick}>Click me</button>
      {error && <p>Error: {error}</p>}
      {data && <p>Data: {JSON.stringify(data)}</p>}
    </div>
  );
};

export default RoomCreate;
