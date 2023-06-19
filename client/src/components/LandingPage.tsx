import React from 'react'
import { Link } from "react-router-dom";
import '../styles/LandingPage.css'

const LandingPage: React.FC = () => {
    return (
        <div className="flex flex-col items-center justify-evenly max-h-screen my-8 ">
            <h1 className="flex-1 p-4 flex-grow ">Bug Bluff</h1>
            <img
                src="roach.jpg"
                alt="Cock Roach Poker"
                className="flex-1  p-4 flex-grow"
            />
            <div className="flex-1 w-5/6">
                <div className="grid grid-cols-2 gap-6 p-4 w-full">
                    <Link className="text-center border-primary border rounded-md border-solid bg-secondary p-3" to="/create" >Create Room</Link>
                    <Link className="text-center border-primary border rounded-md border-solid bg-secondary p-3" to="/join" >Join Room</Link>
                </div>
            </div>
        </div>
    )
}

export default LandingPage