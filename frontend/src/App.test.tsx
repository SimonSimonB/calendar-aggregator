import React from 'react';
import { render, screen } from '@testing-library/react';
import { EventTable, Event } from './App';

describe('table component', () => {
  test('renders total event count', () => {
    let events = new Map<string, Event[]>([
      ['url1', [
        {text: 'Concert', time: {NaiveDate: '2022-08-07'}},
        {text: 'Concert', time: {NaiveDate: '2022-08-07'}},
      ]],
      ['url2', []],
      ['url3', []],
      ['url4', [
        {text: 'Concert', time: {NaiveDate: '2022-08-07'}},
        {text: 'Concert', time: {NaiveDate: '2022-08-07'}}
      ]],
    ]);
    render(<EventTable events={events}/>);
    const totalNumberOfEvents = screen.getByText('4');
    expect(totalNumberOfEvents).toBeInTheDocument();
  });

  test('renders event text', () => {
    let events = new Map<string, Event[]>([
      ['url1', [{text: 'Concert1', time: {NaiveDate: '2022-08-07'}}, {text: 'Concert2', time: {NaiveDate: '2022-08-07'}}]],
    ]);
    render(<EventTable events={events}/>);
    expect(screen.getByText('Concert1')).toBeInTheDocument();
    expect(screen.getByText('Concert2')).toBeInTheDocument();
  });
});
