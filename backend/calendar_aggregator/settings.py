from pathlib import Path

from pydantic import BaseSettings


class Settings(BaseSettings):
    frontend_path: Path = Path("placeholder")
    event_cache_expiration_s: float = 10 * 60


settings = Settings()
