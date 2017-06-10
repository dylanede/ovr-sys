use ::{
    ovrResult,
    ovrSession,
    ovrTextureSwapChainDesc,
    ovrTextureSwapChain,
    ovrMirrorTexture,
    ovrMirrorTextureDesc
};

use ::libc::{
    c_int,
    c_void,
};

use ::winapi::guiddef::IID;
use ::winapi::unknwnbase::IUnknown;

//-----------------------------------------------------------------------------------
// ***** Direct3D Specific

extern "C" {

    /// Create Texture Swap Chain suitable for use with Direct3D 11 and 12.
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `d3dPtr` Specifies the application's `D3D11Device` to create resources with or the `D3D12CommandQueue`
    ///             which must be the same one the application renders to the eye textures with.
    ///
    /// **in** `desc` Specifies requested texture properties. See notes for more info about texture format.
    ///
    /// **in** `bindFlags` Specifies what `ovrTextureBindFlags` the application requires for this texture chain.
    ///
    /// **out** `out_TextureSwapChain` Returns the created `ovrTextureSwapChain`, which will be valid upon a successful return value, else it will be NULL.
    ///             This texture chain must be eventually destroyed via `ovr_DestroyTextureSwapChain` before destroying the session with `ovr_Destroy`.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    /// **Note**: The texture format provided in the desc should be thought of as the format the distortion-compositor will use for the
    /// `ShaderResourceView` when reading the contents of the texture. To that end, it is highly recommended that the application
    /// requests texture swapchain formats that are in sRGB-space (e.g. `OVR_FORMAT_R8G8B8A8_UNORM_SRGB`) as the compositor
    /// does sRGB-correct rendering. As such, the compositor relies on the GPU's hardware sampler to do the sRGB-to-linear
    /// conversion. If the application still prefers to render to a linear format (e.g. `OVR_FORMAT_R8G8B8A8_UNORM`) while handling the
    /// linear-to-gamma conversion via HLSL code, then the application must still request the corresponding sRGB format and also use
    /// the `ovrTextureMisc_DX_Typeless` flag in the `ovrTextureSwapChainDesc`'s Flag field. This will allow the application to create
    /// a RenderTargetView that is the desired linear format while the compositor continues to treat it as sRGB. Failure to do so
    /// will cause the compositor to apply unexpected gamma conversions leading to gamma-curve artifacts. The `ovrTextureMisc_DX_Typeless`
    /// flag for depth buffer formats (e.g. `OVR_FORMAT_D32_FLOAT`) is ignored as they are always converted to be typeless.
    ///
    /// see [`ovr_GetTextureSwapChainLength`](../fn.ovr_GetTextureSwapChainLength.html), [`ovr_GetTextureSwapChainCurrentIndex`](../fn.ovr_GetTextureSwapChainCurrentIndex.html), [`ovr_GetTextureSwapChainDesc`](../fn.ovr_GetTextureSwapChainDesc.html), [`ovr_GetTextureSwapChainBufferDX`](fn.ovr_GetTextureSwapChainBufferDX.html), [`ovr_DestroyTextureSwapChain`](../fn.ovr_DestroyTextureSwapChain.html)
    ///
    pub fn ovr_CreateTextureSwapChainDX(session: ovrSession, d3dPtr: * mut IUnknown, desc: * const ovrTextureSwapChainDesc, out_TextureSwapChain: * mut ovrTextureSwapChain) -> ovrResult;


    /// Get a specific buffer within the chain as any compatible COM interface (similar to `QueryInterface`)
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `chain` Specifies an `ovrTextureSwapChain` previously returned by `ovr_CreateTextureSwapChainDX`
    ///
    /// **in** `index` Specifies the index within the chain to retrieve. Must be between 0 and length (see `ovr_GetTextureSwapChainLength`),
    ///             or may pass -1 to get the buffer at the `CurrentIndex` location. (Saving a call to `GetTextureSwapChainCurrentIndex`)
    ///
    /// **in** `iid` Specifies the interface ID of the interface pointer to query the buffer for.
    ///
    /// **out** `out_Buffer` Returns the COM interface pointer retrieved.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    /// **Example code, not translated from C**
    ///
    /// ```ignore
    /// ovr_GetTextureSwapChainBufferDX(session, chain, 0, IID_ID3D11Texture2D, &d3d11_texture);
    /// ovr_GetTextureSwapChainBufferDX(session, chain, 1, IID_PPV_ARGS(&dxgi_resource));
    /// ```
    ///
    pub fn ovr_GetTextureSwapChainBufferDX(session: ovrSession, chain: ovrTextureSwapChain, index: c_int, iid: IID, out_Buffer: *mut *mut c_void) -> ovrResult;


    /// Create Mirror Texture which is auto-refreshed to mirror Rift contents produced by this application.
    ///
    /// A second call to `ovr_CreateMirrorTextureDX` for a given ovrSession before destroying the first one
    /// is not supported and will result in an error return.
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `d3dPtr` Specifies the application's `D3D11Device` to create resources with or the `D3D12CommandQueue`
    ///             which must be the same one the application renders to the textures with.
    ///
    /// **in** `desc` Specifies requested texture properties. See notes for more info about texture format.
    ///
    /// **out** `out_MirrorTexture` Returns the created `ovrMirrorTexture`, which will be valid upon a successful return value, else it will be NULL.
    ///             This texture must be eventually destroyed via `ovr_DestroyMirrorTexture` before destroying the session with `ovr_Destroy`.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    /// **Note**: The texture format provided in the desc should be thought of as the format the compositor will use for the `RenderTargetView` when
    /// writing into mirror texture. To that end, it is highly recommended that the application requests a mirror texture format that is
    /// in sRGB-space (e.g. `OVR_FORMAT_R8G8B8A8_UNORM_SRGB`) as the compositor does sRGB-correct rendering. If however the application wants
    /// to still read the mirror texture as a linear format (e.g. `OVR_FORMAT_R8G8B8A8_UNORM`) and handle the sRGB-to-linear conversion in
    /// HLSL code, then it is recommended the application still requests an sRGB format and also use the `ovrTextureMisc_DX_Typeless` flag in the
    /// `ovrMirrorTextureDesc`'s Flags field. This will allow the application to bind a `ShaderResourceView` that is a linear format while the
    /// compositor continues to treat is as sRGB. Failure to do so will cause the compositor to apply unexpected gamma conversions leading to
    /// gamma-curve artifacts.
    ///
    ///
    /// **Example code, not translated from C**
    ///
    /// ```ignore
    /// ovrMirrorTexture     mirrorTexture = nullptr;
    /// ovrMirrorTextureDesc mirrorDesc = {};
    /// mirrorDesc.Format = OVR_FORMAT_R8G8B8A8_UNORM_SRGB;
    /// mirrorDesc.Width  = mirrorWindowWidth;
    /// mirrorDesc.Height = mirrorWindowHeight;
    /// ovrResult result = ovr_CreateMirrorTextureDX(session, d3d11Device, &mirrorDesc, &mirrorTexture);
    /// [...]
    /// // Destroy the texture when done with it.
    /// ovr_DestroyMirrorTexture(session, mirrorTexture);
    /// mirrorTexture = nullptr;
    /// ```
    ///
    /// see `ovr_GetMirrorTextureBufferDX`, `ovr_DestroyMirrorTexture`
    ///
    pub fn ovr_CreateMirrorTextureDX(session: ovrSession, d3dPtr: *mut IUnknown, desc: *const ovrMirrorTextureDesc, out_MirrorTexture: *mut ovrMirrorTexture) -> ovrResult;

    /// Get the underlying buffer as any compatible COM interface (similar to `QueryInterface`)
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `mirrorTexture` Specifies an `ovrMirrorTexture` previously returned by `ovr_CreateMirrorTextureDX`
    ///
    /// **in** `iid` Specifies the interface ID of the interface pointer to query the buffer for.
    ///
    /// **out** `out_Buffer` Returns the COM interface pointer retrieved.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    /// **Example code, not translated from C**
    ///
    /// ```ignore
    /// ID3D11Texture2D* d3d11Texture = nullptr;
    /// ovr_GetMirrorTextureBufferDX(session, mirrorTexture, IID_PPV_ARGS(&d3d11Texture));
    /// d3d11DeviceContext->CopyResource(d3d11TextureBackBuffer, d3d11Texture);
    /// d3d11Texture->Release();
    /// dxgiSwapChain->Present(0, 0);
    /// ```
    ///
    pub fn ovr_GetMirrorTextureBufferDX(session: ovrSession, mirrorTexture: ovrMirrorTexture, iid: IID, out_Buffer: *mut *mut c_void) -> ovrResult;
}
