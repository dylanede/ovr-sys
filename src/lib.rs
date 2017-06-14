//! This crate provides raw unsafe bindings to LibOVR, the Oculus Rift runtime library.
//!
//! **The currently targeted version of LibOVR is 1.15.0**
//!
//! Good API documentation is provided here, transformed from the original doxygen documentation.
//! More high level documentation can be found on [the Oculus developer web site](https://developer.oculus.com).
//! Example code has been translated from C to mostly equivalent Rust (TODO: except in the DirectX module).
//!
//! The bindings are generated directly from the LibOVR headers, and some conventions have been
//! followed when translating from the C code.
//!
//!  * Each enum has been translated into a type definition (defined to be `i32`) and a collection of
//!    constants. This preserves the ability to easily use them as bitflags.
//!  * Alignment requirements on structs have been implemented using dummy zero-sized array fields.
//!    When initialising these fields, the value `[]` will suffice.
//!  * Where the layout of structs differs between 32-bit and 64-bit platforms due to dummy padding
//!    fields, conditional compilation is used. For this reason it is recommended to not construct the
//!    affected structs in a typical fashion as this may result in code that compiles on one but not both
//!    architectures (due to missing/spurious padding field).
//!
//!    A reasonable approach is to use `..` struct initialisation syntax, like so:
//!
//!    ```no_run
//!    # use ovr_sys::*;
//!    # use ::std::mem;
//!    # unsafe {
//!    let init_params = ovrInitParams {
//!        Flags: ovrInit_RequestVersion,
//!        RequestedMinorVersion: OVR_MINOR_VERSION,
//!        LogCallback: None,
//!        UserData: 0,
//!        ConnectionTimeoutMS: 0,
//!        .. mem::uninitialized()
//!    };
//!    # drop(init_params);
//!    # }
//!    ```
//!
//!    Like all unsafe code uses of `::std::mem::uninitialized()` should be scrutinised for mistakes.
//!  * Function-like C macros have been translated into functions with the same name.
//!
//! Optional features are provided in sub-modules. These features are `audio`, `directx`, `opengl` and `vulkan`.
//! These sub-modules will only be present if the corresponding feature has been enabled in the
//! Cargo manifest. `opengl` is enabled by default.

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

extern crate libc;

#[cfg(all(feature = "directx", windows))]
extern crate winapi;

#[cfg(feature = "vulkan")]
extern crate vks;

use libc::{
    c_char,
    c_int,
    c_uint,
    c_uint as c_unsigned,
    c_short,
    c_longlong,
    c_void
};

use ::std::fmt;

/// LibOVR functions for performing OpenGL interop.
#[cfg(feature = "opengl")]
pub mod opengl;
/// LibOVR functions for performing DirectX interop.
#[cfg(all(feature = "directx", windows))]
pub mod directx;
/// LibOVR functions for performing Vulkan interop.
#[cfg(all(feature = "vulkan"))]
pub mod vulkan;
/// LibOVR functions associated with audio functionality, including identifying audio devices and
/// converting audio data into haptics data.
#[cfg(all(feature = "audio", windows))]
pub mod audio;

pub const OVR_PRODUCT_VERSION: u32 = 1;
pub const OVR_MAJOR_VERSION: u32 = 1;
pub const OVR_MINOR_VERSION: u32 = 15;
pub const OVR_BUILD_VERSION: u32 = 0;

pub const OVR_KEY_USER: &'static [u8]                            = b"User\0";                              // string

pub const OVR_KEY_NAME: &'static [u8]                            = b"Name\0";                              // string

pub const OVR_KEY_GENDER: &'static [u8]                          = b"Gender\0";                            // string = b"Male", = b"Female", or = b"Unknown"
pub const OVR_DEFAULT_GENDER: &'static [u8]                      = b"Unknown\0";

pub const OVR_KEY_PLAYER_HEIGHT: &'static [u8]                   = b"PlayerHeight\0";                      // float meters
pub const OVR_DEFAULT_PLAYER_HEIGHT: f32                         = 1.778;

pub const OVR_KEY_EYE_HEIGHT: &'static [u8]                      = b"EyeHeight\0";                         // float meters
pub const OVR_DEFAULT_EYE_HEIGHT: f32                            = 1.675;

pub const OVR_KEY_NECK_TO_EYE_DISTANCE: &'static [u8]            = b"NeckEyeDistance\0";                   // float[2] meters
pub const OVR_DEFAULT_NECK_TO_EYE_HORIZONTAL: f32                = 0.0805;
pub const OVR_DEFAULT_NECK_TO_EYE_VERTICAL: f32                  = 0.075;

pub const OVR_KEY_EYE_TO_NOSE_DISTANCE: &'static [u8]            = b"EyeToNoseDist\0";                     // float[2] meters

pub const OVR_PERF_HUD_MODE: &'static [u8]                       = b"PerfHudMode\0";                       // int, allowed values are defined in enum ovrPerfHudMode

pub const OVR_LAYER_HUD_MODE: &'static [u8]                      = b"LayerHudMode\0";                      // int, allowed values are defined in enum ovrLayerHudMode
pub const OVR_LAYER_HUD_CURRENT_LAYER: &'static [u8]             = b"LayerHudCurrentLayer\0";              // int, The layer to show
pub const OVR_LAYER_HUD_SHOW_ALL_LAYERS: &'static [u8]           = b"LayerHudShowAll\0";                   // bool, Hide other layers when the hud is enabled

pub const OVR_DEBUG_HUD_STEREO_MODE: &'static [u8]               = b"DebugHudStereoMode\0";                // int, allowed values are defined in enum ovrDebugHudStereoMode
pub const OVR_DEBUG_HUD_STEREO_GUIDE_INFO_ENABLE: &'static [u8]  = b"DebugHudStereoGuideInfoEnable\0";     // bool
pub const OVR_DEBUG_HUD_STEREO_GUIDE_SIZE: &'static [u8]         = b"DebugHudStereoGuideSize2f\0";         // float[2]
pub const OVR_DEBUG_HUD_STEREO_GUIDE_POSITION: &'static [u8]     = b"DebugHudStereoGuidePosition3f\0";     // float[3]
pub const OVR_DEBUG_HUD_STEREO_GUIDE_YAWPITCHROLL: &'static [u8] = b"DebugHudStereoGuideYawPitchRoll3f\0"; // float[3]
pub const OVR_DEBUG_HUD_STEREO_GUIDE_COLOR: &'static [u8]        = b"DebugHudStereoGuideColor4f\0";        // float[4]

/// API call results are represented at the highest level by a single `ovrResult`.
pub type ovrResult = i32;

/// Indicates if an `ovrResult` indicates success.
///
/// Some functions return additional successful values other than `ovrSuccess` and
/// require usage of this macro to indicate successs.
///
#[inline]
pub fn OVR_SUCCESS(r: ovrResult) -> bool {
    r >= 0
}

/// Indicates if an `ovrResult` indicates an unqualified success.
///
/// This is useful for indicating that the code intentionally wants to
/// check for `result == ovrSuccess` as opposed to `OVR_SUCCESS()`, which
/// checks for `result >= ovrSuccess`.
///
#[inline]
pub fn OVR_UNQUALIFIED_SUCCESS(r: ovrResult) -> bool {
    r == ovrSuccess
}

/// Indicates if an `ovrResult` indicates failure.
///
#[inline]
pub fn OVR_FAILURE(r: ovrResult) -> bool {
    !OVR_SUCCESS(r)
}

// Success is a value greater or equal to 0, while all error types are negative values.
/// This is a general success result. Use  `OVR_SUCCESS` to test for success.
pub type ovrSuccessType = i32;
pub const ovrSuccess: ovrSuccessType = 0;

// Public success types
/// Success is a value greater or equal to 0, while all error types are negative values.
pub type ovrSuccessTypes = i32;
/// Returned from a call to SubmitFrame. The call succeeded, but what the app
/// rendered will not be visible on the HMD. Ideally the app should continue
/// calling SubmitFrame, but not do any rendering. When the result becomes
/// `ovrSuccess`, rendering should continue as usual.
pub const ovrSuccess_NotVisible: ovrSuccessTypes                 = 1000;
/// Boundary is invalid due to sensor change or was not setup.
pub const ovrSuccess_BoundaryInvalid: ovrSuccessTypes            = 1001;
/// Device is not available for the requested operation.
pub const ovrSuccess_DeviceUnavailable: ovrSuccessTypes          = 1002;

/// Public error types
pub type ovrErrorType = i32;
/* General errors */
/// Failure to allocate memory.
pub const ovrError_MemoryAllocationFailure: ovrErrorType    = -1000;
/// Invalid `ovrSession` parameter provided.
pub const ovrError_InvalidSession: ovrErrorType             = -1002;
/// The operation timed out.
pub const ovrError_Timeout: ovrErrorType                    = -1003;
/// The system or component has not been initialized.
pub const ovrError_NotInitialized: ovrErrorType             = -1004;
/// Invalid parameter provided. See error info or log for details.
pub const ovrError_InvalidParameter: ovrErrorType           = -1005;
/// Generic service error. See error info or log for details.
pub const ovrError_ServiceError: ovrErrorType               = -1006;
/// The given HMD doesn't exist.
pub const ovrError_NoHmd: ovrErrorType                      = -1007;
/// Function call is not supported on this hardware/software
pub const ovrError_Unsupported: ovrErrorType                = -1009;
/// Specified device type isn't available.
pub const ovrError_DeviceUnavailable: ovrErrorType          = -1010;
/// The headset was in an invalid orientation for the requested operation (e.g. vertically oriented during  `ovr_RecenterPose`).
pub const ovrError_InvalidHeadsetOrientation: ovrErrorType  = -1011;
/// The client failed to call `ovr_Destroy` on an active session before calling  `ovr_Shutdown`. Or the client crashed.
pub const ovrError_ClientSkippedDestroy: ovrErrorType       = -1012;
/// The client failed to call  `ovr_Shutdown` or the client crashed.
pub const ovrError_ClientSkippedShutdown: ovrErrorType      = -1013;
/// The service watchdog discovered a deadlock.
pub const ovrError_ServiceDeadlockDetected: ovrErrorType    = -1014;
/// Function call is invalid for object's current state
pub const ovrError_InvalidOperation: ovrErrorType           = -1015;

/* Audio error range, reserved for Audio errors. */
/// Failure to find the specified audio device.
pub const ovrError_AudioDeviceNotFound: ovrErrorType        = -2001;
/// Generic COM error.
pub const ovrError_AudioComError: ovrErrorType              = -2002;

/* Initialization errors. */
/// Generic initialization error.
pub const ovrError_Initialize: ovrErrorType                 = -3000;
/// Couldn't load LibOVRRT.
pub const ovrError_LibLoad: ovrErrorType                    = -3001;
/// LibOVRRT version incompatibility.
pub const ovrError_LibVersion: ovrErrorType                 = -3002;
/// Couldn't connect to the OVR Service.
pub const ovrError_ServiceConnection: ovrErrorType          = -3003;
/// OVR Service version incompatibility.
pub const ovrError_ServiceVersion: ovrErrorType             = -3004;
/// The operating system version is incompatible.
pub const ovrError_IncompatibleOS: ovrErrorType             = -3005;
/// Unable to initialize the HMD display.
pub const ovrError_DisplayInit: ovrErrorType                = -3006;
/// Unable to start the server. Is it already running?
pub const ovrError_ServerStart: ovrErrorType                = -3007;
/// Attempting to re-initialize with a different version.
pub const ovrError_Reinitialization: ovrErrorType           = -3008;
/// Chosen rendering adapters between client and service do not match
pub const ovrError_MismatchedAdapters: ovrErrorType         = -3009;
/// Calling application has leaked resources
pub const ovrError_LeakingResources: ovrErrorType           = -3010;
/// Client version too old to connect to service
pub const ovrError_ClientVersion: ovrErrorType              = -3011;
/// The operating system is out of date.
pub const ovrError_OutOfDateOS: ovrErrorType                = -3012;
/// The graphics driver is out of date.
pub const ovrError_OutOfDateGfxDriver: ovrErrorType         = -3013;
/// The graphics hardware is not supported
pub const ovrError_IncompatibleGPU: ovrErrorType            = -3014;
/// No valid VR display system found.
pub const ovrError_NoValidVRDisplaySystem: ovrErrorType     = -3015;
/// Feature or API is obsolete and no longer supported.
pub const ovrError_Obsolete: ovrErrorType                   = -3016;
/// No supported VR display system found, but disabled or driverless adapter found.
pub const ovrError_DisabledOrDefaultAdapter: ovrErrorType   = -3017;
/// The system is using hybrid graphics (Optimus, etc...), which is not support.
pub const ovrError_HybridGraphicsNotSupported: ovrErrorType = -3018;
/// Initialization of the DisplayManager failed.
pub const ovrError_DisplayManagerInit: ovrErrorType         = -3019;
/// Failed to get the interface for an attached tracker
pub const ovrError_TrackerDriverInit: ovrErrorType          = -3020;
/// LibOVRRT signature check failure.
pub const ovrError_LibSignCheck: ovrErrorType               = -3021;
/// LibOVRRT path failure.
pub const ovrError_LibPath: ovrErrorType                    = -3022;
/// LibOVRRT symbol resolution failure.
pub const ovrError_LibSymbols: ovrErrorType                 = -3023;
/// Failed to connect to the service because remote connections to the service are not allowed.
pub const ovrError_RemoteSession: ovrErrorType              = -3024;

/* Rendering errors */
/// In the event of a system-wide graphics reset or cable unplug this is returned to the app.
pub const ovrError_DisplayLost: ovrErrorType                = -6000;
///  `ovr_CommitTextureSwapChain` was called too many times on a texture swapchain without calling submit to use the chain.
pub const ovrError_TextureSwapChainFull: ovrErrorType       = -6001;
/// The `ovrTextureSwapChain` is in an incomplete or inconsistent state. Ensure  `ovr_CommitTextureSwapChain` was called at least once first.
pub const ovrError_TextureSwapChainInvalid: ovrErrorType    = -6002;
/// Graphics device has been reset (TDR, etc...)
pub const ovrError_GraphicsDeviceReset: ovrErrorType        = -6003;
/// HMD removed from the display adapter
pub const ovrError_DisplayRemoved: ovrErrorType             = -6004;
///Content protection is not available for the display
pub const ovrError_ContentProtectionNotAvailable: ovrErrorType = -6005;
/// Application declared itself as an invisible type and is not allowed to submit frames.
pub const ovrError_ApplicationInvisible: ovrErrorType       = -6006;
/// The given request is disallowed under the current conditions.
pub const ovrError_Disallowed: ovrErrorType                 = -6007;
/// Display portion of HMD is plugged into an incompatible port (ex: IGP)
pub const ovrError_DisplayPluggedIncorrectly: ovrErrorType  = -6008;

/* Fatal errors */
/// A runtime exception occurred. The application is required to shutdown LibOVR and re-initialize it before this error state will be cleared.
pub const ovrError_RuntimeException: ovrErrorType           = -7000;

/* Calibration errors */
/// Result of a missing calibration block
pub const ovrError_NoCalibration: ovrErrorType              = -9000;
/// Result of an old calibration block
pub const ovrError_OldVersion: ovrErrorType                 = -9001;
/// Result of a bad calibration block due to lengths
pub const ovrError_MisformattedBlock: ovrErrorType          = -9002;

/* Other errors */



/// Provides information about the last error.
///
/// see  [`ovr_GetLastErrorInfo`](fn.ovr_GetLastErrorInfo.html)
#[repr(C)]
#[derive(Copy)]
pub struct ovrErrorInfo {
    /// The result from the last API call that generated an error `ovrResult`.
    pub Result: ovrResult,
    /// A UTF8-encoded null-terminated English string describing the problem. The format of this string is subject to change in future versions.
    pub ErrorString: [c_char; 512],
}
impl Clone for ovrErrorInfo {
    fn clone(&self) -> ovrErrorInfo {
        *self
    }
}
impl fmt::Debug for ovrErrorInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use ::std::ffi::CStr;
        let string = unsafe{CStr::from_ptr(&self.ErrorString as *const c_char)}.to_str().unwrap();
        fmt.debug_struct("ovrErrorInfo")
            .field("Result", &self.Result)
            .field("ErrorString", &string)
            .finish()
    }
}
//-----------------------------------------------------------------------------------
// ***** ovrBool

/// Boolean type
pub type ovrBool = c_char;
/// `ovrBool` value of false.
pub const ovrFalse: ovrBool = 0;
/// `ovrBool` value of true.
pub const ovrTrue: ovrBool = 1;


//-----------------------------------------------------------------------------------
// ***** Simple Math Structures

/// A RGBA color with normalized `f32` components.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrColorf {
    pub _align: [u32; 0],
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

/// A 2D vector with integer components.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrVector2i {
    pub _align: [u32; 0],
    pub x: c_int,
    pub y: c_int,
}

/// A 2D size with integer components.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrSizei {
    pub _align: [u32; 0],
    pub w: c_int,
    pub h: c_int,
}

/// A 2D rectangle with a position and size.
/// All components are integers.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrRecti {
    pub _align: [u32; 0],
    pub Pos: ovrVector2i,
    pub Size: ovrSizei,
}

/// A quaternion rotation.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrQuatf {
    pub _align: [u32; 0],
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// A 2D vector with `f32` components.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrVector2f {
    pub _align: [u32; 0],
    pub x: f32,
    pub y: f32,
}

/// A 3D vector with `f32` components.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrVector3f {
    pub _align: [u32; 0],
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// A 4x4 matrix with `f32` elements.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrMatrix4f {
    pub _align: [u32; 0],
    M: [[f32; 4]; 4],
}


/// Position and orientation together.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrPosef {
    pub _align: [u32; 0],
    pub Orientation: ovrQuatf,
    pub Position: ovrVector3f,
}

/// A full pose (rigid body) configuration with first and second derivatives.
///
/// Body refers to any object for which `ovrPoseStatef` is providing data.
/// It can be the HMD, Touch controller, sensor or something else. The context
/// depends on the usage of the struct.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrPoseStatef {
    pub _align: [u64; 0],
    /// Position and orientation.
    pub ThePose: ovrPosef,
    /// Angular velocity in radians per second.
    pub AngularVelocity: ovrVector3f,
    /// Velocity in meters per second.
    pub LinearVelocity: ovrVector3f,
    /// Angular acceleration in radians per second per second.
    pub AngularAcceleration: ovrVector3f,
    /// Acceleration in meters per second per second.
    pub LinearAcceleration: ovrVector3f,
    /// \internal struct pad.
    pub _pad0: [u8; 4],
    /// Absolute time that this pose refers to. see  `ovr_GetTimeInSeconds`
    pub TimeInSeconds: f64,
}

/// Describes the up, down, left, and right angles of the field of view.
///
/// Field Of View (FOV) tangent of the angle units.
///
/// **Note**: For a standard 90 degree vertical FOV, we would
/// have: { UpTan = tan(90 degrees / 2), DownTan = tan(90 degrees / 2) }.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrFovPort {
    pub _align: [u32; 0],
    /// The tangent of the angle between the viewing vector and the top edge of the field of view.
    pub UpTan: f32,
    /// The tangent of the angle between the viewing vector and the bottom edge of the field of view.
    pub DownTan: f32,
    /// The tangent of the angle between the viewing vector and the left edge of the field of view.
    pub LeftTan: f32,
    /// The tangent of the angle between the viewing vector and the right edge of the field of view.
    pub RightTan: f32,
}


//-----------------------------------------------------------------------------------
// ***** HMD Types

/// Enumerates all HMD types that we support.
///
/// The currently released developer kits are `ovrHmd_DK1` and `ovrHmd_DK2`. The other enumerations are for internal use only.
pub type ovrHmdType = i32;
pub const ovrHmd_None: ovrHmdType      = 0;
pub const ovrHmd_DK1: ovrHmdType       = 3;
pub const ovrHmd_DKHD: ovrHmdType      = 4;
pub const ovrHmd_DK2: ovrHmdType       = 6;
pub const ovrHmd_CB: ovrHmdType        = 8;
pub const ovrHmd_Other: ovrHmdType     = 9;
pub const ovrHmd_E3_2015: ovrHmdType   = 10;
pub const ovrHmd_ES06: ovrHmdType      = 11;
pub const ovrHmd_ES09: ovrHmdType      = 12;
pub const ovrHmd_ES11: ovrHmdType      = 13;
pub const ovrHmd_CV1: ovrHmdType       = 14;

/// HMD capability bits reported by device.
pub type ovrHmdCaps = i32;
// Read-only flags
/// <B>(read only)</B> Specifies that the HMD is a virtual debug device.
pub const ovrHmdCap_DebugDevice: ovrHmdCaps             = 0x0010;
/// Tracking capability bits reported by the device.
/// Used with  `ovr_GetTrackingCaps`.
pub type ovrTrackingCaps = i32;
/// Supports orientation tracking (IMU).
pub const ovrTrackingCap_Orientation: ovrTrackingCaps      = 0x0010;
/// Supports yaw drift correction via a magnetometer or other means.
pub const ovrTrackingCap_MagYawCorrection: ovrTrackingCaps = 0x0020;
/// Supports positional tracking.
pub const ovrTrackingCap_Position: ovrTrackingCaps         = 0x0040;
/// Specifies which eye is being used for rendering.
/// This type explicitly does not include a third "NoStereo" monoscopic option, as such is
/// not required for an HMD-centered API.
pub type ovrEyeType = i32;
/// The left eye, from the viewer's perspective.
pub const ovrEye_Left: ovrEyeType     = 0;
/// The right eye, from the viewer's perspective.
pub const ovrEye_Right: ovrEyeType    = 1;
/// \internal Count of enumerated elements.
pub const ovrEye_Count: ovrEyeType    = 2;
/// Specifies the coordinate system `ovrTrackingState` returns tracking poses in.
/// Used with  `ovr_SetTrackingOriginType()`
pub type ovrTrackingOrigin = i32;
/// Tracking system origin reported at eye (HMD) height
///
/// Prefer using this origin when your application requires
/// matching user's current physical head pose to a virtual head pose
/// without any regards to a the height of the floor. Cockpit-based,
/// or 3rd-person experiences are ideal candidates.
/// When used, all poses in `ovrTrackingState` are reported as an offset
/// transform from the profile calibrated or recentered HMD pose.
/// It is recommended that apps using this origin type call  `ovr_RecenterTrackingOrigin`
/// prior to starting the VR experience, but notify the user before doing so
/// to make sure the user is in a comfortable pose, facing a comfortable
/// direction.
pub const ovrTrackingOrigin_EyeLevel: ovrTrackingOrigin = 0;
/// Tracking system origin reported at floor height
///
/// Prefer using this origin when your application requires the
/// physical floor height to match the virtual floor height, such as
/// standing experiences.
/// When used, all poses in `ovrTrackingState` are reported as an offset
/// transform from the profile calibrated floor pose. Calling  `ovr_RecenterTrackingOrigin`
/// will recenter the X & Z axes as well as yaw, but the Y-axis (i.e. height) will continue
/// to be reported using the floor height as the origin for all poses.
pub const ovrTrackingOrigin_FloorLevel: ovrTrackingOrigin = 1;
/// \internal Count of enumerated elements.
#[doc(hidden)]
pub const ovrTrackingOrigin_Count: ovrTrackingOrigin = 2;
/// Identifies a graphics device in a platform-specific way.
/// For Windows this is a LUID type.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrGraphicsLuid {
    pub _align: [isize; 0],
    /// Public definition reserves space for graphics API-specific implementation
    pub Reserved: [c_char; 8],
}


/// This is a complete descriptor of the HMD.
#[repr(C)]
#[derive(Copy)]
#[cfg(target_pointer_width = "32")]
pub struct ovrHmdDesc {
    pub _align: [isize; 0],
    /// The type of HMD.
    pub Type: ovrHmdType,
    /// UTF8-encoded product identification string (e.g. "Oculus Rift DK1").
    pub ProductName: [c_char; 64],
    /// UTF8-encoded HMD manufacturer identification string.
    pub Manufacturer: [c_char; 64],
    /// HID (USB) vendor identifier of the device.
    pub VendorId: c_short,
    /// HID (USB) product identifier of the device.
    pub ProductId: c_short,
    /// HMD serial number.
    pub SerialNumber: [c_char; 24],
    /// HMD firmware major version.
    pub FirmwareMajor: c_short,
    /// HMD firmware minor version.
    pub FirmwareMinor: c_short,
    /// Capability bits described by `ovrHmdCaps` which the HMD currently supports.
    pub AvailableHmdCaps: c_uint,
    /// Capability bits described by `ovrHmdCaps` which are default for the current Hmd.
    pub DefaultHmdCaps: c_uint,
    /// Capability bits described by `ovrTrackingCaps` which the system currently supports.
    pub AvailableTrackingCaps: c_uint,
    /// Capability bits described by `ovrTrackingCaps` which are default for the current system.
    pub DefaultTrackingCaps: c_uint,
    /// Defines the recommended FOVs for the HMD.
    pub DefaultEyeFov: [ovrFovPort; ovrEye_Count as usize],
    /// Defines the maximum FOVs for the HMD.
    pub MaxEyeFov: [ovrFovPort; ovrEye_Count as usize],
    /// Resolution of the full HMD screen (both eyes) in pixels.
    pub Resolution: ovrSizei,
    /// Nominal refresh rate of the display in cycles per second at the time of HMD creation.
    pub DisplayRefreshRate: f32,
}
impl Clone for ovrHmdDesc {
    fn clone(&self) -> ovrHmdDesc {
        *self
    }
}
/// This is a complete descriptor of the HMD.
#[repr(C)]
#[derive(Copy)]
#[cfg(target_pointer_width = "64")]
pub struct ovrHmdDesc {
    pub _align: [isize; 0],
    /// The type of HMD.
    pub Type: ovrHmdType,
    /// \internal struct paddding.
    pub _pad0: [u8; 4],
    /// UTF8-encoded product identification string (e.g. "Oculus Rift DK1").
    pub ProductName: [c_char; 64],
    /// UTF8-encoded HMD manufacturer identification string.
    pub Manufacturer: [c_char; 64],
    /// HID (USB) vendor identifier of the device.
    pub VendorId: c_short,
    /// HID (USB) product identifier of the device.
    pub ProductId: c_short,
    /// HMD serial number.
    pub SerialNumber: [c_char; 24],
    /// HMD firmware major version.
    pub FirmwareMajor: c_short,
    /// HMD firmware minor version.
    pub FirmwareMinor: c_short,
    /// Capability bits described by `ovrHmdCaps` which the HMD currently supports.
    pub AvailableHmdCaps: c_uint,
    /// Capability bits described by `ovrHmdCaps` which are default for the current Hmd.
    pub DefaultHmdCaps: c_uint,
    /// Capability bits described by `ovrTrackingCaps` which the system currently supports.
    pub AvailableTrackingCaps: c_uint,
    /// Capability bits described by `ovrTrackingCaps` which are default for the current system.
    pub DefaultTrackingCaps: c_uint,
    /// Defines the recommended FOVs for the HMD.
    pub DefaultEyeFov: [ovrFovPort; ovrEye_Count as usize],
    /// Defines the maximum FOVs for the HMD.
    pub MaxEyeFov: [ovrFovPort; ovrEye_Count as usize],
    /// Resolution of the full HMD screen (both eyes) in pixels.
    pub Resolution: ovrSizei,
    /// Nominal refresh rate of the display in cycles per second at the time of HMD creation.
    pub DisplayRefreshRate: f32,
    /// \internal struct paddding.
    pub _pad1: [u8; 4],
}

#[cfg(windows)]
pub type ovrProcessId = u32;
#[cfg(not(windows))]
pub type ovrProcessId = c_int;

#[doc(hidden)]
pub enum ovrHmdStruct {}
/// Used as an opaque pointer to an OVR session.
pub type ovrSession = *mut ovrHmdStruct;
/// Bit flags describing the current status of sensor tracking.
///
/// The values must be the same as in enum StatusBits
///
/// see [`ovrTrackingState`](struct.ovrTrackingState.html)
///
pub type ovrStatusBits = i32;
/// Orientation is currently tracked (connected and in use).
pub const ovrStatus_OrientationTracked: ovrStatusBits    = 0x0001;
/// Position is currently tracked (false if out of range).
pub const ovrStatus_PositionTracked: ovrStatusBits       = 0x0002;
/// Specifies the description of a single sensor.
///
/// see  [`ovr_GetTrackerDesc`](fn.ovr_GetTrackerDesc.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrTrackerDesc {
    pub _align: [isize; 0],
    /// Sensor frustum horizontal field-of-view (if present).
    pub FrustumHFovInRadians: f32,
    /// Sensor frustum vertical field-of-view (if present).
    pub FrustumVFovInRadians: f32,
    /// Sensor frustum near Z (if present).
    pub FrustumNearZInMeters: f32,
    /// Sensor frustum far Z (if present).
    pub FrustumFarZInMeters: f32,
}


/// Specifies sensor flags.
///
/// see [`ovrTrackerPose`](struct.ovrTrackerPose.html)
///
pub type ovrTrackerFlags = i32;
/// The sensor is present, else the sensor is absent or offline.
pub const ovrTracker_Connected: ovrTrackerFlags   = 0x0020;
/// The sensor has a valid pose, else the pose is unavailable. This will only be set if `ovrTracker_Connected` is set.
pub const ovrTracker_PoseTracked: ovrTrackerFlags = 0x0004;
/// Specifies the pose for a single sensor.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrTrackerPose {
    pub _align: [u64; 0],
    /// `ovrTrackerFlags`.
    pub TrackerFlags: c_uint,
    /// The sensor's pose. This pose includes sensor tilt (roll and pitch). For a leveled coordinate system use LeveledPose.
    pub Pose: ovrPosef,
    /// The sensor's leveled pose, aligned with gravity. This value includes position and yaw of the sensor, but not roll and pitch. It can be used as a reference point to render real-world objects in the correct location.
    pub LeveledPose: ovrPosef,
    /// \internal struct pad.
    pub _pad0: [u8; 4],
}


/// Tracking state at a given absolute time (describes predicted HMD pose, etc.).
/// Returned by  `ovr_GetTrackingState`.
///
/// see  [`ovr_GetTrackingState`](fn.ovr_GetTrackingState.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrTrackingState {
    pub _align: [u64; 0],
    /// Predicted head pose (and derivatives) at the requested absolute time.
    pub HeadPose: ovrPoseStatef,

    /// HeadPose tracking status described by `ovrStatusBits`.
    pub StatusFlags: c_uint,

    /// The most recent calculated pose for each hand when hand controller tracking is present.
    /// HandPoses[`ovrHand_Left` as usize] refers to the left hand and HandPoses[`ovrHand_Right` as usize] to the right hand.
    /// These values can be combined with `ovrInputState` for complete hand controller information.
    pub HandPoses: [ovrPoseStatef; 2],

    /// HandPoses status flags described by `ovrStatusBits`.
    /// Only `ovrStatus_OrientationTracked` and `ovrStatus_PositionTracked` are reported.
    pub HandStatusFlags: [c_uint; 2],

    /// The pose of the origin captured during calibration.
    /// Like all other poses here, this is expressed in the space set by `ovr_RecenterTrackingOrigin`,
    /// or `ovr_SpecifyTrackingOrigin` and so will change every time either of those functions are
    /// called. This pose can be used to calculate where the calibrated origin lands in the new
    /// recentered space. If an application never calls `ovr_RecenterTrackingOrigin` or
    /// `ovr_SpecifyTrackingOrigin`, expect this value to be the identity pose and as such will point
    /// respective origin based on `ovrTrackingOrigin` requested when calling `ovr_GetTrackingState`.
    pub CalibratedOrigin: ovrPosef,

}



/// Rendering information for each eye. Computed by  `ovr_GetRenderDesc()` based on the
/// specified FOV. Note that the rendering viewport is not included
/// here as it can be specified separately and modified per frame by
/// passing different Viewport values in the layer structure.
///
/// see  [`ovr_GetRenderDesc`](fn.ovr_GetRenderDesc.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrEyeRenderDesc {
    pub _align: [u32; 0],
    /// The eye index to which this instance corresponds.
    pub Eye: ovrEyeType,
    /// The field of view.
    pub Fov: ovrFovPort,
    /// Distortion viewport.
    pub DistortedViewport: ovrRecti,
    /// How many display pixels will fit in tan(angle) = 1.
    pub PixelsPerTanAngleAtCenter: ovrVector2f,
    /// Translation of each eye, in meters.
    pub HmdToEyeOffset: ovrVector3f,
}


/// Projection information for `ovrLayerEyeFovDepth`.
///
/// Use the utility function `ovrTimewarpProjectionDesc_FromProjection` to
/// generate this structure from the application's projection matrix.
///
/// see `ovrLayerEyeFovDepth`, `ovrTimewarpProjectionDesc_FromProjection`
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrTimewarpProjectionDesc {
    pub _align: [u32; 0],
    /// Projection matrix element [2][2].
    pub Projection22: f32,
    /// Projection matrix element [2][3].
    pub Projection23: f32,
    /// Projection matrix element [3][2].
    pub Projection32: f32,
}


/// Contains the data necessary to properly calculate position info for various layer types.
///
/// * HmdToEyeOffset is the same value pair provided in `ovrEyeRenderDesc`.
/// * HmdSpaceToWorldScaleInMeters is used to scale player motion into in-application units.
///   In other words, it is how big an in-application unit is in the player's physical meters.
///   For example, if the application uses inches as its units then HmdSpaceToWorldScaleInMeters would be 0.0254.
///   Note that if you are scaling the player in size, this must also scale. So if your application
///   units are inches, but you're shrinking the player to half their normal size, then
///   HmdSpaceToWorldScaleInMeters would be 0.0254*2.0.
///
/// see [`ovrEyeRenderDesc`](struct.ovrEyeRenderDesc.html),  [`ovr_SubmitFrame`](fn.ovr_SubmitFrame.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrViewScaleDesc {
    pub _align: [u32; 0],
    /// Translation of each eye.
    pub HmdToEyeOffset: [ovrVector3f; ovrEye_Count as usize],
    /// Ratio of viewer units to meter units.
    pub HmdSpaceToWorldScaleInMeters: f32,
}


//-----------------------------------------------------------------------------------
// ***** Platform-independent Rendering Configuration

/// The type of texture resource.
///
/// see [`ovrTextureSwapChainDesc`](struct.ovrTextureSwapChainDesc.html)
///
pub type ovrTextureType = i32;
/// 2D textures.
pub const ovrTexture_2D: ovrTextureType = 0;
/// External 2D texture. Not used on PC
pub const ovrTexture_2D_External: ovrTextureType = 1;
/// Cube maps. Not currently supported on PC.
pub const ovrTexture_Cube: ovrTextureType = 2;
pub const ovrTexture_Count: ovrTextureType = 3;
/// The bindings required for texture swap chain.
///
/// All texture swap chains are automatically bindable as shader
/// input resources since the Oculus runtime needs this to read them.
///
/// see [`ovrTextureSwapChainDesc`](struct.ovrTextureSwapChainDesc.html)
///
pub type ovrTextureBindFlags = i32;
pub const ovrTextureBind_None: ovrTextureBindFlags = 0;
/// The application can write into the chain with pixel shader
pub const ovrTextureBind_DX_RenderTarget: ovrTextureBindFlags = 0x0001;
/// The application can write to the chain with compute shader
pub const ovrTextureBind_DX_UnorderedAccess: ovrTextureBindFlags = 0x0002;
/// The chain buffers can be bound as depth and/or stencil buffers
pub const ovrTextureBind_DX_DepthStencil: ovrTextureBindFlags = 0x0004;
/// The format of a texture.
///
/// see [`ovrTextureSwapChainDesc`](struct.ovrTextureSwapChainDesc.html)
///
pub type ovrTextureFormat = i32;
pub const OVR_FORMAT_UNKNOWN: ovrTextureFormat              = 0;
/// Not currently supported on PC. Would require a DirectX 11.1 device.
pub const OVR_FORMAT_B5G6R5_UNORM: ovrTextureFormat         = 1;
/// Not currently supported on PC. Would require a DirectX 11.1 device.
pub const OVR_FORMAT_B5G5R5A1_UNORM: ovrTextureFormat       = 2;
/// Not currently supported on PC. Would require a DirectX 11.1 device.
pub const OVR_FORMAT_B4G4R4A4_UNORM: ovrTextureFormat       = 3;
pub const OVR_FORMAT_R8G8B8A8_UNORM: ovrTextureFormat       = 4;
pub const OVR_FORMAT_R8G8B8A8_UNORM_SRGB: ovrTextureFormat  = 5;
pub const OVR_FORMAT_B8G8R8A8_UNORM: ovrTextureFormat       = 6;
/// Not supported for OpenGL applications
pub const OVR_FORMAT_B8G8R8A8_UNORM_SRGB: ovrTextureFormat  = 7;
/// Not supported for OpenGL applications
pub const OVR_FORMAT_B8G8R8X8_UNORM: ovrTextureFormat       = 8;
/// Not supported for OpenGL applications
pub const OVR_FORMAT_B8G8R8X8_UNORM_SRGB: ovrTextureFormat  = 9;
pub const OVR_FORMAT_R16G16B16A16_FLOAT: ovrTextureFormat   = 10;
/// Introduced in v1.10
pub const OVR_FORMAT_R11G11B10_FLOAT: ovrTextureFormat      = 25;

// Depth formats
pub const OVR_FORMAT_D16_UNORM: ovrTextureFormat            = 11;
pub const OVR_FORMAT_D24_UNORM_S8_UINT: ovrTextureFormat    = 12;
pub const OVR_FORMAT_D32_FLOAT: ovrTextureFormat            = 13;
pub const OVR_FORMAT_D32_FLOAT_S8X24_UINT: ovrTextureFormat = 14;

// Added in 1.5 compressed formats can be used for static layers
pub const OVR_FORMAT_BC1_UNORM: ovrTextureFormat            = 15;
pub const OVR_FORMAT_BC1_UNORM_SRGB: ovrTextureFormat       = 16;
pub const OVR_FORMAT_BC2_UNORM: ovrTextureFormat            = 17;
pub const OVR_FORMAT_BC2_UNORM_SRGB: ovrTextureFormat       = 18;
pub const OVR_FORMAT_BC3_UNORM: ovrTextureFormat            = 19;
pub const OVR_FORMAT_BC3_UNORM_SRGB: ovrTextureFormat       = 20;
pub const OVR_FORMAT_BC6H_UF16: ovrTextureFormat            = 21;
pub const OVR_FORMAT_BC6H_SF16: ovrTextureFormat            = 22;
pub const OVR_FORMAT_BC7_UNORM: ovrTextureFormat            = 23;
pub const OVR_FORMAT_BC7_UNORM_SRGB: ovrTextureFormat       = 24;
/// Misc flags overriding particular behaviors of a texture swap chain
///
/// see [`ovrTextureSwapChainDesc`](struct.ovrTextureSwapChainDesc.html)
///
pub type ovrTextureMiscFlags = i32;
pub const ovrTextureMisc_None: ovrTextureMiscFlags = 0;
/// DX only: The underlying texture is created with a TYPELESS equivalent of the
/// format specified in the texture desc. The SDK will still access the
/// texture using the format specified in the texture desc, but the app can
/// create views with different formats if this is specified.
pub const ovrTextureMisc_DX_Typeless: ovrTextureMiscFlags = 0x0001;
/// DX only: Allow generation of the mip chain on the GPU via the GenerateMips
/// call. This flag requires that RenderTarget binding also be specified.
pub const ovrTextureMisc_AllowGenerateMips: ovrTextureMiscFlags = 0x0002;
/// Texture swap chain contains protected content, and requires
/// HDCP connection in order to display to HMD. Also prevents
/// mirroring or other redirection of any frame containing this contents
pub const ovrTextureMisc_ProtectedContent: ovrTextureMiscFlags = 0x0004;
/// Description used to create a texture swap chain.
///
/// see  [`ovr_CreateTextureSwapChainDX`](directx/fn.ovr_CreateTextureSwapChainDX.html), [`ovr_CreateTextureSwapChainGL`](opengl/fn.ovr_CreateTextureSwapChainGL.html), [`ovr_CreateTextureSwapChainVk`](opengl/fn.ovr_CreateTextureSwapChainVk.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrTextureSwapChainDesc {
    pub Type: ovrTextureType,
    pub Format: ovrTextureFormat,
    /// Only supported with `ovrTexture_2D`. Not supported on PC at this time.
    pub ArraySize: c_int,
    pub Width: c_int,
    pub Height: c_int,
    pub MipLevels: c_int,
    /// Current only supported on depth textures
    pub SampleCount: c_int,
    /// Not buffered in a chain. For images that don't change
    pub StaticImage: ovrBool,
    /// `ovrTextureFlags`
    pub MiscFlags: c_uint,
    /// `ovrTextureBindFlags`. Not used for GL.
    pub BindFlags: c_uint,
}

/// Description used to create a mirror texture.
///
/// see  [`ovr_CreateMirrorTextureDX`](directx/fn.ovr_CreateMirrorTextureDX.html), [`ovr_CreateMirrorTextureVk`](opengl/fn.ovr_CreateMirrorTextureVk.html), [`ovr_CreateMirrorTextureVk`](opengl/fn.ovr_CreateMirrorTextureVk.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrMirrorTextureDesc {
    pub Format: ovrTextureFormat,
    pub Width: c_int,
    pub Height: c_int,
    /// `ovrTextureFlags`
    pub MiscFlags: c_uint,
}
#[doc(hidden)]
pub enum ovrTextureSwapChainData {}
pub type ovrTextureSwapChain = *mut ovrTextureSwapChainData;
#[doc(hidden)]
pub enum ovrMirrorTextureData {}
pub type ovrMirrorTexture = *mut ovrMirrorTextureData;

//-----------------------------------------------------------------------------------

/// Describes button input types.
/// Button inputs are combined; that is they will be reported as pressed if they are
/// pressed on either one of the two devices.
/// The `ovrButton_Up`/Down/Left/Right map to both XBox D-Pad and directional buttons.
/// The `ovrButton_Enter` and `ovrButton_Return` map to Start and Back controller buttons, respectively.
pub type ovrButton = i32;
/// A button on XBox controllers and right Touch controller. Select button on Oculus Remote.
pub const ovrButton_A: ovrButton         = 0x00000001;
/// B button on XBox controllers and right Touch controller. Back button on Oculus Remote.
pub const ovrButton_B: ovrButton         = 0x00000002;
/// Right thumbstick on XBox controllers and Touch controllers. Not present on Oculus Remote.
pub const ovrButton_RThumb: ovrButton    = 0x00000004;
/// Right shoulder button on XBox controllers. Not present on Touch controllers or Oculus Remote.
pub const ovrButton_RShoulder: ovrButton = 0x00000008;

/// X button on XBox controllers and left Touch controller. Not present on Oculus Remote.
pub const ovrButton_X: ovrButton         = 0x00000100;
/// Y button on XBox controllers and left Touch controller. Not present on Oculus Remote.
pub const ovrButton_Y: ovrButton         = 0x00000200;
/// Left thumbstick on XBox controllers and Touch controllers. Not present on Oculus Remote.
pub const ovrButton_LThumb: ovrButton    = 0x00000400;
/// Left shoulder button on XBox controllers. Not present on Touch controllers or Oculus Remote.
pub const ovrButton_LShoulder: ovrButton = 0x00000800;

/// Up button on XBox controllers and Oculus Remote. Not present on Touch controllers.
pub const ovrButton_Up: ovrButton        = 0x00010000;
/// Down button on XBox controllers and Oculus Remote. Not present on Touch controllers.
pub const ovrButton_Down: ovrButton      = 0x00020000;
/// Left button on XBox controllers and Oculus Remote. Not present on Touch controllers.
pub const ovrButton_Left: ovrButton      = 0x00040000;
/// Right button on XBox controllers and Oculus Remote. Not present on Touch controllers.
pub const ovrButton_Right: ovrButton     = 0x00080000;
/// Start on XBox 360 controller. Menu on XBox One controller and Left Touch controller. Should be referred to as the Menu button in user-facing documentation.
pub const ovrButton_Enter: ovrButton     = 0x00100000;
/// Back on Xbox 360 controller. View button on XBox One controller. Not present on Touch controllers or Oculus Remote.
pub const ovrButton_Back: ovrButton      = 0x00200000;
/// Volume button on Oculus Remote. Not present on XBox or Touch controllers.
pub const ovrButton_VolUp: ovrButton     = 0x00400000;
/// Volume button on Oculus Remote. Not present on XBox or Touch controllers.
pub const ovrButton_VolDown: ovrButton   = 0x00800000;
/// Home button on XBox controllers. Oculus button on Touch controllers and Oculus Remote.
pub const ovrButton_Home: ovrButton      = 0x01000000;

/// Bit mask of all buttons that are for private usage by Oculus
pub const ovrButton_Private: ovrButton   = ovrButton_VolUp | ovrButton_VolDown | ovrButton_Home;

/// Bit mask of all buttons on the right Touch controller
pub const ovrButton_RMask: ovrButton = ovrButton_A | ovrButton_B | ovrButton_RThumb | ovrButton_RShoulder;

/// Bit mask of all buttons on the left Touch controller
pub const ovrButton_LMask: ovrButton = ovrButton_X | ovrButton_Y | ovrButton_LThumb | ovrButton_LShoulder |
    ovrButton_Enter;
/// Describes touch input types.
/// These values map to capacitive touch values reported `ovrInputState`::Touch.
/// Some of these values are mapped to button bits for consistency.
pub type ovrTouch = i32;
pub const ovrTouch_A: ovrTouch              = ovrButton_A;
pub const ovrTouch_B: ovrTouch              = ovrButton_B;
pub const ovrTouch_RThumb: ovrTouch         = ovrButton_RThumb;
pub const ovrTouch_RThumbRest: ovrTouch     = 0x00000008;
pub const ovrTouch_RIndexTrigger: ovrTouch  = 0x00000010;

/// Bit mask of all the button touches on the right controller
pub const ovrTouch_RButtonMask: ovrTouch    = ovrTouch_A | ovrTouch_B | ovrTouch_RThumb | ovrTouch_RThumbRest | ovrTouch_RIndexTrigger;

pub const ovrTouch_X: ovrTouch              = ovrButton_X;
pub const ovrTouch_Y: ovrTouch              = ovrButton_Y;
pub const ovrTouch_LThumb: ovrTouch         = ovrButton_LThumb;
pub const ovrTouch_LThumbRest: ovrTouch     = 0x00000800;
pub const ovrTouch_LIndexTrigger: ovrTouch  = 0x00001000;

/// Bit mask of all the button touches on the left controller
pub const ovrTouch_LButtonMask: ovrTouch    = ovrTouch_X | ovrTouch_Y | ovrTouch_LThumb | ovrTouch_LThumbRest | ovrTouch_LIndexTrigger;

/// Finger pose state
/// Derived internally based on distance, proximity to sensors and filtering.
pub const ovrTouch_RIndexPointing: ovrTouch = 0x00000020;
pub const ovrTouch_RThumbUp: ovrTouch       = 0x00000040;
pub const ovrTouch_LIndexPointing: ovrTouch = 0x00002000;
pub const ovrTouch_LThumbUp: ovrTouch       = 0x00004000;

/// Bit mask of all right controller poses
pub const ovrTouch_RPoseMask: ovrTouch      = ovrTouch_RIndexPointing | ovrTouch_RThumbUp;

/// Bit mask of all left controller poses
pub const ovrTouch_LPoseMask: ovrTouch      = ovrTouch_LIndexPointing | ovrTouch_LThumbUp;
/// Describes the Touch Haptics engine.
/// Currently, those values will NOT change during a session.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrTouchHapticsDesc {
    pub _align: [isize; 0],
    /// Haptics engine frequency/sample-rate, sample time in seconds equals 1.0/sampleRateHz
    pub SampleRateHz: c_int,
    /// Size of each Haptics sample, sample value range is `[0, 2^(Bytes*8)-1]`
    pub SampleSizeInBytes: c_int,

    /// Queue size that would guarantee Haptics engine would not starve for data
    /// Make sure size doesn't drop below it for best results
    pub QueueMinSizeToAvoidStarvation: c_int,

    /// Minimum, Maximum and Optimal number of samples that can be sent to Haptics through  `ovr_SubmitControllerVibration`
    pub SubmitMinSamples: c_int,
    pub SubmitMaxSamples: c_int,
    pub SubmitOptimalSamples: c_int,
}

/// Specifies which controller is connected; multiple can be connected at once.
pub type ovrControllerType = i32;
pub const ovrControllerType_None: ovrControllerType      = 0x00;
pub const ovrControllerType_LTouch: ovrControllerType    = 0x01;
pub const ovrControllerType_RTouch: ovrControllerType    = 0x02;
pub const ovrControllerType_Touch: ovrControllerType     = 0x03;
pub const ovrControllerType_Remote: ovrControllerType    = 0x04;
pub const ovrControllerType_XBox: ovrControllerType      = 0x10;
/// Operate on or query whichever controller is active.
pub const ovrControllerType_Active: ovrControllerType    = 0xff;
/// Haptics buffer submit mode
pub type ovrHapticsBufferSubmitMode = i32;
/// Enqueue buffer for later playback
pub const ovrHapticsBufferSubmit_Enqueue: ovrHapticsBufferSubmitMode = 0;
/// Haptics buffer descriptor, contains amplitude samples used for Touch vibration
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrHapticsBuffer {
    /// Samples stored in opaque format
    pub Samples: *const c_void,
    /// Number of samples
    pub SamplesCount: c_int,
    /// How samples are submitted to the hardware
    pub SubmitMode: ovrHapticsBufferSubmitMode,
}

/// State of the Haptics playback for Touch vibration
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrHapticsPlaybackState {
    /// Remaining space available to queue more samples
    pub RemainingQueueSpace: c_int,

    /// Number of samples currently queued
    pub SamplesQueued: c_int,
}

/// Position tracked devices
pub type ovrTrackedDeviceType = i32;
pub const ovrTrackedDevice_HMD: ovrTrackedDeviceType        = 0x0001;
pub const ovrTrackedDevice_LTouch: ovrTrackedDeviceType     = 0x0002;
pub const ovrTrackedDevice_RTouch: ovrTrackedDeviceType     = 0x0004;
pub const ovrTrackedDevice_Touch: ovrTrackedDeviceType      = 0x0006;
pub const ovrTrackedDevice_All: ovrTrackedDeviceType        = 0xFFFF;
/// Boundary types that specified while using the boundary system
pub type ovrBoundaryType = i32;
/// Outer boundary - closely represents user setup walls
pub const ovrBoundary_Outer: ovrBoundaryType           = 0x0001;

/// Play area - safe rectangular area inside outer boundary which can optionally be used to restrict user interactions and motion.
pub const ovrBoundary_PlayArea: ovrBoundaryType        = 0x0100;
/// Boundary system look and feel
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrBoundaryLookAndFeel {
    /// Boundary color (alpha channel is ignored)
    pub Color: ovrColorf,
}

/// Provides boundary test information
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrBoundaryTestResult {
    /// True if the boundary system is being triggered. Note that due to fade in/out effects this may not exactly match visibility.
    pub IsTriggering: ovrBool,

    /// Distance to the closest play area or outer boundary surface.
    pub ClosestDistance: f32,

    /// Closest point on the boundary surface.
    pub ClosestPoint: ovrVector3f,

    /// Unit surface normal of the closest boundary surface.
    pub ClosestPointNormal: ovrVector3f,
}

/// Provides names for the left and right hand array indexes.
///
/// see [`ovrInputState`](struct.ovrInputState.html), [`ovrTrackingState`](struct.ovrTrackingState.html)
///
pub type ovrHandType = i32;
pub const ovrHand_Left: ovrHandType  = 0;
pub const ovrHand_Right: ovrHandType = 1;
pub const ovrHand_Count: ovrHandType = 2;
/// `ovrInputState` describes the complete controller input state, including Oculus Touch,
/// and XBox gamepad. If multiple inputs are connected and used at the same time,
/// their inputs are combined.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrInputState {
    /// System type when the controller state was last updated.
    pub TimeInSeconds: f64,

    /// Values for buttons described by `ovrButton`.
    pub Buttons: c_uint,

    /// Touch values for buttons and sensors as described by `ovrTouch`.
    pub Touches: c_uint,

    /// Left and right finger trigger values (`ovrHand_Left` and `ovrHand_Right`), in the range 0.0 to 1.0f.
    /// Returns 0 if the value would otherwise be less than 0.1176, for `ovrControllerType_XBox`.
    /// This has been formally named simply "Trigger". We retain the name IndexTrigger for backwards code compatibility.
    /// User-facing documentation should refer to it as the Trigger.
    pub IndexTrigger: [f32; ovrHand_Count as usize],

    /// Left and right hand trigger values (`ovrHand_Left` and `ovrHand_Right`), in the range 0.0 to 1.0f.
    /// This has been formally named "Grip Button". We retain the name HandTrigger for backwards code compatibility.
    /// User-facing documentation should refer to it as the Grip Button or simply Grip.
    pub HandTrigger: [f32; ovrHand_Count as usize],

    /// Horizontal and vertical thumbstick axis values (`ovrHand_Left` and `ovrHand_Right`), in the range -1.0f to 1.0f.
    /// Returns a deadzone (value 0) per each axis if the value on that axis would otherwise have been between -.2746 to +.2746, for `ovrControllerType_XBox`
    pub Thumbstick: [ovrVector2f; ovrHand_Count as usize],

    /// The type of the controller this state is for.
    pub ControllerType: ovrControllerType,

    /// Left and right finger trigger values (`ovrHand_Left` and `ovrHand_Right`), in the range 0.0 to 1.0f.
    /// Does not apply a deadzone.  Only touch applies a filter.
    /// This has been formally named simply "Trigger". We retain the name IndexTrigger for backwards code compatibility.
    /// User-facing documentation should refer to it as the Trigger.
    /// Added in 1.7
    pub IndexTriggerNoDeadzone: [f32; ovrHand_Count as usize],

    /// Left and right hand trigger values (`ovrHand_Left` and `ovrHand_Right`), in the range 0.0 to 1.0f.
    /// Does not apply a deadzone. Only touch applies a filter.
    /// This has been formally named "Grip Button". We retain the name HandTrigger for backwards code compatibility.
    /// User-facing documentation should refer to it as the Grip Button or simply Grip.
    /// Added in 1.7
    pub HandTriggerNoDeadzone: [f32; ovrHand_Count as usize],

    /// Horizontal and vertical thumbstick axis values (`ovrHand_Left` and `ovrHand_Right`), in the range -1.0f to 1.0f
    /// Does not apply a deadzone or filter.
    /// Added in 1.7
    pub ThumbstickNoDeadzone: [ovrVector2f; ovrHand_Count as usize],

    /// Left and right finger trigger values (`ovrHand_Left` and `ovrHand_Right`), in range 0.0 to 1.0f.
    /// No deadzone or filter
    /// This has been formally named "Grip Button". We retain the name HandTrigger for backwards code
    /// compatibility.
    /// User-facing documentation should refer to it as the Grip Button or simply Grip.
    pub IndexTriggerRaw: [f32; ovrHand_Count as usize],

    /// Left and right hand trigger values (`ovrHand_Left` and `ovrHand_Right`), in the range 0.0 to 1.0f.
    /// No deadzone or filter
    /// This has been formally named "Grip Button". We retain the name HandTrigger for backwards code
    /// compatibility.
    /// User-facing documentation should refer to it as the Grip Button or simply Grip.
    pub HandTriggerRaw: [f32; ovrHand_Count as usize],

    /// Horizontal and vertical thumbstick axis values (`ovrHand_Left` and `ovrHand_Right`), in the range
    /// -1.0f to 1.0f
    /// No deadzone or filter
    pub ThumbstickRaw: [ovrVector2f; ovrHand_Count as usize]
}



//-----------------------------------------------------------------------------------
// ***** Initialize structures

/// Initialization flags.
///
/// see [`ovrInitParams`](struct.ovrInitParams.html),  [`ovr_Initialize`](fn.ovr_Initialize.html)
///
pub type ovrInitFlags = i32;
/// When a debug library is requested, a slower debugging version of the library will
/// run which can be used to help solve problems in the library and debug application code.
pub const ovrInit_Debug: ovrInitFlags          = 0x00000001;
/// When a version is requested, the LibOVR runtime respects the RequestedMinorVersion
/// field and verifies that the RequestedMinorVersion is supported. Normally when you
/// specify this flag you simply use  `OVR_MINOR_VERSION` for `ovrInitParams`::RequestedMinorVersion,
/// though you could use a lower version than  `OVR_MINOR_VERSION` to specify previous
/// version behavior.
pub const ovrInit_RequestVersion: ovrInitFlags = 0x00000004;

/// These bits are writable by user code.
pub const ovrinit_WritableBits: ovrInitFlags   = 0x00ffffff;
/// Logging levels
///
/// see [`ovrInitParams`](struct.ovrInitParams.html), [`ovrLogCallback`](type.ovrLogCallback.html)
///
pub type ovrLogLevel = i32;
/// Debug-level log event.
pub const ovrLogLevel_Debug: ovrLogLevel    = 0;
/// Info-level log event.
pub const ovrLogLevel_Info: ovrLogLevel     = 1;
/// Error-level log event.
pub const ovrLogLevel_Error: ovrLogLevel    = 2;
/// Signature of the logging callback function pointer type.
///
/// `userData` is an arbitrary value specified by the user of `ovrInitParams`.
/// `level` is one of the `ovrLogLevel` constants.
/// `message` is a UTF8-encoded null-terminated string.
/// see [`ovrInitParams`](struct.ovrInitParams.html), [`ovrLogLevel`](type.ovrLogLevel.html),  [`ovr_Initialize`](fn.ovr_Initialize.html)
///

pub type ovrLogCallback = Option<extern "C" fn(usize, c_int, *const c_char)>;

/// Parameters for  `ovr_Initialize`.
///
/// see  [`ovr_Initialize`](fn.ovr_Initialize.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg(target_pointer_width = "32")]
pub struct ovrInitParams {
    pub _align: [u64; 0],
    /// Flags from `ovrInitFlags` to override default behavior.
    /// Use 0 for the defaults.
    pub Flags: ovrInitFlags,

    /// Requests a specific minor version of the LibOVR runtime.
    /// Flags must include `ovrInit_RequestVersion` or this will be ignored and  `OVR_MINOR_VERSION`
    /// will be used. If you are directly calling the LibOVRRT version of  `ovr_Initialize`
    /// in the LibOVRRT DLL then this must be valid and include `ovrInit_RequestVersion`.
    pub RequestedMinorVersion: u32,

    /// User-supplied log callback function, which may be called at any time
    /// asynchronously from multiple threads until  `ovr_Shutdown` completes.
    /// Use NULL to specify no log callback.
    pub LogCallback: ovrLogCallback,

    /// User-supplied data which is passed as-is to LogCallback. Typically this
    /// is used to store an application-specific pointer which is read in the
    /// callback function.
    pub UserData: usize,

    /// Relative number of milliseconds to wait for a connection to the server
    /// before failing. Use 0 for the default timeout.
    pub ConnectionTimeoutMS: u32,

}

/// Parameters for  `ovr_Initialize`.
///
/// see  [`ovr_Initialize`](fn.ovr_Initialize.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg(target_pointer_width = "64")]
pub struct ovrInitParams {
    pub _align: [u64; 0],
    /// Flags from `ovrInitFlags` to override default behavior.
    /// Use 0 for the defaults.
    pub Flags: ovrInitFlags,

    /// Requests a specific minor version of the LibOVR runtime.
    /// Flags must include `ovrInit_RequestVersion` or this will be ignored and  `OVR_MINOR_VERSION`
    /// will be used. If you are directly calling the LibOVRRT version of  `ovr_Initialize`
    /// in the LibOVRRT DLL then this must be valid and include `ovrInit_RequestVersion`.
    pub RequestedMinorVersion: u32,

    /// User-supplied log callback function, which may be called at any time
    /// asynchronously from multiple threads until  `ovr_Shutdown` completes.
    /// Use NULL to specify no log callback.
    pub LogCallback: ovrLogCallback,

    /// User-supplied data which is passed as-is to LogCallback. Typically this
    /// is used to store an application-specific pointer which is read in the
    /// callback function.
    pub UserData: usize,

    /// Relative number of milliseconds to wait for a connection to the server
    /// before failing. Use 0 for the default timeout.
    pub ConnectionTimeoutMS: u32,

    /// \internal
    pub _pad0: [u8; 4],

}

extern "C" {

    // -----------------------------------------------------------------------------------
    // ***** API Interfaces

    /// Initializes LibOVR
    ///
    /// Initialize LibOVR for application usage. This includes finding and loading the LibOVRRT
    /// shared library. No LibOVR API functions, other than `ovr_GetLastErrorInfo` and  `ovr_Detect`, can
    /// be called unless `ovr_Initialize` succeeds. A successful call to  `ovr_Initialize` must be eventually
    /// followed by a call to `ovr_Shutdown`.  `ovr_Initialize` calls are idempotent.
    /// Calling `ovr_Initialize` twice does not require two matching calls to  `ovr_Shutdown`.
    /// If already initialized, the return value is  `ovr_Success`.
    ///
    /// LibOVRRT shared library search order:
    ///
    /// * Current working directory (often the same as the application directory).
    /// * Module directory (usually the same as the application directory,
    ///   but not if the module is a separate shared library).
    /// * Application directory
    /// * Development directory (only if  `OVR_ENABLE_DEVELOPER_SEARCH` is enabled,
    ///   which is off by default).
    /// * Standard OS shared library search location(s) (OS-specific).
    ///
    /// `params` Specifies custom initialization options. May be NULL to indicate default options when
    ///        using the CAPI shim. If you are directly calling the LibOVRRT version of  `ovr_Initialize`
    ///         in the LibOVRRT DLL then this must be valid and include ovrInit_RequestVersion.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///          `ovr_GetLastErrorInfo` to get more information. Example failed results include:
    ///
    /// * `ovrError_Initialize`: Generic initialization error.
    /// * `ovrError_LibLoad`: Couldn't load LibOVRRT.
    /// * `ovrError_LibVersion`: LibOVRRT version incompatibility.
    /// * `ovrError_ServiceConnection`: Couldn't connect to the OVR Service.
    /// * `ovrError_ServiceVersion`: OVR Service version incompatibility.
    /// * `ovrError_IncompatibleOS`: The operating system version is incompatible.
    /// * `ovrError_DisplayInit`: Unable to initialize the HMD display.
    /// * `ovrError_ServerStart`:  Unable to start the server. Is it already running?
    /// * `ovrError_Reinitialization`: Attempted to re-initialize with a different version.
    ///
    /// **Example code**
    ///
    /// ```no_run
    /// # extern crate libc;
    /// # extern crate ovr_sys;
    /// # use ovr_sys::*;
    /// # use ::std::ffi::CStr;
    /// # use libc::c_char;
    /// # use ::std::mem;
    /// # fn main() {
    /// # unsafe fn foo() -> Result<(), String> {
    /// let init_params = ovrInitParams {
    ///     Flags: ovrInit_RequestVersion,
    ///     RequestedMinorVersion: OVR_MINOR_VERSION,
    ///     LogCallback: None,
    ///     UserData: 0,
    ///     ConnectionTimeoutMS: 0,
    ///     .. mem::uninitialized()
    /// };
    /// let result = ovr_Initialize(&init_params as *const _);
    /// if OVR_FAILURE(result) {
    ///     let mut error_info: ovrErrorInfo = mem::zeroed();
    ///     ovr_GetLastErrorInfo(&mut error_info as *mut _);
    ///     let error_string = CStr::from_ptr(&error_info.ErrorString as *const c_char)
    ///         .to_str().unwrap();
    ///     return Err(format!("ovr_Initialize failed: {}", error_string));
    /// }
    /// # Ok(())
    /// # }
    /// # unsafe{drop(foo())};
    /// # }
    /// ```
    ///
    /// see  [`ovr_Shutdown`](fn.ovr_Shutdown.html)
    ///
    pub fn ovr_Initialize(params: *const ovrInitParams) -> ovrResult;
    /// Shuts down LibOVR
    ///
    /// A successful call to `ovr_Initialize` must be eventually matched by a call to  `ovr_Shutdown`.
    /// After calling `ovr_Shutdown`, no LibOVR functions can be called except  `ovr_GetLastErrorInfo`
    /// or another `ovr_Initialize`.  `ovr_Shutdown` invalidates all pointers, references, and created objects
    /// previously returned by LibOVR functions. The LibOVRRT shared library can be unloaded by
    ///  `ovr_Shutdown`.
    ///
    /// see  [`ovr_Initialize`](fn.ovr_Initialize.html)
    ///
    pub fn ovr_Shutdown();
    /// Returns information about the most recent failed return value by the
    /// current thread for this library.
    ///
    /// This function itself can never generate an error.
    /// The last error is never cleared by LibOVR, but will be overwritten by new errors.
    /// Do not use this call to determine if there was an error in the last API
    /// call as successful API calls don't clear the last `ovrErrorInfo`.
    /// To avoid any inconsistency,  `ovr_GetLastErrorInfo` should be called immediately
    /// after an API function that returned a failed `ovrResult`, with no other API
    /// functions called in the interim.
    ///
    /// **out** `errorInfo` The last `ovrErrorInfo` for the current thread.
    ///
    /// see [`ovrErrorInfo`](struct.ovrErrorInfo.html)
    ///
    pub fn ovr_GetLastErrorInfo(errorInfo: *mut ovrErrorInfo);
    /// Returns the version string representing the LibOVRRT version.
    ///
    /// The returned string pointer is valid until the next call to  `ovr_Shutdown`.
    ///
    /// Note that the returned version string doesn't necessarily match the current
    ///  `OVR_MAJOR_VERSION`, etc., as the returned string refers to the LibOVRRT shared
    /// library version and not the locally compiled interface version.
    ///
    /// The format of this string is subject to change in future versions and its contents
    /// should not be interpreted.
    ///
    /// Returns a UTF8-encoded null-terminated version string.
    ///
    pub fn ovr_GetVersionString() -> *const c_char;
    /// Writes a message string to the LibOVR tracing mechanism (if enabled).
    ///
    /// This message will be passed back to the application via the `ovrLogCallback` if
    /// it was registered.
    ///
    /// `level` One of the `ovrLogLevel` constants.
    ///
    /// `message` A UTF8-encoded null-terminated string.
    ///
    /// Returns the strlen of the message or a negative value if the message is too large.
    ///
    /// see [`ovrLogLevel`](type.ovrLogLevel.html), [`ovrLogCallback`](type.ovrLogCallback.html)
    ///
    pub fn ovr_TraceMessage(level: c_int, message: *const c_char) -> c_int;
    /// Identify client application info.
    ///
    /// The string is one or more newline-delimited lines of optional info
    /// indicating engine name, engine version, engine plugin name, engine plugin
    /// version, engine editor. The order of the lines is not relevant. Individual
    /// lines are optional. A newline is not necessary at the end of the last line.
    ///
    /// Call after `ovr_Initialize` and before the first call to  `ovr_Create`.
    ///
    /// Each value is limited to 20 characters. Key names such as 'EngineName:', 'EngineVersion:' do not count towards this limit.
    ///
    /// `identity` Specifies one or more newline-delimited lines of optional info:
    ///
    /// ```text
    /// EngineName: %s\n
    /// EngineVersion: %s\n
    /// EnginePluginName: %s\n
    /// EnginePluginVersion: %s\n
    /// EngineEditor: <boolean> ('true' or 'false')\n
    /// ```
    ///
    /// **Example code**
    ///
    /// ```no_run
    /// # use ovr_sys::*;
    /// # use ::std::ffi::CStr;
    /// # unsafe {
    /// let identity = b"EngineName: Unity\n\
    ///                  EngineVersion: 5.3.3\n\
    ///                  EnginePluginName: `OVRPlugin`\n\
    ///                  EnginePluginVersion: 1.2.0\n\
    ///                  EngineEditor: true\0";
    /// ovr_IdentifyClient(CStr::from_bytes_with_nul_unchecked(identity).as_ptr());
    ///  # }
    /// ```
    ///
    pub fn ovr_IdentifyClient(identity: *const c_char) -> ovrResult;


    //-------------------------------------------------------------------------------------
    // @name HMD Management
    //
    // Handles the enumeration, creation, destruction, and properties of an HMD (head-mounted display).
    //@{


    /// Returns information about the current HMD.
    ///
    ///  `ovr_Initialize` must have first been called in order for this to succeed, otherwise `ovrHmdDesc`::Type
    /// will be reported as `ovrHmd_None`.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`, else NULL in which
    ///                case this function detects whether an HMD is present and returns its info if so.
    ///
    /// Returns an ovrHmdDesc. If the hmd is NULL and `ovrHmdDesc::Type` is `ovrHmd_None` then
    ///         no HMD is present.
    ///
    pub fn ovr_GetHmdDesc(session: ovrSession) -> ovrHmdDesc;
    /// Returns the number of attached trackers.
    ///
    /// The number of trackers may change at any time, so this function should be called before use
    /// as opposed to once on startup.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    pub fn ovr_GetTrackerCount(session: ovrSession) -> c_uint;
    /// Returns a given attached tracker description.
    ///
    ///  `ovr_Initialize` must have first been called in order for this to succeed, otherwise the returned
    /// trackerDescArray will be zero-initialized. The data returned by this function can change at runtime.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// `trackerDescIndex` Specifies a tracker index. The valid indexes are in the range of 0 to
    ///            the tracker count returned by  `ovr_GetTrackerCount`.
    ///
    /// Returns `ovrTrackerDesc`. An empty `ovrTrackerDesc` will be returned if `trackerDescIndex` is out of range.
    ///
    /// see [`ovrTrackerDesc`](struct.ovrTrackerDesc.html),  [`ovr_GetTrackerCount`](fn.ovr_GetTrackerCount.html)
    ///
    pub fn ovr_GetTrackerDesc(session: ovrSession, trackerDescIndex: c_uint) -> ovrTrackerDesc;
    /// Creates a handle to a VR session.
    ///
    /// Upon success the returned `ovrSession` must be eventually freed with  `ovr_Destroy` when it is no longer needed.
    ///
    /// A second call to  `ovr_Create` will result in an error return value if the previous session has not been destroyed.
    ///
    /// **out** `pSession` Provides a pointer to an `ovrSession` which will be written to upon success.
    ///
    /// **out** `luid` Provides a system specific graphics adapter identifier that locates which
    /// graphics adapter has the HMD attached. This must match the adapter used by the application
    /// or no rendering output will be possible. This is important for stability on multi-adapter systems. An
    /// application that simply chooses the default adapter will not run reliably on multi-adapter systems.
    ///
    /// Returns an `ovrResult` indicating success or failure. Upon failure
    ///         the returned `ovrSession` will be NULL.
    ///
    /// *Example code*
    ///
    /// ```no_run
    /// # use ovr_sys::*;
    /// # use ::std::mem;
    /// # unsafe {
    /// let mut session: ovrSession = mem::uninitialized();
    /// let mut luid: ovrGraphicsLuid = mem::uninitialized();
    /// let result = ovr_Create(&mut session as *mut _, &mut luid as *mut _);
    /// if OVR_FAILURE(result) {
    ///     // handle error
    /// }
    /// # }
    /// ```
    ///
    /// see  [`ovr_Destroy`](fn.ovr_Destroy.html)
    ///
    pub fn ovr_Create(pSession: *mut ovrSession, pLuid: *mut ovrGraphicsLuid) -> ovrResult;
    /// Destroys the session.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// see  [`ovr_Create`](fn.ovr_Create.html)
    ///
    pub fn ovr_Destroy(session: ovrSession);

}

/// Specifies status information for the current session.
///
/// see  [`ovr_GetSessionStatus`](fn.ovr_GetSessionStatus.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrSessionStatus {
    /// True if the process has VR focus and thus is visible in the HMD.
    pub IsVisible: ovrBool,
    /// True if an HMD is present.
    pub HmdPresent: ovrBool,
    /// True if the HMD is on the user's head.
    pub HmdMounted: ovrBool,
    /// True if the session is in a display-lost state. See  `ovr_SubmitFrame`.
    pub DisplayLost: ovrBool,
    /// True if the application should initiate shutdown.
    pub ShouldQuit: ovrBool,
    /// True if UX has requested re-centering. Must call `ovr_ClearShouldRecenterFlag`, `ovr_RecenterTrackingOrigin` or `ovr_SpecifyTrackingOrigin`.
    pub ShouldRecenter: ovrBool,
}

extern "C" {

    /// Returns status information for the application.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// **out** `sessionStatus` Provides an `ovrSessionStatus` that is filled in.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of
    ///         failure, use  `ovr_GetLastErrorInfo` to get more information.
    ///          Return values include but aren't limited to:
    ///     * `ovrSuccess`: Completed successfully.
    ///     * `ovrError_ServiceConnection`: The service connection was lost and the application
    ///        must destroy the session.
    ///
    pub fn ovr_GetSessionStatus(session: ovrSession, sessionStatus: *mut ovrSessionStatus) -> ovrResult;


    //@}



    //-------------------------------------------------------------------------------------
    // @name Tracking
    //
    // Tracking functions handle the position, orientation, and movement of the HMD in space.
    //
    // All tracking interface functions are thread-safe, allowing tracking state to be sampled
    // from different threads.
    //
    //@{



    /// Sets the tracking origin type
    ///
    /// When the tracking origin is changed, all of the calls that either provide
    /// or accept `ovrPosef` will use the new tracking origin provided.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// `origin` Specifies an `ovrTrackingOrigin` to be used for all `ovrPosef`
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///          `ovr_GetLastErrorInfo` to get more information.
    ///
    /// see [`ovrTrackingOrigin`](type.ovrTrackingOrigin.html),  [`ovr_GetTrackingOriginType`](fn.ovr_GetTrackingOriginType.html)
    pub fn ovr_SetTrackingOriginType(session: ovrSession, origin: ovrTrackingOrigin) -> ovrResult;
    /// Gets the tracking origin state
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// Returns the `ovrTrackingOrigin` that was either set by default, or previous set by the application.
    ///
    /// see [`ovrTrackingOrigin`](type.ovrTrackingOrigin.html),  [`ovr_SetTrackingOriginType`](fn.ovr_SetTrackingOriginType.html)
    pub fn ovr_GetTrackingOriginType(session: ovrSession) -> ovrTrackingOrigin;
    /// Re-centers the sensor position and orientation.
    ///
    /// This resets the (x,y,z) positional components and the yaw orientation component of the
    /// tracking space for the HMD and controllers using the HMD's current tracking pose.
    /// If the caller requires some tweaks on top of the HMD's current tracking pose, consider using
    /// `ovr_SpecifyTrackingOrigin` instead.
    ///
    /// The Roll and pitch orientation components are always determined by gravity and cannot
    /// be redefined. All future tracking will report values relative to this new reference position.
    /// If you are using `ovrTrackerPoses` then you will need to call  `ovr_GetTrackerPose` after
    /// this, because the sensor position(s) will change as a result of this.
    ///
    /// The headset cannot be facing vertically upward or downward but rather must be roughly
    /// level otherwise this function will fail with `ovrError_InvalidHeadsetOrientation`.
    ///
    /// For more info, see the notes on each `ovrTrackingOrigin` enumeration to understand how
    /// recenter will vary slightly in its behavior based on the current `ovrTrackingOrigin` setting.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///          `ovr_GetLastErrorInfo` to get more information. Return values include but aren't limited to:
    ///     * `ovrSuccess`: Completed successfully.
    ///     * `ovrError_InvalidHeadsetOrientation`: The headset was facing an invalid direction when
    ///       attempting recentering, such as facing vertically.
    ///
    /// see [`ovrTrackingOrigin`](type.ovrTrackingOrigin.html),  [`ovr_GetTrackerPose`](fn.ovr_GetTrackerPose.html), [`ovr_SpecifyTrackingOrigin`](fn.ovr_SpecifyTrackingOrigin.html)
    ///
    pub fn ovr_RecenterTrackingOrigin(session: ovrSession) -> ovrResult;
    /// Allows manually tweaking the sensor position and orientation.
    ///
    /// This function is similar to `ovr_RecenterTrackingOrigin` in that it modifies the
    /// (x,y,z) positional components and the yaw orientation component of the tracking space for
    /// the HMD and controllers.
    ///
    /// While `ovr_RecenterTrackingOrigin` resets the tracking origin in reference to the HMD's
    /// current pose, `ovr_SpecifyTrackingOrigin` allows the caller to explicitly specify a transform
    /// for the tracking origin. This transform is expected to be an offset to the most recent
    /// recentered origin, so calling this function repeatedly with the same originPose will keep
    /// nudging the recentered origin in that direction.
    ///
    /// There are several use cases for this function. For example, if the application decides to
    /// limit the yaw, or translation of the recentered pose instead of directly using the HMD pose
    /// the application can query the current tracking state via `ovr_GetTrackingState`, and apply
    /// some limitations to the HMD pose because feeding this pose back into this function.
    /// Similarly, this can be used to "adjust the seating position" incrementally in apps that
    /// feature seated experiences such as cockpit-based games.
    ///
    /// This function can emulate `ovr_RecenterTrackingOrigin` as such:
    ///
    /// ```no_run
    /// # use ovr_sys::*;
    /// # unsafe {
    /// # let session = ::std::mem::zeroed();
    /// let ts = ovr_GetTrackingState(session, 0.0, ovrFalse);
    /// ovr_SpecifyTrackingOrigin(session, ts.HeadPose.ThePose);
    /// # }
    /// ```
    ///
    /// The roll and pitch orientation components are determined by gravity and cannot be redefined.
    /// If you are using ovrTrackerPoses then you will need to call `ovr_GetTrackerPose` after
    /// this, because the sensor position(s) will change as a result of this.
    ///
    /// For more info, see the notes on each ovrTrackingOrigin enumeration to understand how
    /// recenter will vary slightly in its behavior based on the current ovrTrackingOrigin setting.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `originPose` Specifies a pose that will be used to transform the current tracking
    /// origin.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information. Return values include but aren't limited
    ///         to:
    ///
    /// * `ovrSuccess`: Completed successfully.
    /// * `ovrError_InvalidParameter`: The heading direction in `originPose` was invalid,
    ///         such as facing vertically. This can happen if the caller is directly feeding the pose
    ///         of a position-tracked device such as an HMD or controller into this function.
    ///
    /// see [`ovrTrackingOrigin`](type.ovrTrackingOrigin.html), [`ovr_GetTrackerPose`](fn.ovr_GetTrackerPose.html), [`ovr_RecenterTrackingOrigin`](fn.ovr_RecenterTrackingOrigin.html)
    ///
    pub fn ovr_SpecifyTrackingOrigin(session: ovrSession, originPose: ovrPosef) -> ovrResult;
    /// Clears the ShouldRecenter status bit in `ovrSessionStatus`.
    ///
    /// Clears the ShouldRecenter status bit in `ovrSessionStatus`, allowing further recenter
    /// requests to be detected. Since this is automatically done by  `ovr_RecenterTrackingOrigin` and `ovr_SpecifyTrackingOrigin`,
    /// this is only needs to be called when application is doing its own re-centering.
    pub fn ovr_ClearShouldRecenterFlag(session: ovrSession);
    /// Returns tracking state reading based on the specified absolute system time.
    ///
    /// Pass an absTime value of 0.0 to request the most recent sensor reading. In this case
    /// both PredictedPose and SamplePose will have the same value.
    ///
    /// This may also be used for more refined timing of front buffer rendering logic, and so on.
    ///
    /// This may be called by multiple threads.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// `absTime` Specifies the absolute future time to predict the return
    ///            `ovrTrackingState` value. Use 0 to request the most recent tracking state.
    ///
    /// `latencyMarker` Specifies that this call is the point in time where
    ///            the "App-to-Mid-Photon" latency timer starts from. If a given `ovrLayer`
    ///            provides "SensorSampleTime", that will override the value stored here.
    ///
    /// Returns the `ovrTrackingState` that is predicted for the given absTime.
    ///
    /// see [`ovrTrackingState`](struct.ovrTrackingState.html), [`ovr_GetEyePoses`](fn.ovr_GetEyePoses.html),  [`ovr_GetTimeInSeconds`](fn.ovr_GetTimeInSeconds.html)
    ///
    pub fn ovr_GetTrackingState(session: ovrSession, absTime: f64, latencyMarker: ovrBool) -> ovrTrackingState;
    /// Returns an array of poses, where each pose matches a device type provided by the `deviceTypes`
    /// array parameter.
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `deviceTypes` Array of device types to query for their poses.
    ///
    /// **in** `deviceCount` Number of queried poses. This number must match the length of the
    ///         `outDevicePoses` and `deviceTypes` array.
    ///
    /// **in** `absTime` Specifies the absolute future time to predict the return
    ///        `ovrTrackingState` value. Use 0 to request the most recent tracking state.
    ///
    ///  **out** `outDevicePoses` Array of poses, one for each device type in `deviceTypes` arrays.
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error and
    /// true upon success.
    ///
    pub fn ovr_GetDevicePoses(session: ovrSession, deviceTypes: *const ovrTrackedDeviceType, deviceCount: c_int, absTime: f64, outDevicePoses: *mut ovrPoseStatef) -> ovrResult;
    /// Returns the `ovrTrackerPose` for the given attached tracker.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// `trackerPoseIndex` Index of the tracker being requested.
    ///
    /// Returns the requested `ovrTrackerPose`. An empty `ovrTrackerPose` will be returned if trackerPoseIndex is out of range.
    ///
    /// see  [`ovr_GetTrackerCount`](fn.ovr_GetTrackerCount.html)
    ///
    pub fn ovr_GetTrackerPose(session: ovrSession, trackerPoseIndex: c_uint) -> ovrTrackerPose;
    /// Returns the most recent input state for controllers, without positional tracking info.
    ///
    /// **out** `inputState` Input state that will be filled in.
    ///
    /// `ovrControllerType` Specifies which controller the input will be returned for.
    ///
    /// Returns `ovrSuccess` if the new state was successfully obtained.
    ///
    /// see [`ovrControllerType`](type.ovrControllerType.html)
    ///
    pub fn ovr_GetInputState(session: ovrSession, controllerType: ovrControllerType, inputState: *mut ovrInputState) -> ovrResult;
    /// Returns controller types connected to the system OR'ed together.
    ///
    /// Returns a bitmask of `ovrControllerTypes` connected to the system.
    ///
    /// see [`ovrControllerType`](type.ovrControllerType.html)
    ///
    pub fn ovr_GetConnectedControllerTypes(session: ovrSession) -> c_uint;
    /// Gets information about Haptics engine for the specified Touch controller.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// `controllerType` The controller to retrieve the information from.
    ///
    /// Returns an `ovrTouchHapticsDesc`.
    ///
    pub fn ovr_GetTouchHapticsDesc(session: ovrSession, controllerType: ovrControllerType) -> ovrTouchHapticsDesc;
    /// Sets constant vibration (with specified frequency and amplitude) to a controller.
    ///
    /// Note: `ovr_SetControllerVibration` cannot be used interchangeably with  `ovr_SubmitControllerVibration`.
    ///
    /// This method should be called periodically, vibration lasts for a maximum of 2.5 seconds.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// `controllerType` The controller to set the vibration to.
    ///
    /// `frequency` Vibration frequency. Supported values are: 0.0 (disabled), 0.5 and 1.0. Non valid values will be clamped.
    /// `amplitude` Vibration amplitude in the `[0.0, 1.0]` range.
    ///
    /// Returns an `ovrResult` for which  `OVR_SUCCESS(result)` is false upon error and true
    ///         upon success. Return values include but aren't limited to:
    ///     * `ovrSuccess`: The call succeeded and a result was returned.
    ///     * `ovrSuccess_DeviceUnavailable`: The call succeeded but the device referred to by controllerType is not available.
    ///
    pub fn ovr_SetControllerVibration(session: ovrSession, controllerType: ovrControllerType, frequency: f32, amplitude: f32) -> ovrResult;
    /// Submits a Haptics buffer (used for vibration) to Touch (only) controllers.
    ///
    /// Note: `ovr_SubmitControllerVibration` cannot be used interchangeably with  `ovr_SetControllerVibration`.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// `controllerType` Controller where the Haptics buffer will be played.
    ///
    /// `buffer` Haptics buffer containing amplitude samples to be played.
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error and true
    ///         upon success. Return values include but aren't limited to:
    ///     * `ovrSuccess`: The call succeeded and a result was returned.
    ///     * `ovrSuccess_DeviceUnavailable`: The call succeeded but the device referred to by controllerType is not available.
    ///
    /// see [`ovrHapticsBuffer`](struct.ovrHapticsBuffer.html)
    ///
    pub fn ovr_SubmitControllerVibration(session: ovrSession, controllerType: ovrControllerType, buffer: *const ovrHapticsBuffer) -> ovrResult;
    /// Gets the Haptics engine playback state of a specific Touch controller.
    ///
    /// `session` Specifies an `ovrSession` previously returned by  `ovr_Create`.
    ///
    /// `controllerType` Controller where the Haptics buffer wil be played.
    ///
    /// `outState` State of the haptics engine.
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error and true
    ///         upon success. Return values include but aren't limited to:
    ///     * `ovrSuccess`: The call succeeded and a result was returned.
    ///     * `ovrSuccess_DeviceUnavailable`: The call succeeded but the device referred to by controllerType is not available.
    ///
    /// see [`ovrHapticsPlaybackState`](struct.ovrHapticsPlaybackState.html)
    ///
    pub fn ovr_GetControllerVibrationState(session: ovrSession, controllerType: ovrControllerType, outState: *mut ovrHapticsPlaybackState) -> ovrResult;
    /// Tests collision/proximity of position tracked devices (e.g. HMD and/or Touch) against the Boundary System.
    /// Note: this method is similar to `ovr_BoundaryTestPoint` but can be more precise as it may take into account device acceleration/momentum.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `deviceBitmask` Bitmask of one or more tracked devices to test.
    ///
    /// `boundaryType` Must be either `ovrBoundary_Outer` or `ovrBoundary_PlayArea`.
    ///
    /// **out** `outTestResult` Result of collision/proximity test, contains information such as distance and closest point.
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error and true
    ///         upon success. Return values include but aren't limited to:
    ///     * `ovrSuccess`: The call succeeded and a result was returned.
    ///     * `ovrSuccess_BoundaryInvalid`: The call succeeded but the result is not a valid boundary due to not being set up.
    ///     * `ovrSuccess_DeviceUnavailable`: The call succeeded but the device referred to by deviceBitmask is not available.
    ///
    /// see [`ovrBoundaryTestResult`](struct.ovrBoundaryTestResult.html)
    ///
    pub fn ovr_TestBoundary(session: ovrSession, deviceBitmask: ovrTrackedDeviceType, boundaryType: ovrBoundaryType, outTestResult: *mut ovrBoundaryTestResult) -> ovrResult;
    /// Tests collision/proximity of a 3D point against the Boundary System.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `point` 3D point to test.
    ///
    /// `singleBoundaryType` Must be either `ovrBoundary_Outer` or `ovrBoundary_PlayArea` to test against
    /// **out** `outTestResult` Result of collision/proximity test, contains information such as distance and closest point.
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error and true
    ///         upon success. Return values include but aren't limited to:
    ///     * `ovrSuccess`: The call succeeded and a result was returned.
    ///     * `ovrSuccess_BoundaryInvalid`: The call succeeded but the result is not a valid boundary due to not being set up.
    ///
    /// see [`ovrBoundaryTestResult`](struct.ovrBoundaryTestResult.html)
    ///
    pub fn ovr_TestBoundaryPoint(session: ovrSession, point: *const ovrVector3f, singleBoundaryType: ovrBoundaryType, outTestResult: *mut ovrBoundaryTestResult) -> ovrResult;
    /// Sets the look and feel of the Boundary System.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `lookAndFeel` Look and feel parameters.
    ///
    /// Returns `ovrSuccess` upon success.
    ///
    /// see [`ovrBoundaryLookAndFeel`](struct.ovrBoundaryLookAndFeel.html)
    ///
    pub fn ovr_SetBoundaryLookAndFeel(session: ovrSession, lookAndFeel: *const ovrBoundaryLookAndFeel) -> ovrResult;
    /// Resets the look and feel of the Boundary System to its default state.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// Returns `ovrSuccess` upon success.
    ///
    /// see [`ovrBoundaryLookAndFeel`](struct.ovrBoundaryLookAndFeel.html)
    ///
    pub fn ovr_ResetBoundaryLookAndFeel(session: ovrSession) -> ovrResult;
    /// Gets the geometry of the Boundary System's "play area" or "outer boundary" as 3D floor points.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `boundaryType` Must be either `ovrBoundary_Outer` or `ovrBoundary_PlayArea`.
    ///
    /// **out** `outFloorPoints` Array of 3D points (in clockwise order) defining the boundary at floor height (can be NULL to retrieve only the number of points).
    /// **out** `outFloorPointsCount` Number of 3D points returned in the array.
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error and true
    ///         upon success. Return values include but aren't limited to:
    ///     * `ovrSuccess`: The call succeeded and a result was returned.
    ///     * `ovrSuccess_BoundaryInvalid`: The call succeeded but the result is not a valid boundary due to not being set up.
    ///
    pub fn ovr_GetBoundaryGeometry(session: ovrSession, boundaryType: ovrBoundaryType, outFloorPoints: *mut ovrVector3f, outFloorPointsCount: *mut c_int) -> ovrResult;
    /// Gets the dimension of the Boundary System's "play area" or "outer boundary".
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `boundaryType` Must be either `ovrBoundary_Outer` or `ovrBoundary_PlayArea`.
    ///
    /// **out** `dimensions` Dimensions of the axis aligned bounding box that encloses the area in meters (width, height and length).
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error and true
    ///         upon success. Return values include but aren't limited to:
    ///     * `ovrSuccess`: The call succeeded and a result was returned.
    ///     * `ovrSuccess_BoundaryInvalid`: The call succeeded but the result is not a valid boundary due to not being set up.
    ///
    pub fn ovr_GetBoundaryDimensions(session: ovrSession, boundaryType: ovrBoundaryType, outDimensions: *mut ovrVector3f) -> ovrResult;
    /// Returns if the boundary is currently visible.
    /// Note: visibility is false if the user has turned off boundaries, otherwise, it's true if the app has requested
    /// boundaries to be visible or if any tracked device is currently triggering it. This may not exactly match rendering
    /// due to fade-in and fade-out effects.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **out** `outIsVisible` `ovrTrue`, if the boundary is visible.
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error and true
    ///         upon success. Return values include but aren't limited to:
    ///     * `ovrSuccess`: Result was successful and a result was returned.
    ///     * `ovrSuccess_BoundaryInvalid`: The call succeeded but the result is not a valid boundary due to not being set up.
    ///
    pub fn ovr_GetBoundaryVisible(session: ovrSession, outIsVisible: *mut ovrBool) -> ovrResult;
    /// Requests boundary to be visible.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    /// `visible` forces the outer boundary to be visible. An application can't force it to be invisible, but can cancel its request by passing false.

    /// Returns `ovrSuccess` upon success.
    ///
    pub fn ovr_RequestBoundaryVisible(session: ovrSession, visible: ovrBool) -> ovrResult;

}

//-------------------------------------------------------------------------------------
// @name Layers
//
//@{


///  Specifies the maximum number of layers supported by `ovr_SubmitFrame`.
///
///  /see `ovr_SubmitFrame`
///
/// Describes layer types that can be passed to `ovr_SubmitFrame`.
pub const ovrMaxLayerCount: u32 = 16;
/// Each layer type has an associated struct, such as `ovrLayerEyeFov`.
///
/// see [`ovrLayerHeader`](struct.ovrLayerHeader.html)
///
pub type ovrLayerType = i32;
/// Layer is disabled.
pub const ovrLayerType_Disabled: ovrLayerType    = 0;
/// Described by `ovrLayerEyeFov`.
pub const ovrLayerType_EyeFov: ovrLayerType      = 1;
/// Described by `ovrLayerQuad`. Previously called `ovrLayerType_QuadInWorld`.
pub const ovrLayerType_Quad: ovrLayerType        = 3;
// enum 4 used to be ovrLayerType_QuadHeadLocked. Instead, use ovrLayerType_Quad with ovrLayerFlag_HeadLocked.
/// Described by `ovrLayerEyeMatrix`.
pub const ovrLayerType_EyeMatrix: ovrLayerType   = 5;
/// Identifies flags used by `ovrLayerHeader` and which are passed to `ovr_SubmitFrame`.
///
/// see [`ovrLayerHeader`](struct.ovrLayerHeader.html)
///
pub type ovrLayerFlags = i32;
/// `ovrLayerFlag_HighQuality` enables 4x anisotropic sampling during the composition of the layer.
///
/// The benefits are mostly visible at the periphery for high-frequency & high-contrast visuals.
///
/// For best results consider combining this flag with an `ovrTextureSwapChain` that has mipmaps and
/// instead of using arbitrary sized textures, prefer texture sizes that are powers-of-two.
///
/// Actual rendered viewport and doesn't necessarily have to fill the whole texture.
pub const ovrLayerFlag_HighQuality: ovrLayerFlags               = 0x01;
/// `ovrLayerFlag_TextureOriginAtBottomLeft`: the opposite is TopLeft.
/// Generally this is false for D3D, true for OpenGL.
pub const ovrLayerFlag_TextureOriginAtBottomLeft: ovrLayerFlags = 0x02;
/// Mark this surface as "headlocked", which means it is specified
/// relative to the HMD and moves with it, rather than being specified
/// relative to sensor/torso space and remaining still while the head moves.
///
/// What used to be `ovrLayerType_QuadHeadLocked` is now `ovrLayerType_Quad` plus this flag.
///
/// However the flag can be applied to any layer type to achieve a similar effect.
pub const ovrLayerFlag_HeadLocked: ovrLayerFlags                = 0x04;
/// Defines properties shared by all `ovrLayer` structs, such as `ovrLayerEyeFov`.
///
/// `ovrLayerHeader` is used as a base member in these larger structs.
///
/// This struct cannot be used by itself except for the case that Type is `ovrLayerType_Disabled`.
///
/// see [`ovrLayerType`](type.ovrLayerType.html), [`ovrLayerFlags`](type.ovrLayerFlags.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrLayerHeader {
    pub _align: [isize; 0],
    /// Described by `ovrLayerType`.
    pub Type: ovrLayerType,
    /// Described by `ovrLayerFlags`.
    pub Flags: c_unsigned,
}


/// Describes a layer that specifies a monoscopic or stereoscopic view.
///
/// This is the kind of layer that's typically used as layer 0 to `ovr_SubmitFrame`,
/// as it is the kind of layer used to render a 3D stereoscopic view.
///
/// Three options exist with respect to mono/stereo texture usage:
///    * `ColorTexture[0]` and `ColorTexture[1]` contain the left and right stereo renderings, respectively.
///      `Viewport[0]` and `Viewport[1]` refer to `ColorTexture[0]` and `ColorTexture[1]`, respectively.
///    * `ColorTexture[0]` contains both the left and right renderings, `ColorTexture[1]` is NULL,
///      and `Viewport[0]` and `Viewport[1]` refer to sub-rects with `ColorTexture[0]`.
///    * `ColorTexture[0]` contains a single monoscopic rendering, and `Viewport[0]` and
///      `Viewport[1]` both refer to that rendering.
///
/// see [`ovrTextureSwapChain`](type.ovrTextureSwapChain.html), [`ovr_SubmitFrame`](fn.ovr_SubmitFrame.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrLayerEyeFov {
    pub _align: [isize; 0],
    /// Header.Type must be `ovrLayerType_EyeFov`.
    pub Header: ovrLayerHeader,

    /// `ovrTextureSwapChains` for the left and right eye respectively.
    ///
    /// The second one of which can be NULL for cases described above.
    pub ColorTexture: [ovrTextureSwapChain; ovrEye_Count as usize],

    /// Specifies the ColorTexture sub-rect UV coordinates.
    ///
    /// Both `Viewport[0]` and `Viewport[1]` must be valid.
    pub Viewport: [ovrRecti; ovrEye_Count as usize],

    /// The viewport field of view.
    pub Fov: [ovrFovPort; ovrEye_Count as usize],

    /// Specifies the position and orientation of each eye view, with the position specified in meters.
    ///
    /// RenderPose will typically be the value returned from `ovr_CalcEyePoses`,
    /// but can be different in special cases if a different head pose is used for rendering.
    pub RenderPose: [ovrPosef; ovrEye_Count as usize],

    /// Specifies the timestamp when the source `ovrPosef` (used in calculating RenderPose)
    /// was sampled from the SDK. Typically retrieved by calling `ovr_GetTimeInSeconds`
    /// around the instant the application calls `ovr_GetTrackingState`
    /// The main purpose for this is to accurately track app tracking latency.
    pub SensorSampleTime: f64,

}




/// Describes a layer that specifies a monoscopic or stereoscopic view.
///
/// This uses a direct 3x4 matrix to map from view space to the UV coordinates.
///
/// It is essentially the same thing as `ovrLayerEyeFov` but using a much
/// lower level. This is mainly to provide compatibility with specific apps.
///
/// Unless the application really requires this flexibility, it is usually better
/// to use `ovrLayerEyeFov`.
///
/// Three options exist with respect to mono/stereo texture usage:
///    * `ColorTexture[0]` and `ColorTexture[1]` contain the left and right stereo renderings, respectively.
///      `Viewport[0]` and `Viewport[1]` refer to `ColorTexture[0]` and `ColorTexture[1]`, respectively.
///    * `ColorTexture[0]` contains both the left and right renderings, `ColorTexture[1]` is NULL,
///      and `Viewport[0]` and `Viewport[1]` refer to sub-rects with `ColorTexture[0]`.
///    * `ColorTexture[0]` contains a single monoscopic rendering, and `Viewport[0]` and
///      `Viewport[1]` both refer to that rendering.
///
/// see [`ovrTextureSwapChain`](type.ovrTextureSwapChain.html), [`ovr_SubmitFrame`](fn.ovr_SubmitFrame.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrLayerEyeMatrix {
    pub _align: [isize; 0],
    /// Header.Type must be `ovrLayerType_EyeMatrix`.
    pub Header: ovrLayerHeader,

    /// `ovrTextureSwapChains` for the left and right eye respectively.
    ///
    /// The second one of which can be NULL for cases described above.
    pub ColorTexture: [ovrTextureSwapChain; ovrEye_Count as usize],

    /// Specifies the ColorTexture sub-rect UV coordinates.
    ///
    /// Both `Viewport[0]` and `Viewport[1]` must be valid.
    pub Viewport: [ovrRecti; ovrEye_Count as usize],

    /// Specifies the position and orientation of each eye view, with the position specified in meters.
    ///
    /// RenderPose will typically be the value returned from `ovr_CalcEyePoses`,
    /// but can be different in special cases if a different head pose is used for rendering.
    pub RenderPose: [ovrPosef; ovrEye_Count as usize],

    /// Specifies the mapping from a view-space vector
    /// to a UV coordinate on the textures given above.
    ///
    /// P = (x,y,z,1)*Matrix
    /// TexU  = P.x/P.z
    /// TexV  = P.y/P.z
    pub Matrix: [ovrMatrix4f; ovrEye_Count as usize],

    /// Specifies the timestamp when the source `ovrPosef` (used in calculating RenderPose)
    /// was sampled from the SDK. Typically retrieved by calling `ovr_GetTimeInSeconds`
    /// around the instant the application calls `ovr_GetTrackingState`
    /// The main purpose for this is to accurately track app tracking latency.
    pub SensorSampleTime: f64,

}





/// Describes a layer of Quad type, which is a single quad in world or viewer space.
///
/// It is used for `ovrLayerType_Quad`. This type of layer represents a single
/// object placed in the world and not a stereo view of the world itself.
///
/// A typical use of `ovrLayerType_Quad` is to draw a television screen in a room
/// that for some reason is more convenient to draw as a layer than as part of the main
/// view in layer 0. For example, it could implement a 3D popup GUI that is drawn at a
/// higher resolution than layer 0 to improve fidelity of the GUI.
///
/// Quad layers are visible from both sides; they are not back-face culled.
///
/// see [`ovrTextureSwapChain`](type.ovrTextureSwapChain.html), [`ovr_SubmitFrame`](fn.ovr_SubmitFrame.html)
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrLayerQuad {
    pub _align: [isize; 0],
    /// Header.Type must be `ovrLayerType_Quad`.
    pub Header: ovrLayerHeader,

    /// Contains a single image, never with any stereo view.
    pub ColorTexture: ovrTextureSwapChain,

    /// Specifies the ColorTexture sub-rect UV coordinates.
    pub Viewport: ovrRecti,

    /// Specifies the orientation and position of the center point of a Quad layer type.
    ///
    /// The supplied direction is the vector perpendicular to the quad.
    ///
    /// The position is in real-world meters (not the application's virtual world,
    /// the physical world the user is in) and is relative to the "zero" position
    /// set by `ovr_RecenterTrackingOrigin` unless the `ovrLayerFlag_HeadLocked` flag is used.
    pub QuadPoseCenter: ovrPosef,

    /// Width and height (respectively) of the quad in meters.
    pub QuadSize: ovrVector2f,

}




/// Union that combines `ovrLayer` types in a way that allows them
/// to be used in a polymorphic way.
/*typedef union ovrLayer_Union_
{
    pub Header: ovrLayerHeader,
    pub EyeFov: ovrLayerEyeFov,
    pub Quad: ovrLayerQuad,
}*/


//@}

// @name SDK Distortion Rendering
//
// All of rendering functions including the configure and frame functions
// are not thread safe. It is OK to use ConfigureRendering on one thread and handle
// frames on another thread, but explicit synchronization must be done since
// functions that depend on configured state are not reentrant.
//
// These functions support rendering of distortion by the SDK.
//
//@{

extern "C" {

    /// TextureSwapChain creation is rendering API-specific.
    ///
    /// `ovr_CreateTextureSwapChainDX`, `ovr_CreateTextureSwapChainGL` and `ovr_CreateTextureSwapChainVk` can be found in the
    /// rendering API-specific headers, such as OVR_CAPI_D3D.h and `OVR_CAPI_GL`.h

    /// Gets the number of buffers in an `ovrTextureSwapChain`.
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `chain` Specifies the `ovrTextureSwapChain` for which the length should be retrieved.
    ///
    /// **out** `out_Length` Returns the number of buffers in the specified chain.
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error.
    ///
    /// see [`ovr_CreateTextureSwapChainDX`](directx/fn.ovr_CreateTextureSwapChainDX.html), [`ovr_CreateTextureSwapChainGL`](opengl/fn.ovr_CreateTextureSwapChainGL.html), [`ovr_CreateTextureSwapChainVk`](opengl/fn.ovr_CreateTextureSwapChainVk.html)
    ///
    pub fn ovr_GetTextureSwapChainLength(session: ovrSession, chain: ovrTextureSwapChain, out_Length: *mut c_int) -> ovrResult;
    /// Gets the current index in an `ovrTextureSwapChain`.
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `chain` Specifies the `ovrTextureSwapChain` for which the index should be retrieved.
    ///
    /// **out** `out_Index` Returns the current (free) index in specified chain.
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error.
    ///
    /// see [`ovr_CreateTextureSwapChainDX`](directx/fn.ovr_CreateTextureSwapChainDX.html), [`ovr_CreateTextureSwapChainGL`](opengl/fn.ovr_CreateTextureSwapChainGL.html), [`ovr_CreateTextureSwapChainVk`](opengl/fn.ovr_CreateTextureSwapChainVk.html)
    ///
    pub fn ovr_GetTextureSwapChainCurrentIndex(session: ovrSession, chain: ovrTextureSwapChain, out_Index: *mut c_int) -> ovrResult;
    /// Gets the description of the buffers in an `ovrTextureSwapChain`
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `chain` Specifies the `ovrTextureSwapChain` for which the description should be retrieved.
    ///
    /// **out** `out_Desc` Returns the description of the specified chain.
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error.
    ///
    /// see [`ovr_CreateTextureSwapChainDX`](directx/fn.ovr_CreateTextureSwapChainDX.html), [`ovr_CreateTextureSwapChainGL`](opengl/fn.ovr_CreateTextureSwapChainGL.html), [`ovr_CreateTextureSwapChainVk`](opengl/fn.ovr_CreateTextureSwapChainVk.html)
    ///
    pub fn ovr_GetTextureSwapChainDesc(session: ovrSession, chain: ovrTextureSwapChain, out_Desc: *mut ovrTextureSwapChainDesc) -> ovrResult;
    /// Commits any pending changes to an `ovrTextureSwapChain`, and advances its current index
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `chain` Specifies the `ovrTextureSwapChain` to commit.
    ///
    /// **Note**: When Commit is called, the texture at the current index is considered ready for use by the
    /// runtime, and further writes to it should be avoided. The swap chain's current index is advanced,
    /// providing there's room in the chain. The next time the SDK dereferences this texture swap chain,
    /// it will synchronize with the app's graphics context and pick up the submitted index, opening up
    /// room in the swap chain for further commits.
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error.
    /// Failures include but aren't limited to:
    ///
    /// * `ovrError_TextureSwapChainFull`: `ovr_CommitTextureSwapChain` was called too many times on a texture swapchain without calling submit to use the chain.
    ///
    /// see [`ovr_CreateTextureSwapChainDX`](directx/fn.ovr_CreateTextureSwapChainDX.html), [`ovr_CreateTextureSwapChainGL`](opengl/fn.ovr_CreateTextureSwapChainGL.html), [`ovr_CreateTextureSwapChainVk`](opengl/fn.ovr_CreateTextureSwapChainVk.html)
    ///
    pub fn ovr_CommitTextureSwapChain(session: ovrSession, chain: ovrTextureSwapChain) -> ovrResult;
    /// Destroys an `ovrTextureSwapChain` and frees all the resources associated with it.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `chain` Specifies the `ovrTextureSwapChain` to destroy. If it is NULL then this function has no effect.
    ///
    /// see [`ovr_CreateTextureSwapChainDX`](directx/fn.ovr_CreateTextureSwapChainDX.html), [`ovr_CreateTextureSwapChainGL`](opengl/fn.ovr_CreateTextureSwapChainGL.html), [`ovr_CreateTextureSwapChainVk`](opengl/fn.ovr_CreateTextureSwapChainVk.html)
    ///
    pub fn ovr_DestroyTextureSwapChain(session: ovrSession, chain: ovrTextureSwapChain);
    /// MirrorTexture creation is rendering API-specific.
    ///
    /// `ovr_CreateMirrorTextureDX` and `ovr_CreateMirrorTextureGL` can be found in the
    /// rendering API-specific headers, such as OVR_CAPI_D3D.h and OVR_CAPI_GL.h

    /// Destroys a mirror texture previously created by one of the mirror texture creation functions.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `mirrorTexture` Specifies the `ovrTexture` to destroy. If it is NULL then this function has no effect.
    ///
    /// see [`ovr_CreateMirrorTextureDX`](directx/fn.ovr_CreateMirrorTextureDX.html), [`ovr_CreateMirrorTextureGL`](opengl/fn.ovr_CreateMirrorTextureGL.html), [`ovr_CreateMirrorTextureVk`](opengl/fn.ovr_CreateMirrorTextureVk.html)
    ///
    pub fn ovr_DestroyMirrorTexture(session: ovrSession, mirrorTexture: ovrMirrorTexture);
    /// Calculates the recommended viewport size for rendering a given eye within the HMD
    /// with a given FOV cone.
    ///
    /// Higher FOV will generally require larger textures to maintain quality.
    ///
    /// Apps packing multiple eye views together on the same texture should ensure there are
    /// at least 8 pixels of padding between them to prevent texture filtering and chromatic
    /// aberration causing images to leak between the two eye views.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `eye` Specifies which eye (left or right) to calculate for.
    ///
    /// `fov` Specifies the `ovrFovPort` to use.
    ///
    /// `pixelsPerDisplayPixel` Specifies the ratio of the number of render target pixels
    /// to display pixels at the center of distortion. 1.0 is the default value. Lower
    /// values can improve performance, higher values give improved quality.
    ///
    /// *Example code*
    ///
    /// ```no_run
    /// # use ovr_sys::*;
    /// # unsafe {
    /// # let session = ::std::mem::zeroed();
    /// let hmd_desc = ovr_GetHmdDesc(session);
    /// let eye_size_left = ovr_GetFovTextureSize(session, ovrEye_Left,  hmd_desc.DefaultEyeFov[ovrEye_Left as usize],  1.0);
    /// let eye_size_right = ovr_GetFovTextureSize(session, ovrEye_Right, hmd_desc.DefaultEyeFov[ovrEye_Right as usize], 1.0);
    /// # drop((eye_size_left, eye_size_right));
    /// # }
    /// ```
    ///
    /// Returns the texture width and height size.
    ///
    pub fn ovr_GetFovTextureSize(session: ovrSession, eye: ovrEyeType, fov: ovrFovPort, pixelsPerDisplayPixel: f32) -> ovrSizei;
    /// Computes the distortion viewport, view adjust, and other rendering parameters for
    /// the specified eye.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `eyeType` Specifies which eye (left or right) for which to perform calculations.
    ///
    /// `fov` Specifies the `ovrFovPort` to use.
    ///
    /// Returns the computed `ovrEyeRenderDesc` for the given eyeType and field of view.
    ///
    /// see [`ovrEyeRenderDesc`](struct.ovrEyeRenderDesc.html)
    ///
    pub fn ovr_GetRenderDesc(session: ovrSession, eyeType: ovrEyeType, fov: ovrFovPort) -> ovrEyeRenderDesc;
    /// Submits layers for distortion and display.
    ///
    /// `ovr_SubmitFrame` triggers distortion and processing which might happen asynchronously.
    ///
    /// The function will return when there is room in the submission queue and surfaces
    /// are available. Distortion might or might not have completed.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `frameIndex` Specifies the targeted application frame index, or 0 to refer to one frame
    ///        after the last time `ovr_SubmitFrame` was called.
    ///
    /// `viewScaleDesc` Provides additional information needed only if layerPtrList contains
    ///        an `ovrLayerType_Quad`. If NULL, a default version is used based on the current configuration and a 1.0 world scale.
    ///
    /// `layerPtrList` Specifies a list of `ovrLayer` pointers, which can include NULL entries to
    ///        indicate that any previously shown layer at that index is to not be displayed.
    ///
    /// Each layer header must be a part of a layer structure such as `ovrLayerEyeFov` or `ovrLayerQuad`,
    /// with Header.Type identifying its type. A NULL layerPtrList entry in the array indicates the
    /// absence of the given layer.
    ///
    /// `layerCount` Indicates the number of valid elements in layerPtrList. The maximum
    ///        supported layerCount is not currently specified, but may be specified in a future version.
    ///
    /// - Layers are drawn in the order they are specified in the array, regardless of the layer type.
    ///
    /// - Layers are not remembered between successive calls to `ovr_SubmitFrame`. A layer must be
    ///   specified in every call to `ovr_SubmitFrame` or it won't be displayed.
    ///
    /// - If a layerPtrList entry that was specified in a previous call to `ovr_SubmitFrame` is
    ///   passed as NULL or is of type `ovrLayerType_Disabled`, that layer is no longer displayed.
    ///
    /// - A layerPtrList entry can be of any layer type and multiple entries of the same layer type
    ///   are allowed. No layerPtrList entry may be duplicated (i.e. the same pointer as an earlier entry).
    ///
    /// *Example code*
    ///
    /// ```no_run
    /// # use ovr_sys::*;
    /// # use ::std::ptr;
    /// # unsafe {
    /// # fn foo() -> ovrLayerEyeFov { panic!() }
    /// # fn bar() -> ovrLayerQuad { panic!() }
    /// # let (session, frame_index) = ::std::mem::zeroed();
    /// // In initialisation
    /// let layer0: ovrLayerEyeFov = foo();
    /// let layer1: ovrLayerQuad = bar();
    /// // In frame loop
    /// let layers = [&layer0.Header as *const _, &layer1.Header as *const _];
    /// let result = ovr_SubmitFrame(session, frame_index, ptr::null(), layers.as_ptr(), 2);
    /// # drop(result);
    /// # }
    /// ```
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error and true
    /// upon success. Return values include but aren't limited to:
    /// 
    /// * `ovrSuccess`: rendering completed successfully.
    /// * `ovrSuccess_NotVisible`: rendering completed successfully but was not displayed on the HMD,
    ///   usually because another application currently has ownership of the HMD. Applications receiving
    ///   this result should stop rendering new content, but continue to call `ovr_SubmitFrame` periodically
    ///   until it returns a value other than `ovrSuccess_NotVisible`.
    /// * `ovrError_DisplayLost`: The session has become invalid (such as due to a device removal)
    ///   and the shared resources need to be released (`ovr_DestroyTextureSwapChain`), the session needs to
    ///   destroyed (`ovr_Destroy`) and recreated (`ovr_Create`), and new resources need to be created
    ///   (`ovr_CreateTextureSwapChainXXX`). The application's existing private graphics resources do not
    ///   need to be recreated unless the new `ovr_Create` call returns a different GraphicsLuid.
    /// * `ovrError_TextureSwapChainInvalid`: The `ovrTextureSwapChain` is in an incomplete or inconsistent state.
    ///   Ensure `ovr_CommitTextureSwapChain` was called at least once first.
    ///
    /// see [`ovr_GetPredictedDisplayTime`](fn.ovr_GetPredictedDisplayTime.html), [`ovrViewScaleDesc`](struct.ovrViewScaleDesc.html), [`ovrLayerHeader`](struct.ovrLayerHeader.html)
    ///
    pub fn ovr_SubmitFrame(session: ovrSession, frameIndex: c_longlong, viewScaleDesc: *const ovrViewScaleDesc, layerPtrList: *const *const ovrLayerHeader, layerCount: c_uint) -> ovrResult;

}

//-------------------------------------------------------------------------------------
// @name Frame Timing
//
//@{

///
/// Contains the performance stats for a given SDK compositor frame
///
/// All of the c_int fields can be reset via the `ovr_ResetPerfStats` call.
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrPerfStatsPerCompositorFrame {
    pub _align: [u32; 0],
    ///
    /// Vsync Frame Index - increments with each HMD vertical synchronization signal (i.e. vsync or refresh rate)
    /// If the compositor drops a frame, expect this value to increment more than 1 at a time.
    ///
    pub HmdVsyncIndex: c_int,

    ///
    /// Application stats
    ///

    /// Index that increments with each successive `ovr_SubmitFrame` call
    pub AppFrameIndex: c_int,

    /// If the app fails to call `ovr_SubmitFrame` on time, then expect this value to increment with each missed frame
    pub AppDroppedFrameCount: c_int,

    /// Motion-to-photon latency for the application
    /// This value is calculated by either using the SensorSampleTime provided for the `ovrLayerEyeFov` or if that
    /// is not available, then the call to `ovr_GetTrackingState` which has latencyMarker set to `ovrTrue`
    pub AppMotionToPhotonLatency: f32,

    /// Amount of queue-ahead in seconds provided to the app based on performance and overlap of CPU & GPU utilization
    /// A value of 0.0 would mean the CPU & GPU workload is being completed in 1 frame's worth of time, while
    /// 11 ms (on the CV1) of queue ahead would indicate that the app's CPU workload for the next frame is
    /// overlapping the app's GPU workload for the current frame.
    pub AppQueueAheadTime: f32,

    /// Amount of time in seconds spent on the CPU by the app's render-thread that calls `ovr_SubmitFrame`
    /// Measured as elapsed time between from when app regains control from `ovr_SubmitFrame` to the next time the app
    /// calls `ovr_SubmitFrame`.
    pub AppCpuElapsedTime: f32,

    /// Amount of time in seconds spent on the GPU by the app
    /// Measured as elapsed time between each `ovr_SubmitFrame` call using GPU timing queries.
    pub AppGpuElapsedTime: f32,

    ///
    /// SDK Compositor stats
    ///

    /// Index that increments each time the SDK compositor completes a distortion and timewarp pass
    /// Since the compositor operates asynchronously, even if the app calls `ovr_SubmitFrame` too late,
    /// the compositor will kick off for each vsync.
    pub CompositorFrameIndex: c_int,

    /// Increments each time the SDK compositor fails to complete in time
    /// This is not tied to the app's performance, but failure to complete can be tied to other factors
    /// such as OS capabilities, overall available hardware cycles to execute the compositor in time
    /// and other factors outside of the app's control.
    pub CompositorDroppedFrameCount: c_int,

    /// Motion-to-photon latency of the SDK compositor in seconds
    /// This is the latency of timewarp which corrects the higher app latency as well as dropped app frames.
    pub CompositorLatency: f32,

    /// The amount of time in seconds spent on the CPU by the SDK compositor. Unless the VR app is utilizing
    /// all of the CPU cores at their peak performance, there is a good chance the compositor CPU times
    /// will not affect the app's CPU performance in a major way.
    pub CompositorCpuElapsedTime: f32,

    /// The amount of time in seconds spent on the GPU by the SDK compositor. Any time spent on the compositor
    /// will eat away from the available GPU time for the app.
    pub CompositorGpuElapsedTime: f32,

    /// The amount of time in seconds spent from the point the CPU kicks off the compositor to the point in time
    /// the compositor completes the distortion & timewarp on the GPU. In the event the GPU time is not
    /// available, expect this value to be -1.0f
    pub CompositorCpuStartToGpuEndElapsedTime: f32,

    /// The amount of time in seconds left after the compositor is done on the GPU to the associated V-Sync time.
    ///
    /// In the event the GPU time is not available, expect this value to be -1.0f
    pub CompositorGpuEndToVsyncElapsedTime: f32,

    ///
    /// Async Spacewarp stats (ASW)
    ///

    /// Will be true if ASW is active for the given frame such that the application is being forced
    /// into half the frame-rate while the compositor continues to run at full frame-rate.
    pub AswIsActive: ovrBool,

    /// Increments each time ASW it activated where the app was forced in and out of
    /// half-rate rendering.
    pub AswActivatedToggleCount: c_int,

    /// Accumulates the number of frames presented by the compositor which had extrapolated
    /// ASW frames presented.
    pub AswPresentedFrameCount: c_int,

    /// Accumulates the number of frames that the compositor tried to present when ASW is
    /// active but failed.
    pub AswFailedFrameCount: c_int,
}

///
/// Maximum number of frames of performance stats provided back to the caller of `ovr_GetPerfStats`
///
///
pub const ovrMaxProvidedFrameStats: u32 = 5;
/// This is a complete descriptor of the performance stats provided by the SDK
///
/// FrameStatsCount will have a maximum value set by `ovrMaxProvidedFrameStats`
/// If the application calls `ovr_GetPerfStats` at the native refresh rate of the HMD
/// then FrameStatsCount will be 1. If the app's workload happens to force
/// `ovr_GetPerfStats` to be called at a lower rate, then FrameStatsCount will be 2 or more.
///
/// If the app does not want to miss any performance data for any frame, it needs to
/// ensure that it is calling `ovr_SubmitFrame` and `ovr_GetPerfStats` at a rate that is at least:
/// "HMD_refresh_rate / `ovrMaxProvidedFrameStats`". On the Oculus Rift CV1 HMD, this will
/// be equal to 18 times per second.
///
/// If the app calls `ovr_SubmitFrame` at a rate less than 18 fps, then when calling
/// `ovr_GetPerfStats`, expect AnyFrameStatsDropped to become `ovrTrue` while FrameStatsCount
/// is equal to `ovrMaxProvidedFrameStats`.
///
/// The performance entries will be ordered in reverse chronological order such that the
/// first entry will be the most recent one.
///
/// AdaptiveGpuPerformanceScale is an edge-filtered value that a caller can use to adjust
/// the graphics quality of the application to keep the GPU utilization in check. The value
/// is calculated as: (desired_GPU_utilization / current_GPU_utilization)
/// As such, when this value is 1.0, the GPU is doing the right amount of work for the app.
///
/// Lower values mean the app needs to pull back on the GPU utilization.
///
/// If the app is going to directly drive render-target resolution using this value, then
/// be sure to take the square-root of the value before scaling the resolution with it.
///
/// Changing render target resolutions however is one of the many things an app can do
/// increase or decrease the amount of GPU utilization.
///
/// Since AdaptiveGpuPerformanceScale is edge-filtered and does not change rapidly
/// (i.e. reports non-1.0 values once every couple of seconds) the app can make the
/// necessary adjustments and then keep watching the value to see if it has been satisfied.
///
/// see [`ovr_GetPerfStats`](fn.ovr_GetPerfStats.html), [`ovrPerfStatsPerCompositorFrame`](struct.ovrPerfStatsPerCompositorFrame.html)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ovrPerfStats {
    pub _align: [u32; 0],
    /// `FrameStatsCount` will have a maximum value set by `ovrMaxProvidedFrameStats`
    /// If the application calls `ovr_GetPerfStats` at the native refresh rate of the HMD
    /// then `FrameStatsCount` will be 1. If the app's workload happens to force
    /// `ovr_GetPerfStats` to be called at a lower rate, then `FrameStatsCount` will be 2 or more.
    /// If the app does not want to miss any performance data for any frame, it needs to
    /// ensure that it is calling `ovr_SubmitFrame` and `ovr_GetPerfStats` at a rate that is at least:
    /// "HMD_refresh_rate / ovrMaxProvidedFrameStats". On the Oculus Rift CV1 HMD, this will
    /// be equal to 18 times per second.
    ///
    /// The performance entries will be ordered in reverse chronological order such that the
    /// first entry will be the most recent one.
    pub FrameStats: [ovrPerfStatsPerCompositorFrame; ovrMaxProvidedFrameStats as usize],
    pub FrameStatsCount: c_int,
    /// If the app calls `ovr_GetPerfStats` at less than 18 fps for CV1, then `AnyFrameStatsDropped`
    /// will be `ovrTrue` and `FrameStatsCount` will be equal to `ovrMaxProvidedFrameStats`.
    pub AnyFrameStatsDropped: ovrBool,
    /// `AdaptiveGpuPerformanceScale` is an edge-filtered value that a caller can use to adjust
    /// the graphics quality of the application to keep the GPU utilization in check. The value
    /// is calculated as: (desired_GPU_utilization / current_GPU_utilization)
    /// As such, when this value is 1.0, the GPU is doing the right amount of work for the app.
    /// Lower values mean the app needs to pull back on the GPU utilization.
    /// If the app is going to directly drive render-target resolution using this value, then
    /// be sure to take the square-root of the value before scaling the resolution with it.
    /// Changing render target resolutions however is one of the many things an app can do
    /// increase or decrease the amount of GPU utilization.
    /// Since `AdaptiveGpuPerformanceScale` is edge-filtered and does not change rapidly
    /// (i.e. reports non-1.0 values once every couple of seconds) the app can make the
    /// necessary adjustments and then keep watching the value to see if it has been satisfied.
    pub AdaptiveGpuPerformanceScale: f32,

    /// Will be true if Async Spacewarp (ASW) is available for this system which is dependent on
    /// several factors such as choice of GPU, OS and debug overrides
    pub AswIsAvailable: ovrBool,

    /// Contains the Process ID of the VR application the stats are being polled for
    /// If an app continues to grab perf stats even when it is not visible, then expect this
    /// value to point to the other VR app that has grabbed focus (i.e. became visible)
    pub VisibleProcessId: ovrProcessId,
}

extern "C" {

    /// Retrieves performance stats for the VR app as well as the SDK compositor.
    ///
    /// This function will return stats for the VR app that is currently visible in the HMD
    /// regardless of what VR app is actually calling this function.
    ///
    /// If the VR app is trying to make sure the stats returned belong to the same application,
    /// the caller can compare the `VisibleProcessId` with their own process ID. Normally this will
    /// be the case if the caller is only calling `ovr_GetPerfStats` when `ovr_GetSessionStatus` has
    /// IsVisible flag set to be true.
    ///
    /// If the VR app calling `ovr_GetPerfStats` is actually the one visible in the HMD,
    /// then new perf stats will only be populated after a new call to `ovr_SubmitFrame`.
    /// That means subsequent calls to `ovr_GetPerfStats` after the first one without calling
    /// `ovr_SubmitFrame` will receive a `FrameStatsCount` of zero.
    ///
    /// If the VR app is not visible, or was initially marked as `ovrInit_Invisible`, then each call
    /// to `ovr_GetPerfStats` will immediately fetch new perf stats from the compositor without
    /// a need for the `ovr_SubmitFrame` call.
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **out** `outStats` Contains the performance stats for the application and SDK compositor
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error and true
    ///         upon success.
    ///
    /// see [`ovrPerfStats`](struct.ovrPerfStats.html), [`ovrPerfStatsPerCompositorFrame`](struct.ovrPerfStatsPerCompositorFrame.html), [`ovr_ResetPerfStats`](fn.ovr_ResetPerfStats.html)
    ///
    pub fn ovr_GetPerfStats(session: ovrSession, outStats: *mut ovrPerfStats) -> ovrResult;
    /// Resets the accumulated stats reported in each `ovrPerfStatsPerCompositorFrame` back to zero.
    ///
    /// Only the integer values such as HmdVsyncIndex, AppDroppedFrameCount etc. will be reset
    /// as the other fields such as AppMotionToPhotonLatency are independent timing values updated
    /// per-frame.
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// Returns an `ovrResult` for which `OVR_SUCCESS(result)` is false upon error and true
    ///         upon success.
    ///
    /// see [`ovrPerfStats`](struct.ovrPerfStats.html), [`ovrPerfStatsPerCompositorFrame`](struct.ovrPerfStatsPerCompositorFrame.html), [`ovr_GetPerfStats`](fn.ovr_GetPerfStats.html)
    ///
    pub fn ovr_ResetPerfStats(session: ovrSession) -> ovrResult;
    /// Gets the time of the specified frame midpoint.
    ///
    /// Predicts the time at which the given frame will be displayed. The predicted time
    /// is the middle of the time period during which the corresponding eye images will
    /// be displayed.
    ///
    /// The application should increment frameIndex for each successively targeted frame,
    /// and pass that index to any relevant OVR functions that need to apply to the frame
    /// identified by that index.
    ///
    /// This function is thread-safe and allows for multiple application threads to target
    /// their processing to the same displayed frame.
    ///
    /// In the even that prediction fails due to various reasons (e.g. the display being off
    /// or app has yet to present any frames), the return value will be current CPU time.
    ///
    /// **in** `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `frameIndex` Identifies the frame the caller wishes to target.
    ///            A value of zero returns the next frame index.
    ///
    /// Returns the absolute frame midpoint time for the given frameIndex.
    ///
    /// see  [`ovr_GetTimeInSeconds`](fn.ovr_GetTimeInSeconds.html)
    ///
    pub fn ovr_GetPredictedDisplayTime(session: ovrSession, frameIndex: c_longlong) -> f64;
    /// Returns global, absolute high-resolution time in seconds.
    ///
    /// The time frame of reference for this function is not specified and should not be
    /// depended upon.
    ///
    /// Returns seconds as a floating point value.
    ///
    /// see [`ovrPoseStatef`](struct.ovrPoseStatef.html), `ovrFrameTiming`
    ///
    pub fn ovr_GetTimeInSeconds() -> f64;

}

/// Performance HUD enables the HMD user to see information critical to
/// the real-time operation of the VR application such as latency timing,
/// and CPU & GPU performance metrics
///
/// App can toggle performance HUD modes as such:
///
/// ```no_run
/// # use ovr_sys::*;
/// # use ::std::ffi::CStr;
/// # unsafe {
/// # let session = ::std::mem::zeroed();
/// let perf_hud_mode = ovrPerfHud_LatencyTiming;
/// ovr_SetInt(session, CStr::from_bytes_with_nul_unchecked(OVR_PERF_HUD_MODE).as_ptr(), perf_hud_mode);
/// # }
/// ```
///
pub type ovrPerfHudMode = i32;
/// Turns off the performance HUD
pub const ovrPerfHud_Off: ovrPerfHudMode                = 0;
/// Shows performance summary and headroom
pub const ovrPerfHud_PerfSummary: ovrPerfHudMode        = 1;
/// Shows latency related timing info
pub const ovrPerfHud_LatencyTiming: ovrPerfHudMode      = 2;
/// Shows render timing info for application
pub const ovrPerfHud_AppRenderTiming: ovrPerfHudMode    = 3;
/// Shows render timing info for OVR compositor
pub const ovrPerfHud_CompRenderTiming: ovrPerfHudMode   = 4;
/// Shows SDK & HMD version Info
pub const ovrPerfHud_VersionInfo: ovrPerfHudMode        = 5;
/// \internal Count of enumerated elements.
#[doc(hidden)]
pub const ovrPerfHud_Count: ovrPerfHudMode              = 6;
/// Layer HUD enables the HMD user to see information about a layer
///
/// App can toggle layer HUD modes as such:
///
/// ```no_run
/// # use ovr_sys::*;
/// # use ::std::ffi::CStr;
/// # unsafe {
/// # let session = ::std::mem::zeroed();
/// let layer_hud_mode = ovrLayerHud_Info;
/// ovr_SetInt(session, CStr::from_bytes_with_nul_unchecked(OVR_LAYER_HUD_MODE).as_ptr(), layer_hud_mode);
/// # }
/// ```
///
pub type ovrLayerHudMode = i32;
/// Turns off the layer HUD
pub const ovrLayerHud_Off: ovrLayerHudMode = 0;
/// Shows info about a specific layer
pub const ovrLayerHud_Info: ovrLayerHudMode = 1;
//@}

/// Debug HUD is provided to help developers gauge and debug the fidelity of their app's
/// stereo rendering characteristics. Using the provided quad and crosshair guides,
/// the developer can verify various aspects such as VR tracking units (e.g. meters),
/// stereo camera-parallax properties (e.g. making sure objects at infinity are rendered
/// with the proper separation), measuring VR geometry sizes and distances and more.
///
/// App can toggle the debug HUD modes as such:
///
/// ```no_run
/// # use ovr_sys::*;
/// # use ::std::ffi::CStr;
/// # unsafe {
/// # let session = ::std::mem::zeroed();
/// let debug_hud_mode = ovrDebugHudStereo_QuadWithCrosshair;
/// ovr_SetInt(session, CStr::from_bytes_with_nul_unchecked(OVR_DEBUG_HUD_STEREO_MODE).as_ptr(), debug_hud_mode);
/// # }
/// ```
///
/// The app can modify the visual properties of the stereo guide (i.e. quad, crosshair)
/// using the `ovr_SetFloatArray` function. For a list of tweakable properties,
/// see the `OVR_DEBUG_HUD_STEREO_GUIDE_*` keys in the OVR_CAPI_Keys.h header file.
pub type ovrDebugHudStereoMode = i32;
/// Turns off the Stereo Debug HUD
pub const ovrDebugHudStereo_Off: ovrDebugHudStereoMode                 = 0;
/// Renders Quad in world for Stereo Debugging
pub const ovrDebugHudStereo_Quad: ovrDebugHudStereoMode                = 1;
/// Renders Quad+crosshair in world for Stereo Debugging
pub const ovrDebugHudStereo_QuadWithCrosshair: ovrDebugHudStereoMode   = 2;
/// Renders screen-space crosshair at infinity for Stereo Debugging
pub const ovrDebugHudStereo_CrosshairAtInfinity: ovrDebugHudStereoMode = 3;
/// \internal Count of enumerated elements
#[doc(hidden)]
pub const ovrDebugHudStereo_Count: ovrDebugHudStereoMode = 4;

// -----------------------------------------------------------------------------------
// @name Property Access
//
// These functions read and write OVR properties. Supported properties
// are defined in `OVR_CAPI_Keys`.h
//
extern "C" {

    /// Reads a boolean property.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `propertyName` The name of the property, which needs to be valid for only the call.
    ///
    /// `defaultVal` specifes the value to return if the property couldn't be read.
    ///
    /// Returns the property interpreted as a boolean value. Returns defaultVal if
    ///         the property doesn't exist.
    pub fn ovr_GetBool(session: ovrSession, propertyName: *const c_char, defaultVal: ovrBool) -> ovrBool;
    /// Writes or creates a boolean property.
    ///
    /// If the property wasn't previously a boolean property, it is changed to a boolean property.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `propertyName` The name of the property, which needs to be valid only for the call.
    ///
    /// `value` The value to write.
    ///
    /// Returns true if successful, otherwise false. A false result should only occur if the property
    ///         name is empty or if the property is read-only.
    pub fn ovr_SetBool(session: ovrSession, propertyName: *const c_char, value: ovrBool) -> ovrBool;
    /// Reads an integer property.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `propertyName` The name of the property, which needs to be valid only for the call.
    ///
    /// `defaultVal` Specifes the value to return if the property couldn't be read.
    ///
    /// Returns the property interpreted as an integer value. Returns defaultVal if
    ///         the property doesn't exist.
    pub fn ovr_GetInt(session: ovrSession, propertyName: *const c_char, defaultVal: c_int) -> c_int;
    /// Writes or creates an integer property.
    ///
    /// If the property wasn't previously a boolean property, it is changed to an integer property.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `propertyName` The name of the property, which needs to be valid only for the call.
    ///
    /// `value` The value to write.
    ///
    /// Returns true if successful, otherwise false. A false result should only occur if the property
    ///         name is empty or if the property is read-only.
    pub fn ovr_SetInt(session: ovrSession, propertyName: *const c_char, value: c_int) -> ovrBool;
    /// Reads a `f32` property.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `propertyName` The name of the property, which needs to be valid only for the call.
    ///
    /// `defaultVal` specifes the value to return if the property couldn't be read.
    ///
    /// Returns the property interpreted as an `f32` value. Returns defaultVal if
    ///         the property doesn't exist.
    pub fn ovr_GetFloat(session: ovrSession, propertyName: *const c_char, defaultVal: f32) -> f32;
    /// Writes or creates a `f32` property.
    ///
    /// If the property wasn't previously a `f32` property, it's changed to a `f32` property.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `propertyName` The name of the property, which needs to be valid only for the call.
    ///
    /// `value` The value to write.
    ///
    /// Returns true if successful, otherwise false. A false result should only occur if the property
    ///         name is empty or if the property is read-only.
    pub fn ovr_SetFloat(session: ovrSession, propertyName: *const c_char, value: f32) -> ovrBool;
    /// Reads a `f32` array property.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `propertyName` The name of the property, which needs to be valid only for the call.
    ///
    /// `values` An array of `f32` to write to.
    ///
    /// `valuesCapacity` Specifies the maximum number of elements to write to the values array.
    ///
    /// Returns the number of elements read, or 0 if property doesn't exist or is empty.
    pub fn ovr_GetFloatArray(session: ovrSession, propertyName: *const c_char, values: *mut f32, valuesCapacity: c_uint) -> c_uint;
    /// Writes or creates a `f32` array property.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `propertyName` The name of the property, which needs to be valid only for the call.
    ///
    /// `values` An array of `f32` to write from.
    ///
    /// `valuesSize` Specifies the number of elements to write.
    ///
    /// Returns true if successful, otherwise false. A false result should only occur if the property
    ///         name is empty or if the property is read-only.
    pub fn ovr_SetFloatArray(session: ovrSession, propertyName: *const c_char, values: *const f32, valuesSize: c_uint) -> ovrBool;
    /// Reads a string property.
    ///
    /// Strings are UTF8-encoded and null-terminated.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `propertyName` The name of the property, which needs to be valid only for the call.
    ///
    /// `defaultVal` Specifes the value to return if the property couldn't be read.
    ///
    /// Returns the string property if it exists. Otherwise returns defaultVal, which can be specified as NULL.
    ///
    /// The return memory is guaranteed to be valid until next call to `ovr_GetString` or
    /// until the session is destroyed, whichever occurs first.
    pub fn ovr_GetString(session: ovrSession, propertyName: *const c_char, defaultVal: *const c_char) -> *const c_char;
    /// Writes or creates a string property.
    ///
    /// Strings are UTF8-encoded and null-terminated.
    ///
    /// `session` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// `propertyName` The name of the property, which needs to be valid only for the call.
    ///
    /// `value` The string property, which only needs to be valid for the duration of the call.
    /// 
    /// Returns true if successful, otherwise false. A false result should only occur if the property
    ///         name is empty or if the property is read-only.
    pub fn ovr_SetString(session: ovrSession, propertyName: *const c_char, value: *const c_char) -> ovrBool;

} // extern "C"



#[cfg(test)]
#[test]
pub fn test() {
    use ::std::mem::size_of;
    //-----------------------------------------------------------------------------
    // ***** Compiler packing validation
    //
    // These checks ensure that the compiler settings being used will be compatible
    // with with pre-built dynamic library provided with the runtime.

    assert!(size_of::<ovrBool>() == 1,         "ovrBool size mismatch");
    assert!(size_of::<ovrVector2i>() == 4 * 2, "ovrVector2i size mismatch");
    assert!(size_of::<ovrSizei>() == 4 * 2,    "ovrSizei size mismatch");
    assert!(size_of::<ovrRecti>() == size_of::<ovrVector2i>() + size_of::<ovrSizei>(), "ovrRecti size mismatch");
    assert!(size_of::<ovrQuatf>() == 4 * 4,    "ovrQuatf size mismatch");
    assert!(size_of::<ovrVector2f>() == 4 * 2, "ovrVector2f size mismatch");
    assert!(size_of::<ovrVector3f>() == 4 * 3, "ovrVector3f size mismatch");
    assert!(size_of::<ovrMatrix4f>() == 4 * 16, "ovrMatrix4f size mismatch");

    assert!(size_of::<ovrPosef>() == (7 * 4),       "ovrPosef size mismatch");
    assert!(size_of::<ovrPoseStatef>() == (22 * 4), "ovrPoseStatef size mismatch");
    assert!(size_of::<ovrFovPort>() == (4 * 4),     "ovrFovPort size mismatch");

    assert!(size_of::<ovrHmdCaps>() == 4,      "ovrHmdCaps size mismatch");
    assert!(size_of::<ovrTrackingCaps>() == 4, "ovrTrackingCaps size mismatch");
    assert!(size_of::<ovrEyeType>() == 4,      "ovrEyeType size mismatch");
    assert!(size_of::<ovrHmdType>() == 4,      "ovrHmdType size mismatch");

    assert!(size_of::<ovrTrackerDesc>() == 4 + 4 + 4 + 4, "ovrTrackerDesc size mismatch");
    assert!(size_of::<ovrTrackerPose>() == 4 + 4 + size_of::<ovrPosef>() + size_of::<ovrPosef>(), "ovrTrackerPose size mismatch");
    assert!(size_of::<ovrTrackingState>() == size_of::<ovrPoseStatef>() + 4 + 4 + (size_of::<ovrPoseStatef>() * 2) + (size_of::<c_uint>() * 2) + size_of::<ovrPosef>() + 4, "ovrTrackingState size mismatch");


    //assert!(size_of::<ovrTextureHeader>() == size_of::<ovrRenderAPIType>() + size_of::<ovrSizei>(),
    //                      "ovrTextureHeader size mismatch");
    //assert!(size_of::<ovrTexture>() == size_of::<ovrTextureHeader>() OVR_ON64(+4) + size_of::<usize>() * 8,
    //                      "ovrTexture size mismatch");
    //
    assert!(size_of::<ovrStatusBits>() == 4, "ovrStatusBits size mismatch");

    assert!(size_of::<ovrSessionStatus>() == 6, "ovrSessionStatus size mismatch");

    assert!(size_of::<ovrEyeRenderDesc>() == size_of::<ovrEyeType>() + size_of::<ovrFovPort>() + size_of::<ovrRecti>() +
        size_of::<ovrVector2f>() + size_of::<ovrVector3f>(),
    "ovrEyeRenderDesc size mismatch");
    assert!(size_of::<ovrTimewarpProjectionDesc>() == 4 * 3, "ovrTimewarpProjectionDesc size mismatch");

    assert!(size_of::<ovrInitFlags>() == 4, "ovrInitFlags size mismatch");
    assert!(size_of::<ovrLogLevel>() == 4, "ovrLogLevel size mismatch");

    assert!(size_of::<ovrInitParams>() == 4 + 4 + size_of::<ovrLogCallback>() + size_of::<usize>() + 4 + 4,
    "ovrInitParams size mismatch");

    if cfg!(target_pointer_width = "32") {
        assert!(size_of::<ovrHmdDesc>() ==
            size_of::<ovrHmdType>()           // Type
          + 64                                // ProductName
          + 64                                // Manufacturer
          + 2                                 // VendorId
          + 2                                 // ProductId
          + 24                                // SerialNumber
          + 2                                 // FirmwareMajor
          + 2                                 // FirmwareMinor
          + 4 * 4                             // AvailableHmdCaps - DefaultTrackingCaps
          + size_of::<ovrFovPort>() * 2       // DefaultEyeFov
          + size_of::<ovrFovPort>() * 2       // MaxEyeFov
          + size_of::<ovrSizei>()             // Resolution
          + 4                                 // DisplayRefreshRate
        , "ovrHmdDesc size mismatch");
    } else {
        assert!(size_of::<ovrHmdDesc>() ==
            size_of::<ovrHmdType>()           // Type
          + 4                                 // pad0
          + 64                                // ProductName
          + 64                                // Manufacturer
          + 2                                 // VendorId
          + 2                                 // ProductId
          + 24                                // SerialNumber
          + 2                                 // FirmwareMajor
          + 2                                 // FirmwareMinor
          + 4 * 4                             // AvailableHmdCaps - DefaultTrackingCaps
          + size_of::<ovrFovPort>() * 2       // DefaultEyeFov
          + size_of::<ovrFovPort>() * 2       // MaxEyeFov
          + size_of::<ovrSizei>()             // Resolution
          + 4                                 // DisplayRefreshRate
          + 4                                 // pad1
        , "ovrHmdDesc size mismatch");
    }
}

/// Enumerates modifications to the projection matrix based on the application's needs.
///
/// see [`ovrMatrix4f_Projection`](struct.ovrMatrix4f_Projection.html)
///
pub type ovrProjectionModifier = i32;
/// Use for generating a default projection matrix that is:
///
/// * Right-handed.
/// * Near depth values stored in the depth buffer are smaller than far depth values.
/// * Both near and far are explicitly defined.
/// * With a clipping range that is (0 to w).
///
pub const ovrProjection_None: ovrProjectionModifier = 0x00;

/// Enable if using left-handed transformations in your application.
pub const ovrProjection_LeftHanded: ovrProjectionModifier = 0x01;

/// After the projection transform is applied, far values stored in the depth buffer will be less than closer depth values.
/// NOTE: Enable only if the application is using a floating-point depth buffer for proper precision.
pub const ovrProjection_FarLessThanNear: ovrProjectionModifier = 0x02;

/// When this flag is used, the zfar value pushed into `ovrMatrix4f_Projection()` will be ignored
/// NOTE: Enable only if `ovrProjection_FarLessThanNear` is also enabled where the far clipping plane will be pushed to infinity.
pub const ovrProjection_FarClipAtInfinity: ovrProjectionModifier = 0x04;

/// Enable if the application is rendering with OpenGL and expects a projection matrix with a clipping range of (-w to w).
/// Ignore this flag if your application already handles the conversion from D3D range (0 to w) to OpenGL.
pub const ovrProjection_ClipRangeOpenGL: ovrProjectionModifier = 0x08;

/// Return values for `ovr_Detect`.
///
/// see [`ovr_Detect`](fn.ovr_Detect.html)
///
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ovrDetectResult {
    pub _align: [u64; 0],
    /// Is `ovrFalse` when the Oculus Service is not running.
    ///   This means that the Oculus Service is either uninstalled or stopped.
    ///   `IsOculusHMDConnected` will be `ovrFalse` in this case.
    ///
    /// Is `ovrTrue` when the Oculus Service is running.
    ///   This means that the Oculus Service is installed and running.
    ///   `IsOculusHMDConnected` will reflect the state of the HMD.
    pub IsOculusServiceRunning: ovrBool,

    /// Is `ovrFalse` when an Oculus HMD is not detected.
    ///   If the Oculus Service is not running, this will be `ovrFalse`.
    ///
    /// Is `ovrTrue` when an Oculus HMD is detected.
    ///   This implies that the Oculus Service is also installed and running.
    pub IsOculusHMDConnected: ovrBool,
    /// \internal struct pad.
    pub _pad0: [u8; 6],
}

#[cfg(test)]
#[test]
fn test_detect_result() {
    assert!(::std::mem::size_of::<ovrDetectResult>() == 8, "ovrDetectResult size mismatch");
}

extern "C" {
    /// Detects Oculus Runtime and Device Status
    ///
    /// Checks for Oculus Runtime and Oculus HMD device status without loading the LibOVRRT
    /// shared library.  This may be called before `ovr_Initialize()` to help decide whether or
    /// not to initialize LibOVR.
    ///
    /// **in** `timeoutMilliseconds` Specifies a timeout to wait for HMD to be attached or 0 to poll.
    ///
    /// Returns an `ovrDetectResult` object indicating the result of detection.
    ///
    /// see [`ovrDetectResult`](struct.ovrDetectResult.html)
    ///
    pub fn ovr_Detect(timeoutMilliseconds: c_int) -> ovrDetectResult;

    /// Used to generate projection from `ovrEyeDesc::Fov`.
    ///
    /// **in** `fov` Specifies the `ovrFovPort` to use.
    ///
    /// **in** `znear` Distance to near Z limit.
    ///
    /// **in** `zfar` Distance to far Z limit.
    ///
    /// **in** `projectionModFlags` A combination of the `ovrProjectionModifier` flags.
    ///
    /// Returns the calculated projection matrix.
    ///
    /// see [`ovrProjectionModifier`](struct.ovrProjectionModifier.html)
    ///
    pub fn ovrMatrix4f_Projection(fov: ovrFovPort, znear: f32, zfar: f32, projectionModFlags: c_uint) -> ovrMatrix4f;


    /// Extracts the required data from the result of `ovrMatrix4f_Projection`.
    ///
    /// **in** `projection` Specifies the project matrix from which to extract `ovrTimewarpProjectionDesc`.
    ///
    /// **in** `projectionModFlags` A combination of the `ovrProjectionModifier` flags.
    ///
    /// Returns the extracted `ovrTimewarpProjectionDesc`.
    ///
    /// see [`ovrTimewarpProjectionDesc`](struct.ovrTimewarpProjectionDesc.html)
    ///
    pub fn ovrTimewarpProjectionDesc_FromProjection(projection: ovrMatrix4f, projectionModFlags: c_uint) -> ovrTimewarpProjectionDesc;


    /// Generates an orthographic sub-projection.
    ///
    /// Used for 2D rendering, Y is down.
    ///
    /// **in** `projection` The perspective matrix that the orthographic matrix is derived from.
    ///
    /// **in** `orthoScale` Equal to `1.0f / pixelsPerTanAngleAtCenter`.
    ///
    /// **in** `orthoDistance` Equal to the distance from the camera in meters, such as 0.8m.
    ///
    /// **in** `HmdToEyeOffsetX` Specifies the offset of the eye from the center.
    ///
    /// Returns the calculated projection matrix.
    ///
    pub fn ovrMatrix4f_OrthoSubProjection(projection: ovrMatrix4f, orthoScale: ovrVector2f, orthoDistance: f32, HmdToEyeOffsetX: f32) -> ovrMatrix4f;



    /// Computes offset eye poses based on headPose returned by `ovrTrackingState`.
    ///
    /// **in** `headPose` Indicates the HMD position and orientation to use for the calculation.
    ///
    /// **in** `hmdToEyeOffset` Can be `ovrEyeRenderDesc.HmdToEyeOffset` returned from
    ///            `ovr_GetRenderDesc`. For monoscopic rendering, use a vector that is the average
    ///            of the two vectors for both eyes.
    ///
    /// **out** `outEyePoses` If `outEyePoses` are used for rendering, they should be passed to
    ///             `ovr_SubmitFrame` in `ovrLayerEyeFov::RenderPose` or `ovrLayerEyeFovDepth::RenderPose`.
    ///
    pub fn ovr_CalcEyePoses(headPose: ovrPosef, hmdToEyeOffset: *const [ovrVector3f; 2], outEyePoses: *const [ovrPosef; 2]);


    /// Returns the predicted head pose in outHmdTrackingState and offset eye poses in outEyePoses.
    ///
    /// This is a thread-safe function where caller should increment frameIndex with every frame
    /// and pass that index where applicable to functions called on the rendering thread.
    /// Assuming outEyePoses are used for rendering, it should be passed as a part of `ovrLayerEyeFov`.
    /// The caller does not need to worry about applying `HmdToEyeOffset` to the returned `outEyePoses` variables.
    ///
    /// **in** `hmd` Specifies an `ovrSession` previously returned by `ovr_Create`.
    ///
    /// **in** `frameIndex` Specifies the targeted frame index, or 0 to refer to one frame after
    ///             the last time `ovr_SubmitFrame` was called.
    ///
    /// **in** `latencyMarker` Specifies that this call is the point in time where
    ///             the "App-to-Mid-Photon" latency timer starts from. If a given `ovrLayer`
    ///             provides "SensorSampleTimestamp", that will override the value stored here.
    ///
    /// **in** `hmdToEyeOffset` Can be `ovrEyeRenderDesc.HmdToEyeOffset` returned from
    ///             `ovr_GetRenderDesc`. For monoscopic rendering, use a vector that is the average
    ///             of the two vectors for both eyes.
    ///
    /// **out** `outEyePoses` The predicted eye poses.
    ///
    /// **out** `outSensorSampleTime` The time when this function was called. May be NULL, in which case it is ignored.
    ///
    pub fn ovr_GetEyePoses(session: ovrSession, frameIndex: c_longlong, latencyMarker: ovrBool, hmdToEyeOffset: *const [ovrVector3f; 2], outEyePoses: *const [ovrPosef; 2], outSensorSampleTime: *mut f64);



    /// Tracking poses provided by the SDK come in a right-handed coordinate system. If an application
    /// is passing in `ovrProjection_LeftHanded` into `ovrMatrix4f_Projection`, then it should also use
    /// this function to flip the HMD tracking poses to be left-handed.
    ///
    /// While this utility function is intended to convert a left-handed `ovrPosef` into a right-handed
    /// coordinate system, it will also work for converting right-handed to left-handed since the
    /// flip operation is the same for both cases.
    ///
    /// **in** `inPose` that is right-handed
    ///
    /// **out** `outPose` that is requested to be left-handed (can be the same pointer to `inPose`)
    ///
    pub fn ovrPosef_FlipHandedness(inPose: *const ovrPosef, outPose: *mut ovrPosef);
}
