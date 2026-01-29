from typing import Mapping, TypeVar, Generic

T = TypeVar("T")


class MatchResult(Generic[T]):
    """Router match result."""
    route: T
    params: Mapping[str, str]

class Router(Generic[T]):
    """Router.

    Holds url patterns for each route and allows to retrieve route matching provided path.

    It is not safe to insert routes after application is started up.
    """

    def insert(self, url_pattern: str, route: T) -> None:
        """Add route to the router.
        """
        ...

    def at(self, path: str) -> MatchResult[T]: ...