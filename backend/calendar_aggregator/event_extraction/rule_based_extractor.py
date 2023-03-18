import datetime
from typing import Dict, List, Optional

from bs4 import BeautifulSoup, PageElement
from bs4.element import Tag

from ..models import Event
from .interfaces import AbstractEventExtractor
from . import date_extraction


class RuleBasedExtractor(AbstractEventExtractor):
    """Extracts events from HTML of web pages by apply rule-based heuristics.

    This extractor finds the largest containers in the HTML tree that contain exactly one date, and then attempts to
    extract one event from each container.
    """

    def extract(self, html: str) -> List[Event]:
        events: List[Event] = []
        event_html_elements = RuleBasedExtractor._extract_event_elements(html)
        for element in event_html_elements:
            event = RuleBasedExtractor._extract_event(element)
            if event:
                events.append(event)

        return events

    @staticmethod
    def _extract_event_elements(html: str) -> List[PageElement]:
        # We assume that, for each date we detect on the web page, the largest HTML element that contains only that one date describes an event.
        html_tree = BeautifulSoup(html, "html.parser")

        event_elements: List[PageElement] = []
        for root in html_tree.contents:
            date_counts = RuleBasedExtractor._date_counts(root)
            event_elements.extend(
                RuleBasedExtractor._largest_elements_with_single_date(root, date_counts)
            )

        return event_elements

    @staticmethod
    def _date_counts(root: PageElement) -> Dict[PageElement, int]:
        # Use an inner function with an accumulator to avoid having to aggregate dictionaries at each node
        def _date_counts_with_accumulator(
            el: PageElement, result: Dict[PageElement, int]
        ) -> None:
            if isinstance(el, str):
                result[el] = len(RuleBasedExtractor._extract_dates(el))
            elif isinstance(el, Tag):
                for child in el.contents:
                    _date_counts_with_accumulator(child, result)
                result[el] = sum(result[child] for child in el.contents)
            else:
                result[el] = 0

        result: Dict[PageElement, int] = {}
        _date_counts_with_accumulator(root, result)
        return result

    @staticmethod
    def _largest_elements_with_single_date(
        el: PageElement, date_counts: Dict[PageElement, int]
    ) -> List[PageElement]:
        if date_counts[el] == 1:
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
            date_extraction.GermanShortMonthExtractor(),
            date_extraction.GermanLongMonthExtractor(),
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
