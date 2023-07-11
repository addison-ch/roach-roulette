import React, { useState, useEffect, useRef } from 'react';
import axios, { AxiosError } from 'axios';

interface ApiResponse {
    code: string;
}

const RoomCreate: React.FC = () => {
    const ws = useRef<WebSocket | null>(null);
    const [data, setData] = useState<ApiResponse | null>(null);
    const [error, setError] = useState<string | null>(null);

    const ws_start = (code: string) => {
        // Create a WebSocket instance
        if (code === null) {
            throw new Error('Room code cant be null')
        }
        console.log(code)
        ws.current = new WebSocket(`ws://127.0.0.1:3005/start_connection/${code}`);

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

        ws.current.onerror = (event) => {
            setError('WebSocket connection error');
            console.error('WebSocket connection error:', event);
        };


        // Clean up the WebSocket instance on unmount
        return () => {
            if (ws.current) {
                ws.current.close();
            }
        };
    }

    const handleClick = async () => {
        try {
            const res = await axios.get<ApiResponse>('http://127.0.0.1:3005/create_room');
            console.log(res.data.code)
            ws_start(res.data.code);
            setData(res.data);
        } catch (error) {
            const axiosError = error as AxiosError;
            // Handle the error based on the response from the server
            if (axiosError.response) {
                setError(`Request failed with status code ${axiosError.response.status}`);
            } else if (axiosError.request) {
                setError('No response was received');
            } else {
                setError(axiosError.message);
            }
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

export default RoomCreate