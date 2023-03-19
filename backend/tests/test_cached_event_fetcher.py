from typing import List

import pytest
from calendar_aggregator.event_fetching.cached_event_fetcher import CachedEventFetcher
from calendar_aggregator.event_fetching.interfaces import AbstractEventFetcher
from calendar_aggregator.models import Event


@pytest.mark.asyncio
async def test_second_request_served_from_cache_if_not_expired() -> None:
    fake_fetcher = FakeEventFetcher()
    cached_fetcher = CachedEventFetcher(fake_fetcher, cache_expiration_s=60)

    await cached_fetcher.fetch("foobar")
    await cached_fetcher.fetch("foobar")

    assert fake_fetcher.num_fetches == 1


@pytest.mark.asyncio
async def test_second_request_not_served_from_cache_if_expired() -> None:
    fake_fetcher = FakeEventFetcher()
    cached_fetcher = CachedEventFetcher(fake_fetcher, cache_expiration_s=0)

    await cached_fetcher.fetch("foobar")
    await cached_fetcher.fetch("foobar")

    assert fake_fetcher.num_fetches == 2


class FakeEventFetcher(AbstractEventFetcher):
    def __init__(self) -> None:
        self.num_fetches = 0

    async def fetch(self, url: str) -> List[Event]:
        self.num_fetches += 1
        return []
