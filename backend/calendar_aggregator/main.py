import uvicorn

from .app import App
from .event_fetching.cached_event_fetcher import CachedEventFetcher
from .event_fetching.event_extraction.rule_based_extractor import RuleBasedExtractor
from .event_fetching.event_fetcher import EventFetcher
from .event_fetching.html_fetching.html_fetcher import HTMLFetcher
from .settings import settings

if __name__ == "__main__":
    app = App(
        event_fetcher=CachedEventFetcher(
            EventFetcher(
                event_extractor=RuleBasedExtractor(),
                html_fetcher=HTMLFetcher(),
            ),
            cache_expiration_s=settings.event_cache_expiration_s,
        ),
        frontend_path=settings.frontend_path,
    )
    uvicorn.run(app, host="0.0.0.0", port=settings.port)
