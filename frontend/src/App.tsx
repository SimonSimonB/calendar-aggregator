import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';

async function getEvents(calendarUrl: string) {
  const response = await fetch('http://127.0.0.1:8000/api/events', {
    method: 'post',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({'urls': [calendarUrl]}),
  });

  return response.json();
}

function EventTable(props: any) {
  return (
    <div>
      <div>
        {props.events.length}
      </div>
      <table>
        <thead>
          <th>Date</th>
          <th>What</th>
        </thead>
        <tbody>
          {props.events.map((event: any) => 
            <tr>
              <td>{JSON.stringify(event.time.start.NaiveDate.date)}</td>
              <td>{event.text}</td>
            </tr>)}
          </tbody>
      </table>
    </div>
  )
}

function App() {
  let [events, setEvents] = useState([]);
  let [inputUrl, setInputUrl] = useState('');

  function handleClick() {
    getEvents(inputUrl).then(events => setEvents(events));
  }

  return (
    <div className="App">
      <input onChange={e => setInputUrl(e.target.value)}></input>
      <button onClick={handleClick}>Extract events</button>
      <EventTable events={events} />
    </div>
  );
}

export default App;
