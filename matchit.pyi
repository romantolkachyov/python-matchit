from typing import Mapping


class MatchResult:
    """Router match result."""
    route_id: str
    params: Mapping[str, str]

class Router:
    """Router.

    Holds url patterns for each route and allows to retrieve route matching provided path.

    It is not safe to insert routes after application is started up.
    """

    def insert(self, url_pattern: str, route_id: str) -> None:
        """Add route to the router.
        """
        ...

    def at(self, path: str) -> MatchResult: ...