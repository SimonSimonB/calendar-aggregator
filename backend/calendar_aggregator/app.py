from pathlib import Path
from typing import Any

from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from starlette.responses import FileResponse

from .event_fetching.interfaces import AbstractEventFetcher
from .router import Router


class App(FastAPI):
    def __init__(
        self,
        event_fetcher: AbstractEventFetcher,
        frontend_path: Path,
        *args: Any,
        **kwargs: Any
    ) -> None:  # types: ignore
        super().__init__(*args, **kwargs)
        self.include_router(Router(event_fetcher), prefix="/api")

        @self.get("/")
        def get_index() -> FileResponse:
            return FileResponse(frontend_path / "index.html")

        self.mount(
            "/",
            StaticFiles(directory=frontend_path),
            name="static",
        )
