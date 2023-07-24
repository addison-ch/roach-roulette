import React, { useState, useEffect, useRef } from "react";
import axios, { AxiosError } from "axios";
import useWebSocket from "../utils/useWebSocket";

// Define the shape of the data you expect to receive from the API
interface ApiResponse {
  // Add the properties you expect to receive
  id: number;
  name: string;
  // etc...
}
const RoomJoin: React.FC = () => {
  const [joiningRoom, setJoiningRoom] = useState<boolean>(false);
  const [data, setData] = useState<ApiResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [roomCode, setRoomCode] = useState<string>("");
  const [users, setUsers] = useState<string[]>([]);
  const [isButtonHidden, setButtonHidden] = useState(false);
  const ws = useRef<WebSocket | null>(null);

  const socketUrl = `ws://127.0.0.1:3005/start_connection/${roomCode}`;
  const { isConnected, message, socketError, sendWebSocketMessage } =
    useWebSocket(socketUrl, joiningRoom);

  const joinRoom = (code: string) => {
    setButtonHidden(true);
    setJoiningRoom(true);
    setRoomCode(code);
  };

  // Handle received messages
  useEffect(() => {
    if (message != null) {
      console.log("Received message:", message);
      let data = JSON.parse(message);

      if (data.type == "player_list") {
        setUsers(data.users);
        console.log(users);
      } else if (data.type == "disconnect") {
        console.log(data.msg);
      } else if (data.type == "join") {
        console.log(data.msg);
      } else if (data.type == "welcome") {
        console.log(data.msg);
      }
    }
  }, [message]);

  const handlePing = async () => {
    const message = "ping";
    sendWebSocketMessage(message);
  };

  const handlePlayers = async () => {
    const message = "players";
    sendWebSocketMessage(message);
  };

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setRoomCode(event.target.value);
  };

  return (
    <div>
      <div>
        {!isButtonHidden && (
          <>
            <input type="text" value={roomCode} onChange={handleChange} />
            <button onClick={() => joinRoom(roomCode)}>JOIN</button>
          </>
        )}
      </div>
      <button onClick={handlePing}>PING</button>
      <button onClick={handlePlayers}>List players</button>
      {error && <p>Error: {error}</p>}
      <div>
        {users.map((user, index) => (
          <p key={index}>{user}</p>
        ))}
      </div>
    </div>
  );
};

export default RoomJoin;
