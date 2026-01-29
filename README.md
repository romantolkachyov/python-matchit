**python-matchit** package contains binding for [matchit](https://docs.rs/matchit/latest/matchit/) rust library.

# Installation

```shell
pip install python-matchit
```

# Usage

```python
from matchit import Router
router = Router()
router.insert("/users/{id}", "test_id")

res = router.at("/users/123")

assert res.route == "test_id"
assert res.params == {"id": "123"}

try:
    assert r.at("/noway")
except LookupError:
    print("Not found as expected")
```