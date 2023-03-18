import asyncio
import datetime
import json
from typing import Dict, List

from fastapi import APIRouter

from .event_extraction.interfaces import AbstractEventExtractor
from .html_fetching.interfaces import AbstractHTMLFetcher
from .models import Event


class Router(APIRouter):
    def __init__(
        self,
        html_fetcher: AbstractHTMLFetcher,
        event_extractor: AbstractEventExtractor,
    ) -> None:
        super().__init__()

        @self.get("/events")
        async def get_events(urls: str) -> Dict[str, List[Event]]:
            urls_list: List[str] = json.loads(urls)

            async def _fetch_events(url: str) -> List[Event]:
                return event_extractor.extract(await html_fetcher.fetch(url))

            events = await asyncio.gather(*(_fetch_events(url) for url in urls_list))

            result: Dict[str, List[Event]] = {}
            for url, events in zip(urls_list, events):
                # Only return events that happen today or in the future.
                result[url] = [
                    event
                    for event in events
                    if event.date
                    >= datetime.datetime.today().replace(hour=0, minute=0, second=0)
                ]

            return result
