use super::{
    ovrResult,
    ovrSession,
    ovrTextureSwapChain,
    ovrTextureSwapChainDesc,
    ovrMirrorTextureDesc,
    ovrMirrorTexture
};

use ::libc::{
    c_int,
    c_uint
};

extern "C" {
    /// Creates a TextureSwapChain suitable for use with OpenGL.
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `desc` Specifies the requested texture properties. See notes for more info about texture format.
    ///
    /// **out** `out_TextureSwapChain` Returns the created `ovrTextureSwapChain`, which will be valid upon
    ///             a successful return value, else it will be NULL. This texture swap chain must be eventually
    ///             destroyed via `ovr_DestroyTextureSwapChain` before destroying the session with `ovr_Destroy`.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    /// **Note**: The format provided should be thought of as the format the distortion compositor will use when reading
    /// the contents of the texture. To that end, it is highly recommended that the application requests texture swap chain
    /// formats that are in sRGB-space (e.g. `OVR_FORMAT_R8G8B8A8_UNORM_SRGB`) as the distortion compositor does sRGB-correct
    /// rendering. Furthermore, the app should then make sure `glEnable(GL_FRAMEBUFFER_SRGB);` is called before rendering
    /// into these textures. Even though it is not recommended, if the application would like to treat the texture as a linear
    /// format and do linear-to-gamma conversion in GLSL, then the application can avoid calling `glEnable(GL_FRAMEBUFFER_SRGB);`,
    /// but should still pass in an sRGB variant for the format. Failure to do so will cause the distortion compositor
    /// to apply incorrect gamma conversions leading to gamma-curve artifacts.
    ///
    /// see [`ovr_GetTextureSwapChainLength`](../fn.ovr_GetTextureSwapChainLength.html), [`ovr_GetTextureSwapChainCurrentIndex`](../fn.ovr_GetTextureSwapChainCurrentIndex.html), [`ovr_GetTextureSwapChainDesc`](../fn.ovr_GetTextureSwapChainDesc.html), [`ovr_GetTextureSwapChainBufferGL`](fn.ovr_GetTextureSwapChainBufferGL.html), [`ovr_DestroyTextureSwapChain`](../fn.ovr_DestroyTextureSwapChain.html)
    ///
    pub fn ovr_CreateTextureSwapChainGL(session: ovrSession, desc: *const ovrTextureSwapChainDesc, out_TextureSwapChain: *mut ovrTextureSwapChain) -> ovrResult;

    /// Get a specific buffer within the chain as a GL texture name
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `chain` Specifies an `ovrTextureSwapChain` previously returned by `ovr_CreateTextureSwapChainGL`
    ///
    /// **in** `index` Specifies the index within the chain to retrieve. Must be between 0 and length (see `ovr_GetTextureSwapChainLength`)
    ///             or may pass -1 to get the buffer at the `CurrentIndex` location. (Saving a call to `GetTextureSwapChainCurrentIndex`)
    ///
    /// **out** `out_TexId` Returns the GL texture object name associated with the specific index requested
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    pub fn ovr_GetTextureSwapChainBufferGL(session: ovrSession, chain: ovrTextureSwapChain, index: c_int, out_TexId: *mut c_uint) -> ovrResult;

    /// Creates a Mirror Texture which is auto-refreshed to mirror Rift contents produced by this application.
    ///
    /// A second call to `ovr_CreateMirrorTextureGL` for a given `ovrSession` before destroying the first one
    /// is not supported and will result in an error return.
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `desc` Specifies the requested mirror texture description.
    ///
    /// **out** `out_MirrorTexture` Specifies the created `ovrMirrorTexture`, which will be valid upon a successful return value, else it will be NULL.
    ///             This texture must be eventually destroyed via `ovr_DestroyMirrorTexture` before destroying the session with `ovr_Destroy`.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    /// **Note**: The format provided should be thought of as the format the distortion compositor will use when writing into the mirror
    /// texture. It is highly recommended that mirror textures are requested as sRGB formats because the distortion compositor
    /// does sRGB-correct rendering. If the application requests a non-sRGB format (e.g. `R8G8B8A8_UNORM`) as the mirror texture,
    /// then the application might have to apply a manual linear-to-gamma conversion when reading from the mirror texture.
    /// Failure to do so can result in incorrect gamma conversions leading to gamma-curve artifacts and color banding.
    ///
    /// see `ovr_GetMirrorTextureBufferGL`, `ovr_DestroyMirrorTexture`
    ///
    pub fn ovr_CreateMirrorTextureGL(session: ovrSession, desc: *const ovrMirrorTextureDesc, out_MirrorTexture: *mut ovrMirrorTexture) -> ovrResult;

    /// Get a the underlying buffer as a GL texture name
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `mirrorTexture` Specifies an `ovrMirrorTexture` previously returned by `ovr_CreateMirrorTextureGL`
    ///
    /// **out** `out_TexId` Specifies the GL texture object name associated with the mirror texture
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    pub fn ovr_GetMirrorTextureBufferGL(session: ovrSession, mirrorTexture: ovrMirrorTexture, out_TexId: *mut c_uint) -> ovrResult;
}