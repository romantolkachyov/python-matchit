from matchit import Router

r = Router()
r.insert("/test/{id}", "test_id")

res = r.at("/test/1")

assert res.route == "test_id"
assert res.params == {"id": "1"}

try:
    assert r.at("/noway")
except LookupError:
    print("Not found as expected")