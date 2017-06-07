use ::winapi::minwindef::UINT;
use ::winapi::winnt::WCHAR;
use ::winapi::guiddef::GUID;

use ::libc::{
    c_int,
    c_void
};

use ::ovrResult;

pub const OVR_AUDIO_MAX_DEVICE_STR_SIZE: usize = 128;

extern "C" {
    /// Gets the ID of the preferred VR audio output device.
    ///
    /// **out** `deviceOutId` The ID of the user's preferred VR audio device to use, which will be valid upon a successful return value, else it will be `WAVE_MAPPER`.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    pub fn ovr_GetAudioDeviceOutWaveId(deviceOutId: *mut UINT) -> ovrResult;

    /// Gets the ID of the preferred VR audio input device.
    ///
    /// **out** `deviceInId` The ID of the user's preferred VR audio device to use, which will be valid upon a successful return value, else it will be `WAVE_MAPPER`.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    pub fn ovr_GetAudioDeviceInWaveId(deviceInId: *mut UINT) -> ovrResult;


    /// Gets the GUID of the preferred VR audio device as a string.
    ///
    /// **out** `deviceOutStrBuffer` A buffer where the GUID string for the device will copied to.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    pub fn ovr_GetAudioDeviceOutGuidStr(deviceOutStrBuffer: [WCHAR; OVR_AUDIO_MAX_DEVICE_STR_SIZE]) -> ovrResult;


    /// Gets the GUID of the preferred VR audio device.
    ///
    /// **out** `deviceOutGuid` The GUID of the user's preferred VR audio device to use, which will be valid upon a successful return value, else it will be NULL.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    pub fn ovr_GetAudioDeviceOutGuid(deviceOutGuid: *mut GUID) -> ovrResult;


    /// Gets the GUID of the preferred VR microphone device as a string.
    ///
    /// **out** `deviceInStrBuffer` A buffer where the GUID string for the device will copied to.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    pub fn ovr_GetAudioDeviceInGuidStr(deviceInStrBuffer: [WCHAR; OVR_AUDIO_MAX_DEVICE_STR_SIZE]) -> ovrResult;


    /// Gets the GUID of the preferred VR microphone device.
    ///
    /// **out** `deviceInGuid` The GUID of the user's preferred VR audio device to use, which will be valid upon a successful return value, else it will be NULL.
    ///
    /// Returns an `ovrResult` indicating success or failure. In the case of failure, use
    ///         `ovr_GetLastErrorInfo` to get more information.
    ///
    pub fn ovr_GetAudioDeviceInGuid(deviceInGuid: *mut GUID) -> ovrResult;
}

/// Modes used to generate Touch Haptics from audio PCM buffer.
///
pub type ovrHapticsGenMode = i32;
/// Point sample original signal at Haptics frequency
pub const ovrHapticsGenMode_PointSample: ovrHapticsGenMode = 0;
pub const ovrHapticsGenMode_Count: ovrHapticsGenMode = 1;

/// Store audio PCM data (as 32b float samples) for an audio channel.
///
/// Note: needs to be released with `ovr_ReleaseAudioChannelData` to avoid memory leak.
///
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ovrAudioChannelData {
    /// Samples stored as floats \[-1.0f, 1.0f\].
    pub Samples: *const f32,
    /// Number of samples
    pub SamplesCount: c_int,
    /// Frequency (e.g. 44100)
    pub Frequency: c_int,
}

/// Store a full Haptics clip, which can be used as data source for multiple `ovrHapticsBuffer`s.
///
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ovrHapticsClip {
    /// Samples stored in opaque format
    pub Samples: *const c_void,
    /// Number of samples
    pub SamplesCount: c_int
}
extern "C" {
/// Reads an audio channel from Wav (Waveform Audio File) data.
/// Input must be a byte buffer representing a valid Wav file. Audio samples from the specified channel are read,
/// converted to float \[-1.0f, 1.0f\] and returned through `ovrAudioChannelData`.
///
/// Supported formats: PCM 8b, 16b, 32b and IEEE float (little-endian only).
///
/// **out** `outAudioChannel` output audio channel data.
///
/// **in** `inputData` a binary buffer representing a valid Wav file data.
///
/// **in** `dataSizeInBytes` size of the buffer in bytes.
///
/// **in** `stereoChannelToUse` audio channel index to extract (0 for mono).
///
pub fn ovr_ReadWavFromBuffer(outAudioChannel: *mut ovrAudioChannelData, inputData: *const c_void, dataSizeInBytes: c_int, stereoChannelToUse: c_int) -> ovrResult;

/// Generates playable Touch Haptics data from an audio channel.
///
/// **out** `outHapticsClip` generated Haptics clip.
///
/// **in** `audioChannel` input audio channel data.
///
/// **in** `genMode` mode used to convert and audio channel data to Haptics data.
///
pub fn ovr_GenHapticsFromAudioData(outHapticsClip: *mut ovrHapticsClip, audioChannel: *const ovrAudioChannelData, genMode: ovrHapticsGenMode) -> ovrResult;

/// Releases memory allocated for `ovrAudioChannelData`. Must be called to avoid memory leak.
///
/// **in** `audioChannel` pointer to an audio channel
///
pub fn ovr_ReleaseAudioChannelData(audioChannel: *mut ovrAudioChannelData);

/// Releases memory allocated for `ovrHapticsClip`. Must be called to avoid memory leak.
///
/// **in** `hapticsClip` pointer to a haptics clip
///
pub fn ovr_ReleaseHapticsClip(hapticsClip: *mut ovrHapticsClip);
}