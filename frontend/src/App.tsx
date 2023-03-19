import Autocomplete from '@mui/material/Autocomplete';
import Chip from '@mui/material/Chip';
import TextField from '@mui/material/TextField';
import { useEffect, useState } from 'react';
import { getEventsForUrls } from './Api';
import './App.css';
import { Event } from './Common';
import { EventTable } from './EventTable';

const CALENDAR_URL_LOCAL_STORAGE_KEY = 'calendar-urls';

function App() {
  let [events, setEvents] = useState<Map<string, Event[]>>(new Map<string, Event[]>());
  const calendarUrlsInitialValue = getInitialCalendarUrls();
  let [calendarUrls, setCalendarUrls] = useState<string[]>(calendarUrlsInitialValue);
  useEffect(getAndShowEvents, [calendarUrls]);

  // When the URLs inputted by the user change, write the new list of URLs into the window location URL 
  // and into local storage.
  useEffect(() => localStorage.setItem(CALENDAR_URL_LOCAL_STORAGE_KEY, JSON.stringify(calendarUrls)), [calendarUrls]);
  useEffect(() => setWindowLocation(calendarUrls), [calendarUrls]);

  function getAndShowEvents() {
    if (calendarUrls.length > 0) {
      getEventsForUrls(calendarUrls).then(events => setEvents(events));
    }
  }

  return (
    <div>
      <Input
        setSelectedOptions={
          (newCalendarUrls: string[]) => {
            setCalendarUrls(newCalendarUrls);
          }
        }
        selectedOptions={calendarUrls}
      />
      <EventTable events={events} />
    </div>
  );
}

function Input(props: { selectedOptions: string[], setSelectedOptions: (selectedOptions: string[]) => void }) {
  return (
    <Autocomplete
      options={[]}
      multiple={true}
      value={props.selectedOptions}
      open={false}
      freeSolo
      onChange={(_, newValue: string[]) => {
        props.setSelectedOptions(newValue);
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
          label="event calendar URLs"
          placeholder="paste URLs here"
        />
      )}
    />
  )
}

function getInitialCalendarUrls() {
  const locationWithoutHash = window.location.hash ? window.location.hash.slice(1) : '';
  const urlsInLocation = decodeURIComponent(locationWithoutHash);
  if (urlsInLocation.length > 0) {
    return JSON.parse(urlsInLocation);
  } else {
    return JSON.parse(localStorage.getItem(CALENDAR_URL_LOCAL_STORAGE_KEY) ?? '[]');
  }
}

function setWindowLocation(calendarUrls: string[]) {
  if (calendarUrls.length > 0) {
    window.location.hash = encodeURIComponent(JSON.stringify(calendarUrls));
  } else {
    window.location.hash = '';
  }
}

export { App };
