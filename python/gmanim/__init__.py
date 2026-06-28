from .gmanim import *
from .gmanim import registry, scene

import sys
import gmanim.gmanim as _gmanim

__all__ = _gmanim.__all__ + ["registry", "scene"]
__doc__ = _gmanim.__doc__


def incremental(func):
    """Decorator to mark a function as incremental (stateful), which disables fast-forwarding."""
    func.__incremental__ = True
    return func


from .animations import *
from .mobjects import *
from .animations import __all__ as _anim_all
from .mobjects import __all__ as _mobj_all

__all__.extend(["incremental"])
__all__.extend(_anim_all)
__all__.extend(_mobj_all)
