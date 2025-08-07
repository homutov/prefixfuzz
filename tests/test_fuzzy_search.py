from prefixfuzz import prefix_search_builder, PrefixSearch
import pytest

TERMS = [
    ("эдисон", 1),
    ("эдесон", 2),
    ("эдисон перец", 3),
    ("эдесон перец", 4),
    ("эдессо", 5),
    ("эдессон", 6),
    ("эдесса", 7),
    ("эдессан", 8),
    ("эдесон п", 9)
]


@pytest.fixture(scope="module")
def prefix_search() -> PrefixSearch:
    return prefix_search_builder.from_nodes(TERMS)


def test_exact_match(prefix_search: PrefixSearch):
    assert prefix_search.exact_match("эдисон")[1] == 1
    assert prefix_search.exact_match("эдессон")[1] == 6


def test_fuzzy_match_distance_eq_1(prefix_search: PrefixSearch):
    assert [res[0] for res in prefix_search.fuzzy_match("эдессон", 1, None)] == ["эдессо", "эдессон", "эдессан", "эдесон",]


def test_fuzzy_match_distance_eq_1_with_limit(prefix_search: PrefixSearch):
    assert [res[0] for res in prefix_search.fuzzy_match("эдессон", 1, 2)] == ["эдессо", "эдессон"]
    assert [res[1] for res in prefix_search.fuzzy_match("эдессон", 1, 2)] == [5 ,6]
    assert [res[2] for res in prefix_search.fuzzy_match("эдессон", 1, 2)] == [1, 0]


def test_fuzzy_match_distance_eq_2(prefix_search: PrefixSearch):
    print(prefix_search.fuzzy_match("эдессон", 2, None))
    assert set(res[0] for res in prefix_search.fuzzy_match("эдессон", 2, None)) == {"эдессо", "эдессон", "эдесса", "эдессан", "эдесон", "эдисон"}

