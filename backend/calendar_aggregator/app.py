from pathlib import Path
from typing import Any

from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from starlette.responses import FileResponse

from .api_router import Router
from .event_extraction.interfaces import AbstractEventExtractor
from .html_fetching.interfaces import AbstractHTMLFetcher


class App(FastAPI):
    def __init__(
        self,
        html_fetcher: AbstractHTMLFetcher,
        event_extractor: AbstractEventExtractor,
        frontend_path: Path,
        *args: Any,
        **kwargs: Any
    ) -> None:  # types: ignore
        super().__init__(*args, **kwargs)
        self.include_router(Router(html_fetcher, event_extractor), prefix="/api")

        @self.get("/")
        def get_index() -> FileResponse:
            return FileResponse(frontend_path / "index.html")

        self.mount(
            "/",
            StaticFiles(directory=frontend_path),
            name="static",
        )
