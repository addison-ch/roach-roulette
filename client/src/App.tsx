import { BrowserRouter as Router, Routes, Route } from "react-router-dom";

import LandingPage from "./components/LandingPage";
import RoomCreate from "./components/RoomCreate";
import RoomJoin from "./components/RoomJoin";
import GameBoard from "./components/GameBoard";
import "./App.css";

function App(): JSX.Element {
  return (
    <>
      <div className="flex flex-row items-center justify-center min-h-screen">
        <Router>
          <Routes>
            <Route path="/" element={<LandingPage />} />
            <Route path="/create" element={<RoomCreate />} />
            <Route path="/join" element={<RoomJoin />} />
            <Route path="/game" element={<GameBoard />} />
            <Route path="*" element={<h1>Not Found</h1>} />
          </Routes>
        </Router>
      </div>
    </>
  );
}

export default App;
