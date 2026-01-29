from matchit import Router

r = Router[str]()
r.insert("/test/{id}", "test_id")

res = r.at("/test/1")

assert res.value == "test_id"
assert res.params == {"id": "1"}

try:
    assert r.at("/noway")
except LookupError:
    print("Not found as expected")