# vulk

[![Crates.io](https://img.shields.io/crates/v/vulk)](https://crates.io/crates/vulk)

`vulk` is a **highly** experimental Vulkan bindings library, except it only includes the latest features and extensions, such as ray tracing, mesh shaders, descriptor buffers, shader objects, etc. `vulk` will not maintain backward compatibility when a new Vulkan extension completely replaces a feature. For example, in `vulk`, you can't use vertex shaders because mesh shaders are a superset of all geometry stages.

The design of `vulk` is inspired by `ash`, which is also what you should use for production use cases instead of `vulk`.

Currently, the only supported driver is the latest NVIDIA March 31st, 2023 driver.
