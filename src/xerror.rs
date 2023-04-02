use std::ffi;
use std::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
pub static mut x_error_xlib: MaybeUninit<x11::XErrorHandler> = MaybeUninit::uninit();

#[rustfmt::skip]
pub unsafe extern "C" fn xerror(dpy: *mut x11::Display, ee: *mut x11::XErrorEvent) -> ffi::c_int {
    if (*ee).error_code as u32 == x11::BadWindow
    || ((*ee).request_code as u32 == x11::X_SetInputFocus     && (*ee).error_code as u32 == x11::BadWindow)
    || ((*ee).request_code as u32 == x11::X_PolyText8         && (*ee).error_code as u32 == x11::BadDrawable)
    || ((*ee).request_code as u32 == x11::X_PolyFillRectangle && (*ee).error_code as u32 == x11::BadDrawable)
    || ((*ee).request_code as u32 == x11::X_PolySegment       && (*ee).error_code as u32 == x11::BadDrawable)
    || ((*ee).request_code as u32 == x11::X_ConfigureWindow   && (*ee).error_code as u32 == x11::BadWindow)
    || ((*ee).request_code as u32 == x11::X_GrabButton        && (*ee).error_code as u32 == x11::BadAccess)
    || ((*ee).request_code as u32 == x11::X_GrabKey           && (*ee).error_code as u32 == x11::BadAccess)
    || ((*ee).request_code as u32 == x11::X_CopyArea          && (*ee).error_code as u32 == x11::BadDrawable)
    {
        return 0;
    }
    
    eprintln!(
        "FATAL ERROR: request code={}, error code={}",
        (*ee).request_code,
        (*ee).error_code,
    );

    // The caller must guarentees that x_error_lib is initialized before.
    return x_error_xlib.assume_init().unwrap()(dpy, ee);
}

#[allow(unreachable_code)]
pub extern "C" fn xerror_start(_dpy: *mut x11::Display, _ee: *mut x11::XErrorEvent) -> ffi::c_int {
    panic!("ERROR: another window manager is running");
    return -1;
}
