import { Event } from "./Common";

export async function getEventsForUrls(calendarUrls: string[]) {
  const response = await fetch(
    'http://127.0.0.1:8000/api/events?' + new URLSearchParams({ 'urls': JSON.stringify(calendarUrls) }),
    {
      method: 'get',
      headers: {
        'Accept': 'application/json',
      }
    },
  );

  return await responseToEvents(response);
}

async function responseToEvents(response: Response) {
  const json = await response.json();
  let events = new Map<string, Event[]>();
  for (const [url, eventsForUrl] of Object.entries<any>(json)) {
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