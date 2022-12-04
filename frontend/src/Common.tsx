export interface Event {
  text: string,
  date: Date,
}

export interface EventWithUrl {
  event: Event,
  url: string,
}

export interface Topic {
  id: number,
  name: string,
}
