import abc
from typing import List

from ..models import Event


class AbstractEventFetcher(abc.ABC):
    @abc.abstractmethod
    async def fetch(self, url: str) -> List[Event]:
        pass
