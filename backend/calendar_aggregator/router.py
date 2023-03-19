import asyncio
import datetime
import json
from typing import Dict, List

from fastapi import APIRouter

from .event_fetching.interfaces import AbstractEventFetcher
from .models import Event


class Router(APIRouter):
    def __init__(
        self,
        event_fetcher: AbstractEventFetcher,
    ) -> None:
        super().__init__()

        @self.get("/events")
        async def get_events(urls: str) -> Dict[str, List[Event]]:
            urls_list: List[str] = json.loads(urls)

            events = await asyncio.gather(
                *(event_fetcher.fetch(url) for url in urls_list)
            )

            result: Dict[str, List[Event]] = {}
            for url, events in zip(urls_list, events):
                # Only return events that will happen today or in the future.
                result[url] = [
                    event
                    for event in events
                    if event.date
                    >= datetime.datetime.today().replace(hour=0, minute=0, second=0)
                ]

            return result
