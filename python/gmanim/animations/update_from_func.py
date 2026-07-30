from typing import Callable

import gmanim.gmanim as _gmanim
from gmanim.gmanim import SceneFrame, UpdateFromFunc as _UpdateFromFunc


def UpdateFromFunc(
    func: Callable[[SceneFrame, float], None], frames: int = 60
) -> _UpdateFromFunc:
    return _gmanim.UpdateFromFunc(func, frames)


__all__ = ["UpdateFromFunc"]
