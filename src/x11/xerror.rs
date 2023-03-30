use std::ffi;
use std::mem::MaybeUninit;

use x11_sys as x11;

type XErrorCallback = fn(*mut x11::Display, *mut x11::XErrorEvent) -> ffi::c_int;

pub static mut X_ERROR_XLIB: MaybeUninit<XErrorCallback> = MaybeUninit::uninit();

pub unsafe extern "C" fn xerror(dpy: *mut x11::Display, ee: *mut x11::XErrorEvent) -> ffi::c_int {
    if u32::from((*ee).error_code) == x11::BadWindow
        || (u32::from((*ee).request_code) == x11::X_SetInputFocus
            && u32::from((*ee).error_code) == x11::BadMatch)
        || (u32::from((*ee).request_code) == x11::X_PolyText8
            && u32::from((*ee).error_code) == x11::BadDrawable)
        || (u32::from((*ee).request_code) == x11::X_PolyFillRectangle
            && u32::from((*ee).error_code) == x11::BadDrawable)
        || (u32::from((*ee).request_code) == x11::X_PolySegment
            && u32::from((*ee).error_code) == x11::BadDrawable)
        || (u32::from((*ee).request_code) == x11::X_ConfigureWindow
            && u32::from((*ee).error_code) == x11::BadMatch)
        || (u32::from((*ee).request_code) == x11::X_GrabButton
            && u32::from((*ee).error_code) == x11::BadAccess)
        || (u32::from((*ee).request_code) == x11::X_GrabKey
            && u32::from((*ee).error_code) == x11::BadAccess)
        || (u32::from((*ee).request_code) == x11::X_CopyArea
            && u32::from((*ee).error_code) == x11::BadDrawable)
    {
        return 0;
    }

    eprintln!(
        "FATAL ERROR: request code={}, error code={}",
        (*ee).request_code,
        (*ee).error_code
    );

    // The caller must guarentees that XErrorCallback is initialized before.
    return X_ERROR_XLIB.assume_init()(dpy, ee);
}
