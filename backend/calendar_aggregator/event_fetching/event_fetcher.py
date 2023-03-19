from typing import List

from ..models import Event
from .event_extraction.interfaces import AbstractEventExtractor
from .html_fetching.interfaces import AbstractHTMLFetcher
from .interfaces import AbstractEventFetcher


class EventFetcher(AbstractEventFetcher):
    def __init__(
        self, html_fetcher: AbstractHTMLFetcher, event_extractor: AbstractEventExtractor
    ) -> None:
        self._html_fetcher = html_fetcher
        self._event_extractor = event_extractor

    async def fetch(self, url: str) -> List[Event]:
        return self._event_extractor.extract(await self._html_fetcher.fetch(url))
