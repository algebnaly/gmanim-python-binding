from .gmanim import *

import gmanim.gmanim as _gmanim

from .types import Color, Point3
from .animations import *
from .mobjects import *
from .animations import __all__ as _anim_all
from .mobjects import __all__ as _mobj_all

__all__ = list(_gmanim.__all__) + ["Color", "Point3"]
__doc__ = _gmanim.__doc__
__all__.extend(_anim_all)
__all__.extend(_mobj_all)
