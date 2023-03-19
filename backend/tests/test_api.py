import datetime
import json
from pathlib import Path
from typing import List

from calendar_aggregator.app import App
from calendar_aggregator.event_fetching.interfaces import AbstractEventFetcher
from calendar_aggregator.models import Event
from fastapi.testclient import TestClient


class TestGetEvents:
    def test_only_returns_events_after_today(self) -> None:
        events = [
            Event(
                datetime.datetime.today() - datetime.timedelta(days=1),
                "Yesterday's Event",
            ),
            Event(
                datetime.datetime.today(),
                "Today's Event",
            ),
            Event(
                datetime.datetime.today() + datetime.timedelta(days=1),
                "Tomorrow's Event",
            ),
        ]
        test_client = TestClient(
            App(
                event_fetcher=FakeEventFetcher(events),
                frontend_path=Path("/tmp"),
            )
        )

        response = test_client.get(
            "/api/events", params={"urls": json.dumps(["http://does-not-exist"])}
        )

        assert response.status_code == 200
        returned_events = response.json()["http://does-not-exist"]
        assert len(returned_events) == 2


class FakeEventFetcher(AbstractEventFetcher):
    def __init__(self, events: List[Event]) -> None:
        self._events = events

    async def fetch(self, url: str) -> List[Event]:
        return self._events
