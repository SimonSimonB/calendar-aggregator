import abc


class AbstractHTMLFetcher(abc.ABC):
    @abc.abstractmethod
    async def fetch(self, url: str) -> str:
        pass
