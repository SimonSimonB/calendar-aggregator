import abc
import datetime
import re
from dataclasses import dataclass
from itertools import chain
from typing import List, Optional


@dataclass
class _DateWithOptionalYear:
    year: Optional[int]
    month: int
    day: int

    def to_datetime(self) -> datetime.datetime:
        if self.year is None:
            raise ValueError

        return datetime.datetime(year=self.year, month=self.month, day=self.day)


class DateExtractor(abc.ABC):
    def extract_dates(self, text: str) -> List[datetime.datetime]:
        text_cleaned = text.replace("\n", " ")
        all_groups = re.findall(self._regex(), text_cleaned, re.IGNORECASE)

        dates: List[datetime.datetime] = []
        for groups in all_groups:
            new_date = self._parse_groups(groups)
            if new_date.year is None:
                # If a day and month but not date was found, assume the date refers to this year (if possible).
                # This greatly increases recall (many websites show events of this year but leave out the year in the dates)
                # but it also increases false positives (some websites show events of previous or future years, too).
                new_date.year = datetime.datetime.today().year

            try:
                dates.append(new_date.to_datetime())
            except ValueError:
                # Due to errors on websites or in the date detection logic, we might try to form dates that do not
                # exist (such as the 31st of February). In those cases, ignore the date.
                pass

        return dates

    @abc.abstractmethod
    def _regex(self) -> str:
        pass

    @abc.abstractmethod
    def _parse_groups(self, groups: list[str]) -> _DateWithOptionalYear:
        pass


class DDMMYYYYExtractor(DateExtractor):
    def _regex(self) -> str:
        return r"(\d\d)\.(\d\d)\.(\d\d\d\d)"

    def _parse_groups(self, groups: list[str]) -> _DateWithOptionalYear:
        day, month, year = groups
        return _DateWithOptionalYear(year=int(year), month=int(month), day=int(day))


class DayMonthEnglishExtractor(DateExtractor):
    def __init__(self) -> None:
        self._english_months = {
            "january": 1,
            "february": 2,
            "march": 3,
            "april": 4,
            "may": 5,
            "june": 6,
            "july": 7,
            "august": 8,
            "september": 9,
            "october": 10,
            "november": 11,
            "december": 12,
        }

    def _regex(self) -> str:
        # Match both long and abbreviated month names ('August' and 'Aug').
        long_and_short_month_names = list(
            chain(
                *zip(
                    self._english_months.keys(),
                    (month[:3] for month in self._english_months.keys()),
                )
            )
        )
        month_regex = "|".join(long_and_short_month_names)
        return f"(\\d\\d).{{0,3}}({month_regex}).{{0,2}}(\\d\\d\\d\\d)?"

    def _parse_groups(self, groups: list[str]) -> _DateWithOptionalYear:
        day, month, year = groups
        month = next(
            (
                long_month
                for long_month in self._english_months
                if long_month.lower()[:3] == month.lower()[:3]
            ),
            month,
        )
        return _DateWithOptionalYear(
            year=None if year is "" else int(year),
            month=self._english_months[month.lower()],
            day=int(day),
        )


class MonthDayEnglishExtractor(DateExtractor):
    def __init__(self) -> None:
        self._german_months = {
            "january": 1,
            "february": 2,
            "march": 3,
            "april": 4,
            "may": 5,
            "june": 6,
            "july": 7,
            "august": 8,
            "september": 9,
            "october": 10,
            "november": 11,
            "december": 12,
        }

    def _regex(self) -> str:
        # Match both long and abbreviated month names ('August' and 'Aug').
        long_and_short_month_names = list(
            chain(
                *zip(
                    self._german_months.keys(),
                    (month[:3] for month in self._german_months.keys()),
                )
            )
        )
        month_regex = "|".join(long_and_short_month_names)
        return f"({month_regex}).{{0,2}}(\\d\\d)[^\\d]{{1,3}}(\\d\\d\\d\\d)?"

    def _parse_groups(self, groups: list[str]) -> _DateWithOptionalYear:
        month, day, year = groups
        month = next(
            (
                long_month
                for long_month in self._german_months
                if long_month.lower()[:3] == month.lower()[:3]
            ),
            month,
        )
        return _DateWithOptionalYear(
            year=None if year is "" else int(year),
            month=self._german_months[month.lower()],
            day=int(day),
        )


class GermanMonthsExtractor(DateExtractor):
    def __init__(self) -> None:
        self._german_months = {
            "januar": 1,
            "februar": 2,
            "märz": 3,
            "april": 4,
            "mai": 5,
            "juni": 6,
            "juli": 7,
            "august": 8,
            "september": 9,
            "oktober": 10,
            "november": 11,
            "dezember": 12,
        }

    def _regex(self) -> str:
        # Match both long and abbreviated month names ('August' and 'Aug').
        long_and_short_month_names = list(
            chain(
                *zip(
                    self._german_months.keys(),
                    (month[:3] for month in self._german_months.keys()),
                )
            )
        )
        month_regex = "|".join(long_and_short_month_names)
        return f"(\\d\\d).{{0,3}}({month_regex}).{{0,2}}(\\d\\d\\d\\d)?"

    def _parse_groups(self, groups: list[str]) -> _DateWithOptionalYear:
        day, month, year = groups
        month = next(
            (
                long_month
                for long_month in self._german_months
                if long_month.lower()[:3] == month.lower()[:3]
            ),
            month,
        )
        return _DateWithOptionalYear(
            year=None if year is "" else int(year),
            month=self._german_months[month.lower()],
            day=int(day),
        )
