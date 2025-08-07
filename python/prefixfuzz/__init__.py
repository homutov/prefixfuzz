from ._prefixfuzz import *


__doc__ = _prefixfuzz.__doc__
if hasattr(_prefixfuzz, "__all__"):
    __all__ = _prefixfuzz.__all__

PrefixSearch.from_internal_data = staticmethod(from_internal_data)
PrefixSearch.from_bytes = staticmethod(from_bytes)