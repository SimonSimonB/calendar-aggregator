import React, { useEffect, useState } from 'react';
import './App.css';
import EventTable from './EventTable';
import { Event, Topic } from './Common';
import { getAllTopics, getEventsForTopic } from './Api';
import Autocomplete from '@mui/material/Autocomplete';
import { TextField } from '@mui/material';

function TopicDropdown(props: {
  allTopics: Topic[], setSelectedTopic: ((topic: Topic | null) => void), selectedOption: Topic | null
}) {
  return (
    <Autocomplete
      disablePortal
      id="combo-box-demo"
      options={props.allTopics}
      sx={{ width: 300 }}
      renderInput={(params) => <TextField {...params} />}
      getOptionLabel={(topic) => topic.name}
      onChange={(_, newValue: Topic | null) => {
        console.log(newValue);
        props.setSelectedTopic(newValue);
      }}
    />
  )
}

function TopicEvents() {
  let [topics, setTopics] = useState<Topic[]>([]);
  let [events, setEvents] = useState<Map<string, Event[]>>(new Map<string, Event[]>());
  let [selectedTopic, setSelectedTopic] = useState<Topic | null>(null);
  useEffect(getAndShowEvents, [selectedTopic]);

  function getAndShowEvents() {
    if(selectedTopic != null) {
      getEventsForTopic(selectedTopic.id).then(events => setEvents(events));
    }
  }
  
  useEffect(() => { getAllTopics().then((topics) => { setTopics(topics); console.log(topics); }); }, []);

  return (
    <div className="event-topics-div">
      <TopicDropdown
        allTopics={topics}
        setSelectedTopic={
          (topic: Topic | null) => {
            setSelectedTopic(topic); 
            getAndShowEvents();
          }
        }
        selectedOption={selectedTopic}
      />
      <EventTable events={events} />
    </div>
  );
}


export default TopicEvents;
