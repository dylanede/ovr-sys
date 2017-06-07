extern crate ovr_sys;

use ovr_sys::*;

fn try<F>(f: F) -> Result<(), Box<ovrErrorInfo>> where F: FnOnce() -> ovrResult {
    let result = f();
    if OVR_SUCCESS(result) {
        Ok(())
    } else {
        let mut info = Box::new(unsafe{::std::mem::zeroed()});
        unsafe{ ovr_GetLastErrorInfo(&mut *info as *mut _) }
        Err(info)
    }
}

fn main() {
    unsafe {
        let mut params: ovrInitParams = ::std::mem::zeroed();
        params.Flags |= ovrInit_RequestVersion;
        params.RequestedMinorVersion = OVR_MINOR_VERSION;
        try(|| ovr_Initialize(&params as *const _)).unwrap();
        let mut session: ovrSession = ::std::mem::zeroed();
        let mut luid: ovrGraphicsLuid = ::std::mem::zeroed();
        try(|| ovr_Create(&mut session as *mut _, &mut luid as *mut _)).unwrap();
        assert!(!session.is_null());
        println!("{:?}", luid);
        ovr_Destroy(session);
        ovr_Shutdown();
    }
}