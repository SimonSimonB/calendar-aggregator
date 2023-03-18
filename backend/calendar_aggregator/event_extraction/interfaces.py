import abc
from typing import List
from ..models import Event


class AbstractEventExtractor(abc.ABC):
    @abc.abstractmethod
    def extract(self, html: str) -> List[Event]:
        pass
