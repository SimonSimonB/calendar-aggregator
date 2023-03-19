import datetime
from typing import Dict, List, Optional

from bs4 import BeautifulSoup, PageElement
from bs4.element import Tag

from ..models import Event
from . import date_extraction
from .interfaces import AbstractEventExtractor

_ObjId = int


class RuleBasedExtractor(AbstractEventExtractor):
    """Extracts events from HTML of web pages by apply rule-based heuristics.

    This extractor finds the topmost elements in the HTML tree that contain exactly one date, and then attempts to
    extract one event from each element.
    """

    def extract(self, html: str) -> List[Event]:
        events: List[Event] = []
        event_html_elements = RuleBasedExtractor._extract_event_elements(html)
        for el in event_html_elements:
            event = RuleBasedExtractor._extract_event(el)
            if event:
                events.append(event)

        return events

    @staticmethod
    def _extract_event_elements(html: str) -> List[PageElement]:
        html_tree = BeautifulSoup(html, "html.parser")

        event_elements: List[PageElement] = []
        for root in html_tree.contents:
            # `date_counts` does not map `PageElement` objects to how many dates they contain. Rather, it uses the Python
            # object IDs as keys. Profiling showed that working with a dictionary of `PageElement`s led to the majority
            # of time being spent hashing `PageElement` objects. A speedup of a factor ~10 was observed from switching
            # to object IDs as keys on a benchmark of extracting events from the HTML of ~10 real websites.
            date_counts = RuleBasedExtractor._date_counts(root)
            event_elements.extend(
                RuleBasedExtractor._largest_elements_with_single_date(root, date_counts)
            )

        return event_elements

    @staticmethod
    def _date_counts(root: PageElement) -> Dict[_ObjId, int]:
        # Use an inner function with an accumulator to avoid having to aggregate dictionaries at each node
        def _date_counts_with_accumulator(
            el: PageElement, result: Dict[_ObjId, int]
        ) -> None:
            if isinstance(el, str):
                result[id(el)] = len(RuleBasedExtractor._extract_dates(el))
            elif isinstance(el, Tag):
                for child in el.contents:
                    _date_counts_with_accumulator(child, result)
                # It is not enough to sum the date counts of the children here because maybe there is a date in
                # the concatenation of the texts of the children, but in no individual child.
                result[id(el)] = len(RuleBasedExtractor._extract_dates(el.text))
            else:
                result[id(el)] = 0

        result: Dict[_ObjId, int] = {}
        _date_counts_with_accumulator(root, result)
        return result

    @staticmethod
    def _largest_elements_with_single_date(
        el: PageElement, date_counts: Dict[_ObjId, int]
    ) -> List[PageElement]:
        if date_counts[id(el)] == 1:
            return [el]

        result: List[PageElement] = []
        if isinstance(el, Tag):
            for child in el.contents:
                result.extend(
                    RuleBasedExtractor._largest_elements_with_single_date(
                        child, date_counts
                    )
                )

        return result

    @staticmethod
    def _extract_dates(text: str) -> List[datetime.datetime]:
        date_extractors: List[date_extraction.DateExtractor] = [
            date_extraction.DDMMYYYYExtractor(),
            date_extraction.DayMonthEnglishExtractor(),
            date_extraction.MonthDayEnglishExtractor(),
            date_extraction.GermanMonthsExtractor(),
        ]
        for date_extractor in date_extractors:
            dates = date_extractor.extract_dates(text)
            if len(dates) > 0:
                return dates
        return []

    @staticmethod
    def _extract_event(el: PageElement) -> Optional[Event]:
        text = None
        if isinstance(el, Tag):
            text = el.text
        elif isinstance(el, str):
            text = el
        else:
            return None

        time = next(iter(RuleBasedExtractor._extract_dates(text)), None)
        if not time:
            return None

        description = text

        return Event(time, description)
