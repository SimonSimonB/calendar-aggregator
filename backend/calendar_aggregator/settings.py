from pathlib import Path

from pydantic import BaseSettings


class Settings(BaseSettings):
    event_cache_expiration_s: float = 10 * 60
    frontend_path: Path = Path("placeholder")
    port: int = 8000


settings = Settings()
