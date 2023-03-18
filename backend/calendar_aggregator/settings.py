from pathlib import Path

from pydantic import BaseSettings


class Settings(BaseSettings):
    frontend_path: Path = Path("placeholder")


settings = Settings()
