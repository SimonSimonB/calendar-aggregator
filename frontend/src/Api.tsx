import { Event, Topic } from "./Common";

export async function getEventsForUrl(calendarUrl: string) {
  const response = await fetch(
    'http://127.0.0.1:8000/api/events?' + new URLSearchParams({'url': calendarUrl}),
    {
      method: 'get',
      headers: {
        'Accept': 'application/json',
      }
    },
  );

  return await responseToEvents(response);
}

export async function getEventsForTopic(topicId: number) {
  const response = await fetch(
    'http://127.0.0.1:8000/api/events?' + new URLSearchParams({'topic_id': String(topicId)}),
    {
      method: 'get',
      headers: {
        'Accept': 'application/json',
      }
    },
  );

  return await responseToEvents(response);
}

export async function getAllTopics(): Promise<Topic[]> {
  const response = await fetch(
    'http://127.0.0.1:8000/api/topics',
    {
      method: 'get',
      headers: {
        'Accept': 'application/json',
      }
    },
  );

  return await response.json();
}

async function responseToEvents(response: Response) {
  const json = await response.json();
  let events = new Map<string, Event[]>();
  for(const [url, eventsForUrl] of Object.entries<any>(json)) {
    events.set(
      url, 
      eventsForUrl.map((eventFromApi: any) => {
        return {
          text: eventFromApi.text,
          date: new Date(eventFromApi.date),
        };
      })
    )
  }
  return events;
}