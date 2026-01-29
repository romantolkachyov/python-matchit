from matchit import Router

r = Router()
r.insert("/test/{id}", "test_id")

res = r.at("/test/1")
print(res)
assert res == {"id": "1"}

try:
    assert r.at("/noway")
except LookupError:
    print("Not found as expected")