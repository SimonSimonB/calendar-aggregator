from dataclasses import dataclass
import datetime


@dataclass
class Event:
    date: datetime.datetime
    text: str
