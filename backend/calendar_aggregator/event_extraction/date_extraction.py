import abc
import datetime
import re
from typing import List


class DateExtractor(abc.ABC):
    def extract_dates(self, text: str) -> List[datetime.datetime]:
        all_groups = re.findall(self._regex(), text)
        dates: List[datetime.datetime] = []
        for groups in all_groups:
            dates.append(self._parse_groups(groups))
        return dates

    @abc.abstractmethod
    def _regex(self) -> str:
        pass

    @abc.abstractmethod
    def _parse_groups(self, groups: list[str]) -> datetime.datetime:
        pass


class DDMMYYYYExtractor(DateExtractor):
    def _regex(self) -> str:
        return r"(\d\d)\.(\d\d)\.(\d\d\d\d)"

    def _parse_groups(self, groups: list[str]) -> datetime.datetime:
        day, month, year = groups
        return datetime.datetime(int(year), int(month), int(day))


class GermanShortMonthExtractor(DateExtractor):
    def __init__(self) -> None:
        self._german_months = {
            "Jan": 1,
            "Feb": 2,
            "Mär": 3,
            "Apr": 4,
            "Mai": 5,
            "Jun": 6,
            "Jul": 7,
            "Aug": 8,
            "Sep": 9,
            "Okt": 10,
            "Nov": 11,
            "Dez": 12,
        }

    def _regex(self) -> str:
        month_regex = "|".join(self._german_months.keys())
        return f"(\\d\\d).{{0,3}}({month_regex}) (\\d\\d\\d\\d)"

    def _parse_groups(self, groups: list[str]) -> datetime.datetime:
        day, month, year = groups
        return datetime.datetime(int(year), int(self._german_months[month]), int(day))


class GermanLongMonthExtractor(DateExtractor):
    def __init__(self) -> None:
        self._german_months = {
            "Januar": 1,
            "Februar": 2,
            "März": 3,
            "April": 4,
            "Mai": 5,
            "Juni": 6,
            "Juli": 7,
            "August": 8,
            "September": 9,
            "Oktober": 10,
            "November": 11,
            "Dezember": 12,
        }

    def _regex(self) -> str:
        month_regex = "|".join(self._german_months.keys())
        return f"(\\d\\d).{{0,3}}({month_regex}) (\\d\\d\\d\\d)"

    def _parse_groups(self, groups: list[str]) -> datetime.datetime:
        day, month, year = groups
        return datetime.datetime(int(year), int(self._german_months[month]), int(day))
