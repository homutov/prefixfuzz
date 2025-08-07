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


def test_eq_search(prefix_search: PrefixSearch):
    prefix_search_de: PrefixSearch = PrefixSearch.from_bytes(prefix_search.to_bytes())

    assert prefix_search_de.exact_match("эдессон") == prefix_search.exact_match("эдессон")

    assert len(prefix_search_de.fuzzy_match("эдессон", 2, None)) == 6
    assert prefix_search_de.fuzzy_match("эдессон", 2, None) == prefix_search.fuzzy_match("эдессон", 2, None)
