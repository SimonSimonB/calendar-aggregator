import { render, screen } from '@testing-library/react';
import { Event } from './Common';
import { EventTable } from './EventTable';

describe('table component', () => {
  test('renders event text', () => {
    let events = new Map<string, Event[]>([
      ['url1', [
        { text: 'Concert1', date: new Date('2022-08-07') },
        { text: 'Concert2', date: new Date('2022-08-07') },
      ]],
    ]);
    render(<EventTable events={events} />);
    expect(screen.getByText('Concert1')).toBeInTheDocument();
    expect(screen.getByText('Concert2')).toBeInTheDocument();
  });
});
