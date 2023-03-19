import datetime
import glob
import sys
from pathlib import Path
from typing import Any

from calendar_aggregator.event_fetching.event_extraction.rule_based_extractor import (
    RuleBasedExtractor,
)


def test_extracts_date_from_ddmmyyyy_with_dots() -> None:
    events = RuleBasedExtractor().extract(
        """
        <html>
            <body>
                <div>
                    <p>25.12.2023: Christmas Party</p>
                </div>
            </body>
        </html>
    """
    )

    assert (25, 12, 2023) == (
        events[0].date.day,
        events[0].date.month,
        events[0].date.year,
    )


def test_extracts_date_from_german_month_names() -> None:
    events = RuleBasedExtractor().extract(
        """
        <html>
            <body>
                <div>
                    <p>25. Dezember 2023: Christmas Party</p>
                </div>
            </body>
        </html>
    """
    )

    assert (25, 12, 2023) == (
        events[0].date.day,
        events[0].date.month,
        events[0].date.year,
    )


def test_extracts_date_from_shortened_german_month_names() -> None:
    events = RuleBasedExtractor().extract(
        """
        <html>
            <body>
                <div>
                    <p>25. Dez 2023: Christmas Party</p>
                </div>
            </body>
        </html>
    """
    )

    assert (25, 12, 2023) == (
        events[0].date.day,
        events[0].date.month,
        events[0].date.year,
    )


def test_extracts_date_from_shortened_german_month_names_if_year_left_out() -> None:
    events = RuleBasedExtractor().extract(
        """
        <html>
            <body>
                <div>
                    <p>25. Dez: Christmas Party</p>
                </div>
            </body>
        </html>
    """
    )

    assert (25, 12, datetime.datetime.today().year) == (
        events[0].date.day,
        events[0].date.month,
        events[0].date.year,
    )


def test_extracts_date_from_english_month_day() -> None:
    events = RuleBasedExtractor().extract(
        """
        <html>
            <body>
                <div>
                    <p>Dec 25th: Christmas Party</p>
                </div>
            </body>
        </html>
    """
    )

    assert (25, 12, datetime.datetime.today().year) == (
        events[0].date.day,
        events[0].date.month,
        events[0].date.year,
    )


def test_extracts_date_if_spread_across_multiple_divs() -> None:
    events = RuleBasedExtractor().extract(
        """
        <html>
            <body>
                <div>
                    <div>
                        <div>
                            <div class="eventListItem22__weekday">Sun</div>
                            <div class="eventListItem22__day">19</div>
                            <div class="eventListItem22__month">Mar</div>
                            <div class="eventListItem22__year">2023</div>
                        </div>
                    </div>
                    <div class="eventListItem22__info-wrapper">
                        <h1 class="eventListItem22__title">Authentic Healing, with Kendall Johnson-Smith</h1>
                    </div>
                </div>
            </body>
        </html>
    """
    )

    assert (19, 3, 2023) == (
        events[0].date.day,
        events[0].date.month,
        events[0].date.year,
    )


def test_extracts_event_description() -> None:
    events = RuleBasedExtractor().extract(
        """
        <html>
            <body>
                <div>
                    <p>25.12.2023: Christmas Party</p>
                </div>
            </body>
        </html>
        """
    )

    assert "Christmas" in events[0].text


def test_extracts_event_description_from_parent() -> None:
    events = RuleBasedExtractor().extract(
        """
        <html>
            <body>
                <div>
                    <div>
                        Christmas Party
                        <p>25.12.2023</p>
                    </div>
                </div>
            </body>
        </html>
        """
    )

    assert "Christmas" in events[0].text


def test_extracts_event_description_from_aunt() -> None:
    events = RuleBasedExtractor().extract(
        """
        <html>
            <body>
                <div>
                    <p>Christmas Party</p>
                    <div>
                        <p>25.12.2023</p>
                    </div>
                </div>
            </body>
        </html>
        """
    )

    assert "Christmas" in events[0].text


def test_extracts_multiple_events() -> None:
    events = RuleBasedExtractor().extract(
        """
        <html>
            <body>
                <div>
                    25.12.2023: Christmas Party
                </div>
                <div>
                    31.12.2023: New Year's Eve Party
                </div>
            </body>
        </html>
    """
    )

    assert len(events) == 2


def test_does_not_extract_if_only_month_year() -> None:
    events = RuleBasedExtractor().extract(
        """
        <html>
            <body>
                <div>
                    From August 2023, there will be plenty of events.
                </div>
            </body>
        </html>
    """
    )

    assert len(events) == 0


def _generate_tests_for_html_files():
    """Create one test for each file test_html/*.html, verifying that at least one event is extracted from the file."""
    html_files_directory = Path(__file__).parent / "test_html"
    for html_file in glob.glob(str(html_files_directory / "*.html")):

        def get_test_func(_html_file: str) -> Any:
            def test_func() -> None:
                with open(_html_file, encoding="utf-8") as f:
                    html = f.read()
                    assert len(RuleBasedExtractor().extract(html)) >= 1

            return test_func

        test_name = f"test_{html_file.split('/')[-1].replace(' ', '')}"
        setattr(
            sys.modules[__name__],
            test_name,
            get_test_func(html_file),
        )


_generate_tests_for_html_files()
