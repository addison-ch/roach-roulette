import React, { useState } from 'react';
import axios, { AxiosError } from 'axios';

// Define the shape of the data you expect to receive from the API
interface ApiResponse {
    // Add the properties you expect to receive
    id: number;
    name: string;
    // etc...
}
const RoomCreate: React.FC = () => {

    const [data, setData] = useState<ApiResponse | null>(null);
    const [error, setError] = useState<string | null>(null);

    const handleClick = async () => {
        try {
            const response = await axios.get<ApiResponse>('http://127.0.0.1:3005/create_room');
            setData(response.data);
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