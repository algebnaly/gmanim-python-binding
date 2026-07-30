# Core 架构迁移

Python binding 只负责描述场景和构建动画，场景状态、时间线求值、渲染与编码由 `gmanim-core` 独立完成。

## 对象

`Mobject` 创建后是 detached blueprint，允许设置初始变换和组合 Group。`Scene.add()` 一次性把整棵 blueprint 转换为 `NodeBundle`，由 core 分配 `MobjectId`，Python 对象随后只保存场景标识和稳定 ID。

对象入场后不再直接可变。边界修改通过 `Scene` 完成，逐帧修改通过 `SceneFrame` 完成；跨场景句柄会被拒绝。

## 动画

所有 Python 动画类只生成统一的 `AnimationSpec`。`Scene.play()` 在构建阶段将 spec 编译到 `TimelineBuilder`。

`UpdateFromFunc` 只在构建阶段执行。每次调用收到拥有自身状态的 `SceneFrame`，其写入由 `PropertyWriteRecorder` 编译为 sampled typed property tracks。Python 异常直接传播，不会以 panic 穿越 FFI。

## 播放

首次查询渲染信息或开始渲染时，`TimelineBuilder` 冻结为 `CompiledTimeline`。冻结后场景不可再修改。seek、渲染和编码不保存 Python 对象，并在释放 GIL 后运行。
