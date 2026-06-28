import gmanim.gmanim as _gmanim


def UpdateFromFunc(func, frames=60, is_pure=None):
    if is_pure is None:
        is_pure = not getattr(func, "__incremental__", False)
    return _gmanim.UpdateFromFunc(func, frames, is_pure)


__all__ = ["UpdateFromFunc"]
