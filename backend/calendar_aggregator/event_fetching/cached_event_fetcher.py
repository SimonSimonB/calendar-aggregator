import datetime
from typing import Dict, Generic, List, Tuple, TypeVar

from ..models import Event
from .interfaces import AbstractEventFetcher


class CachedEventFetcher(AbstractEventFetcher):
    """An event fetcher that caches fetched events, re-fetching only after a specified time period has passed."""

    def __init__(
        self, event_fetcher: AbstractEventFetcher, cache_expiration_s: float
    ) -> None:
        super().__init__()
        self._event_fetcher = event_fetcher
        self._cache: _Cache[str, List[Event]] = _Cache(expiration_s=cache_expiration_s)

    async def fetch(self, url: str) -> List[Event]:
        try:
            return self._cache.get(url)
        except KeyError:
            value = await self._event_fetcher.fetch(url)
            self._cache.set(url, value)
            return value


_K = TypeVar("_K")
_V = TypeVar("_V")


class _Cache(Generic[_K, _V]):
    def __init__(self, expiration_s: float) -> None:
        self._cache: Dict[_K, Tuple[_V, datetime.datetime]] = {}
        self._expiration_s: float = expiration_s

    def set(self, key: _K, value: _V) -> None:
        self._cache[key] = (value, datetime.datetime.now())

    def get(self, key: _K) -> _V:
        value, last_update = self._cache.get(key, (None, None))
        if last_update and datetime.datetime.now() > last_update + datetime.timedelta(
            seconds=self._expiration_s
        ):
            value = None

        if value is None:
            raise KeyError

        return value
