from __future__ import annotations

from typing import List
from abc import ABC, abstractmethod


class YouTubeChannalSubject(ABC):
    @abstractmethod
    def subscribe(self, observer: SubscriberObserver) -> None:
        """attach observer"""

    @abstractmethod
    def unsubscribe(self, observer: SubscriberObserver) -> None:
        """detach observer"""

    @abstractmethod
    def notify(self) -> None:
        """notify all observers"""


class SubscriberObserver(ABC):
    @abstractmethod
    def update(self, msg: str) -> None:
        """Rescive update from `Subject`"""


class YouTubeChannal(YouTubeChannalSubject):
    """Concrete Subject"""
    subscribers: List[SubscriberObserver] = []  # Observer

    def subscribe(self, observer: SubscriberObserver) -> None:
        self.subscribers.append(observer)

    def unsubscribe(self, observer: SubscriberObserver) -> None:
        self.subscribers.remove(observer)

    def notify(self, msg: str) -> None:
        for observer in self.subscribers:
            observer.update(msg)


class YouTubeAccount(SubscriberObserver):
    """Concrete Observer"""
    def update(self, msg: str) -> None:
        print("account msg: ", msg)


class SubYouTubeChannal(SubscriberObserver):
    """Concrete Observer"""
    def update(self, msg: str) -> None:
        print("subchannal msg: ", msg)


def main() -> None:
    youtube_channal = YouTubeChannal()

    youtube_account = YouTubeAccount()
    youtube_subchannal = SubYouTubeChannal()

    youtube_channal.subscribe(youtube_account)
    youtube_channal.subscribe(youtube_subchannal)

    youtube_channal.notify("hello from my channal")


if __name__ == "__main__":
    main()
