import uvicorn

from .app import App
from .event_extraction.rule_based_extractor import RuleBasedExtractor
from .html_fetching.html_fetcher import HTMLFetcher
from .settings import settings

if __name__ == "__main__":
    app = App(
        html_fetcher=HTMLFetcher(),
        event_extractor=RuleBasedExtractor(),
        frontend_path=settings.frontend_path,
    )
    uvicorn.run(app, host="0.0.0.0")
