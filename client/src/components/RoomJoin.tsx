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
    const ws = useRef<WebSocket | null>(null);

    useEffect(() => {
        // Create a WebSocket instance
        ws.current = new WebSocket(`ws://127.0.0.1:3005/start_connection/${uuid}/${int32}`);

        // Add an onOpen event listener to the WebSocket instance
        ws.current.onopen = () => {
            console.log('WebSocket connection established');
        };

        // Add an onMessage event listener to the WebSocket instance
        ws.current.onmessage = (event) => {
            console.log(event)
            console.log('Received message:', event.data);
        };

        // Add an onClose event listener to the WebSocket instance
        ws.current.onclose = () => {
            console.log('WebSocket connection closed');
        };

        // Clean up the WebSocket instance on unmount
        return () => {
            if (ws.current) {
                ws.current.close();
            }
        };
    }, []);
    const uuid = require('uuid').v4();
    const int32 = Math.floor(Math.random() * 100) + 1;

    const handleClick = async () => {
        if (ws.current && ws.current.readyState === WebSocket.OPEN) {
            const message = 'Hello, WebSocket!';
            ws.current.send(message);
            console.log('Sent message:', message);
        } else {
            console.log('WebSocket connection not open');
        }
    };

    return (
        <div>
            <button onClick={handleClick}>
                Click me
            </button>
            {error && <p>Error: {error}</p>}
            {data && <p>Data: {JSON.stringify(data)}</p>}
        </div>
    )
}

export default RoomJoin