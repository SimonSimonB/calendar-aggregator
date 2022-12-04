import * as React from "react";
import { Routes, Route, Outlet, Link } from "react-router-dom";
import Debug from "./Debug";
import TopicEvents from "./TopicEvents";

export default function App() {
  return (
    <div>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<TopicEvents />} />
          <Route path="debug" element={<Debug />} />
        </Route>
      </Routes>
    </div>
  );
}

function Layout() {
  return (
    <div>
      <nav>
        <ul>
          <li>
            <Link to="/">Home</Link>
          </li>
          <li>
            <Link to="/debug">Debug</Link>
          </li>
        </ul>
      </nav>

      <hr />

      <Outlet />
    </div>
  );
}
