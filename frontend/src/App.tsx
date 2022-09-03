import React, { useState } from 'react';
import './App.css';

export interface Event {
  text: string,
  time: any,
}

async function getEvents(calendarUrl: string) {
  const response = await fetch('http://127.0.0.1:8000/api/events', {
    method: 'post',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({'urls': [calendarUrl]}),
  });

  const json = await response.json();
  return new Map<string, Event[]>(Object.entries(json));
}

interface EventWithUrl {
  event: Event,
  url: string,
}

function EventTable(props: {events: Map<string, Event[]>}) {
  let allEvents: Array<EventWithUrl> = Array.from(props.events.entries())
    .map(([url, events]) => events.map<EventWithUrl>((event) => {return {url: url, event: event};}))
    .flat()
    .flat();
  allEvents.sort((eventWithUrl1, eventWithUrl2) => eventWithUrl1.event.time > eventWithUrl2.event.time ? 1 : -1)
  return (
    <div>
      <div>
        {Array.from(props.events.values())
          .map((eventsForUrl) => eventsForUrl.length)
          .reduce((acc: number, currentValue: number) => acc + currentValue, 0)}
      </div>
      <table>
        <thead>
          <tr>
            <th>Date</th>
            <th>What</th>
          </tr>
        </thead>
        <tbody>
          {allEvents.map((eventWithUrl) => 
            <tr>
              <td>{JSON.stringify(eventWithUrl.event.time.NaiveDate.date)}</td>
              <td>{eventWithUrl.event.text}</td>
            </tr>)}
          </tbody>
      </table>
    </div>
  )
}

function App() {
  let [events, setEvents] = useState<Map<string, Event[]>>(new Map<string, Event[]>());
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
export { EventTable };
