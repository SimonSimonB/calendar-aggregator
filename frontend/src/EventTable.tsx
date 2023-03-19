import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableRow from "@mui/material/TableRow";
import { Event, EventWithUrl } from "./Common";
import './EventTable.css';

function EventTable(props: { events: Map<string, Event[]> }) {
  let allEvents: Array<EventWithUrl> = Array.from(props.events.entries())
    .map(([url, events]) => events.map<EventWithUrl>((event) => { return { url: url, event: event }; }))
    .flat()
    .flat();
  allEvents.sort((eventWithUrl1, eventWithUrl2) => eventWithUrl1.event.date < eventWithUrl2.event.date ? -1 : 1)
  return (
    <Table>
      <TableBody>
        {allEvents.map((eventWithUrl) =>
          <TableRow>
            <TableCell>
              {`${eventWithUrl.event.date.getMonth() + 1}/${eventWithUrl.event.date.getDate()}`}
            </TableCell>
            <TableCell>
              <div>
                <span>{eventWithUrl.event.text}</span>
                <span className="eventtable__url">{extractDomain(eventWithUrl.url)}</span>
              </div>
            </TableCell>
          </TableRow>
        )}
      </TableBody>
    </Table>
  );
}

function extractDomain(url: string) {
  const match = url.match(/^(?:https?:\/\/)?(?:www\.)?([^/:]+).*$/);
  if (match !== null) {
    return match[1];
  } else {
    return "";
  }
}

export { EventTable };
