use std::ffi::CString;
use std::mem;
use std::ptr;

use crate::xerror;

#[repr(transparent)]
pub struct XDisplay {
    dpy: *mut x11::Display,
}

impl Drop for XDisplay {
    fn drop(&mut self) {
        unsafe {
            x11::XCloseDisplay(self.dpy);
        }
    }
}

impl XDisplay {
    pub fn new(display_name: Option<&str>) -> Option<Self> {
        let dpy = if let Some(name) = display_name {
            let c_str = CString::new(name).ok()?;
            let ptr = c_str.as_ptr();
            mem::forget(c_str);
            unsafe { x11::XOpenDisplay(ptr) }
        } else {
            unsafe { x11::XOpenDisplay(ptr::null()) }
        };

        if dpy.is_null() {
            None
        } else {
            Some(Self { dpy })
        }
    }

    #[inline]
    pub fn into_raw(self) -> *mut x11::Display {
        self.dpy
    }

    #[inline]
    pub unsafe fn from_raw(dpy: *mut x11::Display) -> Self {
        Self { dpy }
    }

    pub fn check_other_wm(&mut self) {
        unsafe {
            xerror::x_error_xlib.write(Some(xerror::xerror_start));
            x11::XSelectInput(
                self.dpy,
                x11::XDefaultRootWindow(self.dpy),
                x11::SubstructureRedirectMask as i64,
            );
            x11::XSync(self.dpy, x11::False as i32);
            x11::XSetErrorHandler(Some(xerror::xerror));
            x11::XSync(self.dpy, x11::False as i32);
        }
    }
}
