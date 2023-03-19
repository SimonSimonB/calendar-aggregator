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
        all_groups = re.findall(self._regex(), text_cleaned)

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
        self._german_months = {
            "January": 1,
            "February": 2,
            "March": 3,
            "April": 4,
            "May": 5,
            "June": 6,
            "July": 7,
            "August": 8,
            "September": 9,
            "October": 10,
            "November": 11,
            "December": 12,
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
                if long_month[:3] == month[:3]
            ),
            month,
        )
        return _DateWithOptionalYear(
            year=None if year is "" else int(year),
            month=self._german_months[month],
            day=int(day),
        )


class MonthDayEnglishExtractor(DateExtractor):
    def __init__(self) -> None:
        self._german_months = {
            "January": 1,
            "February": 2,
            "March": 3,
            "April": 4,
            "May": 5,
            "June": 6,
            "July": 7,
            "August": 8,
            "September": 9,
            "October": 10,
            "November": 11,
            "December": 12,
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
                if long_month[:3] == month[:3]
            ),
            month,
        )
        return _DateWithOptionalYear(
            year=None if year is "" else int(year),
            month=self._german_months[month],
            day=int(day),
        )


class GermanMonthsExtractor(DateExtractor):
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
                if long_month[:3] == month[:3]
            ),
            month,
        )
        return _DateWithOptionalYear(
            year=None if year is "" else int(year),
            month=self._german_months[month],
            day=int(day),
        )
