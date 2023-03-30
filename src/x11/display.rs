use x11_sys as x11;

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
    pub fn new() -> Self {
        Self {
            dpy: unsafe { x11::XOpenDisplay(std::ptr::null()) },
        }
    }
}
