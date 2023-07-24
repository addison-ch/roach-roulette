import React, { useState, useEffect, useRef } from 'react';
import axios, { AxiosError } from 'axios';

// Define the shape of the data you expect to receive from the API
interface ApiResponse {
    // Add the properties you expect to receive
    id: number;
    name: string;
    // etc...
}
const RoomJoin: React.FC = () => {

    const [data, setData] = useState<ApiResponse | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [roomCode, setRoomCode] = useState<string>("");
    const [users, setUsers] = useState<string[]>([]);
    const ws = useRef<WebSocket | null>(null);
    const [isButtonHidden, setButtonHidden] = useState(false);
    const [isWebSocketConnected, setWebSocketConnected] = useState(false);


    const joinRoom = (code: string) => {
        // Create a WebSocket instance
        ws.current = new WebSocket(`ws://127.0.0.1:3005/start_connection/${code}`);

        // Add an onOpen event listener to the WebSocket instance
        ws.current.onopen = () => {
            console.log('WebSocket connection established');
            setWebSocketConnected(true);
        };

        // Add an onMessage event listener to the WebSocket instance
        ws.current.onmessage = (event) => {
            let data = JSON.parse(event.data);
            if (data.type == "player_list") {
                setUsers(data.users);
                console.log(users)
            } else if (data.type == "disconnect") {
                console.log(data.msg)
            } else if (data.type == "join") {
                console.log(data.msg)
            } else if (data.type == "welcome") {
                console.log(data.msg)
            }

            // console.log('Received message:', data);
        };

        // Add an onClose event listener to the WebSocket instance
        ws.current.onclose = () => {
            console.log('WebSocket connection closed');
        };

        ws.current.onerror = (event) => {
            setError('WebSocket connection error');
            console.error('WebSocket connection error:', event);
        };
    }

    const handleClick = async () => {
        if (ws.current && ws.current.readyState === WebSocket.OPEN) {
            const message = 'ping';
            ws.current.send(message);
            console.log('Sent message:', message);
            setButtonHidden(true);
        } else {
            console.log('WebSocket connection not open');
        }
    };


    const handlePlayers = async () => {
        if (ws.current && ws.current.readyState === WebSocket.OPEN) {
            const message = 'players';
            ws.current.send(message);
            console.log('Sent message:', message);
        } else {
            console.log('WebSocket connection not open');
        }
    };

    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setRoomCode(event.target.value);
    };

    return (
        <div>
            <div>
            {!isButtonHidden && !isWebSocketConnected && (
                <><input type="text" value={roomCode} onChange={handleChange} /><button onClick={() => joinRoom(roomCode)}>JOIN</button></>
                )}
            </div>
            <button onClick={handleClick}>
                PING
            </button>
            <button onClick={handlePlayers}>
                List players
            </button>
            {error && <p>Error: {error}</p>}
            <div>
                {users.map((user, index) => (
                    <p key={index}>{user}</p>
                ))}
            </div>
        </div>
    )
}

export default RoomJoin