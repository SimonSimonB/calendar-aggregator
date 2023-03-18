import uvicorn

from .app import App
from .html_fetching.html_fetcher import HTMLFetcher
from .event_extraction.rule_based_extractor import RuleBasedExtractor

if __name__ == "__main__":
    app = App(html_fetcher=HTMLFetcher(), event_extractor=RuleBasedExtractor())
    uvicorn.run(app, host="0.0.0.0")
