import httpx

from .interfaces import AbstractHTMLFetcher


class HTMLFetcher(AbstractHTMLFetcher):
    async def fetch(self, url: str) -> str:
        async with httpx.AsyncClient() as client:
            response = await client.get(url)
            return response.text
