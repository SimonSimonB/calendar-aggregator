import Autocomplete from '@mui/material/Autocomplete';
import Chip from '@mui/material/Chip';
import TextField from '@mui/material/TextField';
import { useEffect, useState } from 'react';
import { getEventsForUrls } from './Api';
import './App.css';
import { Event } from './Common';
import { EventTable } from './EventTable';

const CALENDAR_URL_LOCAL_STORAGE_KEY = 'calendar-url';

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

function App() {
  let [events, setEvents] = useState<Map<string, Event[]>>(new Map<string, Event[]>());
  const calendarUrlsInitialValue = JSON.parse(localStorage.getItem(CALENDAR_URL_LOCAL_STORAGE_KEY) ?? "[]");
  let [calendarUrls, setCalendarUrls] = useState<string[]>(calendarUrlsInitialValue);
  useEffect(getAndShowEvents, [calendarUrls]);
  useEffect(() => localStorage.setItem('calendar-url', JSON.stringify(calendarUrls)), [calendarUrls]);

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

export { App };
