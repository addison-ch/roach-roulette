import React from 'react'
import { Link } from "react-router-dom";
import '../styles/LandingPage.css'

const LandingPage: React.FC = () => {
    return (
        <div className="flex flex-col items-center justify-evenly min-h-screen">
            <h1 className="flex-1 bg-gray-200 p-4">Roach Roulette</h1>
            <img
                src="roach.jpg"
                alt="Cock Roach Poker"
                className="flex-1 bg-gray-200 p-4"
            />
            <div className="flex-1 p-4">
                <Link className="" to="/create" >Create Room</Link>
                <Link className="" to="/join" >Join Room</Link>
            </div>
        </div>
    )
}

export default LandingPage