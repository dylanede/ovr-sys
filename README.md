# ovr-sys

[![Documentation](https://docs.rs/ovr-sys/badge.svg)](https://docs.rs/ovr-sys)

```toml
[dependencies]
ovr-sys = "0.1.0"
```

Well documented raw bindings to libOVR v1.15.0, the Oculus Rift PC runtime library.
The documentation is transformed from the original [doxygen](www.doxygen.org) docs.

Until Oculus resumes support for other platforms, this crate will only work on Windows.
Both 32-bit and 64-bit Windows are supported.

### [Documentation](https://docs.rs/ovr-sys)

Additional documentation is available from Oculus directly,
at https://developer3.oculus.com/documentation/pcsdk/latest/concepts/book-dg/

The intention is for this crate to follow the latest version of libOVR.
Pin to a particular release of ovr-sys to stay with a particular version of libOVR.

## Features

ovr-sys has optional features corresponding to the parts of libOVR dealing with 
audio, OpenGL, DirectX and Vulkan. The relevant feature names are
`audio`, `opengl`, `directx` and `vulkan` respectively. Each feature corresponds to a submodule 
with the relevant functionality. The audio and DirectX features are designed for and are 
only usable on Windows (the core of libOVR, however, does at least hint at potential 
future support for other platforms). By default the OpenGL feature is enabled.

As an example, to enable support for just audio and OpenGL:

```toml
[dependencies]
ovr-sys = { version = "0.1.0", features = ["audio", "opengl"] }
```

## License

The lib directory redistributes Oculus static libraries and its contents are licensed under 
the terms of the Oculus SDK License ([LICENSE-OCULUS](LICENSE-OCULUS)).

Everything else is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.