import React, { useEffect, useState } from 'react';
import './App.css';
import Autocomplete from '@mui/material/Autocomplete';
import TextField from '@mui/material/TextField';
import Chip from '@mui/material/Chip';
import EventTable from './EventTable';
import { getEventsForUrl } from './Api';

const CALENDAR_URL_LOCAL_STORAGE_KEY = 'calendar-url';

export interface Event {
  text: string,
  date: Date,
}

function Input(props: {selectedOption: any, setSelectedOption: any}) {
  return (
    <Autocomplete
      options={[]}
      value={props.selectedOption}
      open={false}
      freeSolo
      onChange={(_, newValue: string | null) => {
        console.log(newValue);
        props.setSelectedOption(newValue);
      }}
      renderTags={(value: readonly string[], getTagProps) =>
        value.map((option: string, index: number) => (
          <Chip variant="outlined" label={option} {...getTagProps({ index })} />
        ))
      }
      renderInput={(params) => (
        <TextField
          {...params}
          variant="filled"
          label="calendar URLs"
          placeholder="paste URLs here"
        />
      )}
    />
  )
}

function Debug() {
  let [events, setEvents] = useState<Map<string, Event[]>>(new Map<string, Event[]>());
  const calendarUrlInitialValue = JSON.parse(localStorage.getItem(CALENDAR_URL_LOCAL_STORAGE_KEY) ?? "[]");
  let [calendarUrl, setCalendarUrl] = useState<string | null>(calendarUrlInitialValue);
  useEffect(getAndShowEvents, [calendarUrl]);
  useEffect(() => localStorage.setItem('calendar-url', JSON.stringify(calendarUrl)), [calendarUrl]);

  function getAndShowEvents() {
    if(calendarUrl != null) {
      getEventsForUrl(calendarUrl).then(events => setEvents(events));
    }
  }

  return (
    <div className="App">
      <Input 
        setSelectedOption={
          (newCalendarUrl: string | null) => {
            setCalendarUrl(newCalendarUrl); 
            getAndShowEvents();
          }
        }
        selectedOption={calendarUrl}
      />
      <EventTable events={events} />
    </div>
  );
}

export default Debug;
export { EventTable };
