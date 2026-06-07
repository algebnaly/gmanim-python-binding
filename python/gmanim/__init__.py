from .gmanim import *
from .gmanim import registry, scene

import sys
import gmanim.gmanim as _gmanim

if hasattr(_gmanim, "__all__"):
    __all__ = _gmanim.__all__ + ["registry", "scene"]
else:
    __all__ = ["registry", "scene"]

__doc__ = getattr(_gmanim, "__doc__", "")

def incremental(func):
    """Decorator to mark a function as incremental (stateful), which disables fast-forwarding."""
    func.__incremental__ = True
    return func

_UpdateFromFunc = getattr(_gmanim, "UpdateFromFunc", None)
if _UpdateFromFunc:
    def UpdateFromFunc(func, frames=60, is_pure=None):
        if is_pure is None:
            is_pure = not getattr(func, "__incremental__", False)
        return _UpdateFromFunc(func, frames, is_pure)
    
    if "UpdateFromFunc" in __all__:
        __all__.remove("UpdateFromFunc")
    __all__.extend(["incremental", "UpdateFromFunc"])
