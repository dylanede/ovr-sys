use ::{
    ovrResult,
    ovrSession,
    ovrTextureSwapChainDesc,
    ovrTextureSwapChain,
    ovrMirrorTexture,
    ovrMirrorTextureDesc,
    ovrGraphicsLuid,
};

use ::libc::c_int;

use ::vks::{
    VkInstance,
    VkPhysicalDevice,
    VkQueue,
    VkDevice,
    VkImage
};

extern "C" {
    /// Find `VkPhysicalDevice` matching `ovrGraphicsLuid`
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in**  `luid` Specifies the luid returned from `ovr_Create`.
    ///
    /// **in**  `instance` Specifies an `VkInstance` to search for matching luids in.
    ///
    /// **out** `out_physicalDevice` Returns the `VkPhysicalDevice` matching the instance and luid.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    /// **Note**: This function enumerates the current physical devices and returns the one matching the
    /// luid. It must be called at least once prior to any `ovr_CreateTextureSwapChainVk` or
    /// `ovr_CreateMirrorTextureWithOptionsVk` calls, and the instance must remain valid for the lifetime
    /// of the returned objects. It is assumed the `VkDevice` created by the application will be for the
    /// returned physical device.
    pub fn ovr_GetSessionPhysicalDeviceVk(
        session: ovrSession,
        luid: ovrGraphicsLuid,
        instance: VkInstance,
        out_physicalDevice: *mut VkPhysicalDevice) -> ovrResult;

    /// Select `Queue` to block on till rendering is complete
    ///
    /// **in**  `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in**  `queue` Specifies a `VkQueue` to add a `Fence` operation to and wait on.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    /// **Note**: The queue may be changed at any time but only the value at the time `ovr_SubmitFrame`
    /// is called will be used. `ovr_SetSynchonizationQueueVk` must be called with a valid `VkQueue`
    /// created on the same `VkDevice` the texture sets were created on prior to the first call to
    /// `ovr_SubmitFrame`. An internally created `VkFence` object will be signalled by the completion
    /// of operations on queue and waited on to synchronize the VR compositor.
    ///
    pub fn ovr_SetSynchonizationQueueVk(session: ovrSession, queue: VkQueue) -> ovrResult;

    /// Create Texture Swap Chain suitable for use with Vulkan
    ///
    /// **in**  `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in**  `device` Specifies the application's `VkDevice` to create resources with.
    ///
    /// **in**  `desc` Specifies requested texture properties. See notes for more info
    ///             about texture format.
    ///
    /// **out** `out_TextureSwapChain` Returns the created `ovrTextureSwapChain`, which will be valid
    ///             upon a successful return value, else it will be NULL.
    ///             This texture chain must be eventually destroyed via `ovr_DestroyTextureSwapChain`
    ///             before destroying the session with `ovr_Destroy`.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    /// **Note**: The texture format provided in `desc` should be thought of as the format the
    ///       distortion-compositor will use for the ShaderResourceView when reading the contents
    ///       of the texture. To that end, it is highly recommended that the application
    ///       requests texture swapchain formats that are in sRGB-space
    ///       (e.g. `OVR_FORMAT_R8G8B8A8_UNORM_SRGB`) as the compositor does sRGB-correct rendering.
    ///       As such, the compositor relies on the GPU's hardware sampler to do the sRGB-to-linear
    ///       conversion. If the application still prefers to render to a linear format (e.g.
    ///       `OVR_FORMAT_R8G8B8A8_UNORM`) while handling the linear-to-gamma conversion via
    ///       SPIRV code, then the application must still request the corresponding sRGB format and
    ///       also use the `ovrTextureMisc_DX_Typeless` flag in the `ovrTextureSwapChainDesc`'s
    ///       `Flag` field. This will allow the application to create a RenderTargetView that is the
    ///       desired linear format while the compositor continues to treat it as sRGB. Failure to
    ///       do so will cause the compositor to apply unexpected gamma conversions leading to
    ///       gamma-curve artifacts. The `ovrTextureMisc_DX_Typeless` flag for depth buffer formats
    ///       (e.g. `OVR_FORMAT_D32_FLOAT`) is ignored as they are always
    ///       converted to be typeless.
    ///
    /// see [`ovr_GetTextureSwapChainLength`](../fn.ovr_GetTextureSwapChainLength.html), [`ovr_GetTextureSwapChainCurrentIndex`](../fn.ovr_GetTextureSwapChainCurrentIndex.html), [`ovr_GetTextureSwapChainDesc`](../fn.ovr_GetTextureSwapChainDesc.html), [`ovr_GetTextureSwapChainBufferVk`](fn.ovr_GetTextureSwapChainBufferVk.html), [`ovr_DestroyTextureSwapChain`](../fn.ovr_DestroyTextureSwapChain.html)
    ///
    pub fn ovr_CreateTextureSwapChainVk(
        session: ovrSession,
        device: VkDevice,
        desc: *const ovrTextureSwapChainDesc,
        out_TextureSwapChain: *mut ovrTextureSwapChain) -> ovrResult;

    /// Get a specific `Image` within the chain
    ///
    /// **in**  `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in**  `chain` Specifies an `ovrTextureSwapChain` previously returned by
    ///             `ovr_CreateTextureSwapChainVk`
    ///
    /// **in**  `index` Specifies the index within the chain to retrieve.
    ///             Must be between 0 and length (see `ovr_GetTextureSwapChainLength`),
    ///             or may pass -1 to get the buffer at the CurrentIndex location (saving a
    ///             call to `GetTextureSwapChainCurrentIndex`).
    ///
    /// **out** `out_Image` Returns the `Image` retrieved.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    pub fn ovr_GetTextureSwapChainBufferVk(
        session: ovrSession,
        chain: ovrTextureSwapChain,
        index: c_int,
        out_Image: *mut VkImage) -> ovrResult;

    /// Create Mirror Texture which is auto-refreshed to mirror Rift contents produced by this
    /// application.
    ///
    /// A second call to `ovr_CreateMirrorTextureWithOptionsVk` for a given `ovrSession` before destroying
    /// the first one is not supported and will result in an error return.
    ///
    /// **in**  `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in**  `device` Specifies the `VkDevice` to create resources with.
    ///
    /// **in**  `desc` Specifies requested texture properties. See notes for more info
    ///             about texture format.
    ///
    /// **out** `out_MirrorTexture` Returns the created `ovrMirrorTexture`, which will be
    ///             valid upon a successful return value, else it will be NULL.
    ///             This texture must be eventually destroyed via `ovr_DestroyMirrorTexture` before
    ///             destroying the session with `ovr_Destroy`.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    /// **Note**: The texture format provided in `desc` should be thought of as the format the
    ///       compositor will use for the `VkImageView` when writing into mirror texture. To that end,
    ///       it is highly recommended that the application requests a mirror texture format that is
    ///       in sRGB-space (e.g. `OVR_FORMAT_R8G8B8A8_UNORM_SRGB`) as the compositor does sRGB-correct
    ///       rendering. If however the application wants to still read the mirror texture as a
    ///       linear format (e.g. `OVR_FORMAT_R8G8B8A8_UNORM`) and handle the sRGB-to-linear conversion
    ///       in SPIRV code, then it is recommended the application still requests an sRGB format and
    ///       also use the `ovrTextureMisc_DX_Typeless` flag in the `ovrMirrorTextureDesc`'s
    ///       Flags field. This will allow the application to bind a ShaderResourceView that is a
    ///       linear format while the compositor continues to treat is as sRGB. Failure to do so will
    ///       cause the compositor to apply unexpected gamma conversions leading to
    ///       gamma-curve artifacts.
    ///
    /// Example code
    ///
    /// ```no_run
    /// # use ::std::{mem, ptr};
    /// # use ::ovr_sys::*;
    /// # use ::ovr_sys::vulkan::*;
    /// # unsafe {
    /// # let (session, vk_device, mirror_window_height, mirror_window_width) = ::std::mem::zeroed();
    /// let mut mirror_texture = ptr::null_mut();
    /// let mirror_desc = ovrMirrorTextureDesc {
    ///     Format: OVR_FORMAT_R8G8B8A8_UNORM_SRGB,
    ///     Width: mirror_window_width,
    ///     Height: mirror_window_height,
    ///     .. mem::zeroed()
    /// };
    /// let result = ovr_CreateMirrorTextureWithOptionsVk(session, vk_device, &mirror_desc as *const _, &mut mirror_texture as *mut _);
    /// # drop(result);
    /// // ...
    ///
    /// // Destroy the texture when done with it.
    /// ovr_DestroyMirrorTexture(session, mirror_texture);
    /// mirror_texture = ptr::null_mut();
    /// # drop(mirror_texture);
    /// # }
    /// ```
    ///
    /// see [`ovr_GetMirrorTextureBufferVk`](../fn.ovr_GetMirrorTextureBufferVk.html), [`ovr_DestroyMirrorTexture`](../fn.ovr_DestroyMirrorTexture.html)
    ///
    pub fn ovr_CreateMirrorTextureWithOptionsVk(
        session: ovrSession,
        device: VkDevice,
        desc: *const ovrMirrorTextureDesc,
        out_MirrorTexture: *mut ovrMirrorTexture) -> ovrResult;

    /// Get a the underlying mirror `VkImage`
    ///
    /// **in**  `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in**  `mirrorTexture` Specifies an `ovrMirrorTexture` previously returned by
    /// `ovr_CreateMirrorTextureWithOptionsVk`
    ///
    /// **out** `out_Image` Returns the `VkImage` pointer retrieved.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    /// Example code:
    ///
    /// ```no_run
    /// # extern crate ovr_sys;
    /// # extern crate vks;
    /// # fn main() {
    /// # unsafe {
    /// # use ovr_sys::vulkan::*;
    /// # use vks::*;
    /// # let (session, mirror_texture, command_buffer, present_image, region, queue, present_info) = ::std::mem::zeroed();
    /// let mut mirror_image = ::std::ptr::null_mut();
    /// ovr_GetMirrorTextureBufferVk(session, mirror_texture, &mut mirror_image as *mut _);
    ///
    /// // ...
    ///
    /// vkCmdBlitImage(command_buffer, mirror_image, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL, present_image, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &region as *const _, VK_FILTER_LINEAR);
    ///
    /// // ...
    ///
    /// vkQueuePresentKHR(queue, &present_info as *const _);
    /// # }}
    /// ```
    ///
    pub fn ovr_GetMirrorTextureBufferVk(
        session: ovrSession,
        mirrorTexture: ovrMirrorTexture,
        out_Image: *mut VkImage) -> ovrResult;
}