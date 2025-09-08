import { invoke } from "@tauri-apps/api/core";
import { Routes, Route, Link } from "react-router-dom";
import "./App.css";

import Home from "./component/Home";
import Signup from "./component/Signup";
import Login from "./component/Login";

function App() {
  return (
    <div>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/signup" element={<Signup />} />
        <Route path="/login" element={<Login />} />
      </Routes>
    </div>
  );
}

export default App;
