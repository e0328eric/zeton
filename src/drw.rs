use std::ffi::{self, CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use crate::display::Display;
use crate::error::{self, ZetonError};

pub struct Font<'dis> {
    dpy: &'dis Display,
    height: ffi::c_uint,
    xfont: *mut x11::XftFont,
    pattern: *mut x11::FcPattern,
    next: Option<Box<Font<'dis>>>,
}

impl<'dis> Font<'dis> {
    fn new(
        dpy: &'dis Display,
        screen: ffi::c_int,
        fontname: Option<&CStr>,
        font_pattern: Option<*mut x11::FcPattern>,
    ) -> error::Result<Self> {
        let mut xfont = ptr::null_mut();
        let mut pattern = ptr::null_mut();

        if let Some(fontname) = fontname {
            if {
                xfont = unsafe { x11::XftFontOpenName(dpy.get_ptr(), screen, fontname.as_ptr()) };
                xfont.is_null()
            } {
                return Err(ZetonError::CannotLoadFontFromName);
            }
            if {
                pattern = unsafe { x11::FcNameParse(fontname.as_ptr() as *mut x11::FcChar8) };
                pattern.is_null()
            } {
                return Err(ZetonError::CannotLoadFontFromPattern);
            }
        } else if let Some(font_pattern) = font_pattern {
            if {
                xfont = unsafe { x11::XftFontOpenPattern(dpy.get_ptr(), font_pattern) };
                xfont.is_null()
            } {
                return Err(ZetonError::CannotLoadFontFromPattern);
            }
        } else {
            return Err(ZetonError::FontNotSpecified);
        }

        let mut is_col = false as x11::FcBool;
        if unsafe {
            x11::FcPatternGetBool(
                (*xfont).pattern,
                x11::FC_COLOR as *const _ as *const c_char,
                0,
                &mut is_col,
            )
        } == x11::_FcResult_FcResultMatch
            && is_col != 0
        {
            unsafe { x11::XftFontClose(dpy.get_ptr(), xfont) };
            return Err(ZetonError::CannotGetFontPattern);
        }

        Ok(Self {
            dpy,
            height: unsafe { (*xfont).ascent + (*xfont).descent } as ffi::c_uint,
            xfont,
            pattern,
            next: None,
        })
    }

    #[inline]
    pub fn get_height(&self) -> ffi::c_uint {
        self.height
    }
}

impl Drop for Font<'_> {
    fn drop(&mut self) {
        if !self.pattern.is_null() {
            unsafe {
                x11::FcPatternDestroy(self.pattern);
            }
        }
        unsafe {
            x11::XftFontClose(self.dpy.get_ptr(), self.xfont);
        }
    }
}

pub struct Draw<'dis> {
    dpy: &'dis Display,
    screen: ffi::c_int,
    root: x11::Window,
    width: ffi::c_uint,
    height: ffi::c_uint,
    drawable: x11::Drawable,
    gc: x11::GC,
    scheme: Option<*mut x11::XftColor>,
    fonts: Option<Box<Font<'dis>>>,
}

impl<'dis> Draw<'dis> {
    pub fn new(
        dpy: &'dis Display,
        screen: ffi::c_int,
        root: x11::Window,
        width: ffi::c_uint,
        height: ffi::c_uint,
    ) -> Self {
        let output = Self {
            dpy,
            screen,
            root,
            width,
            height,
            drawable: unsafe {
                x11::XCreatePixmap(
                    dpy.get_ptr(),
                    root,
                    width,
                    height,
                    x11::XDefaultDepth(dpy.get_ptr(), screen) as ffi::c_uint,
                )
            },
            gc: unsafe { x11::XCreateGC(dpy.get_ptr(), root, 0, ptr::null_mut()) },
            scheme: None,
            fonts: None,
        };
        unsafe {
            x11::XSetLineAttributes(
                dpy.get_ptr(),
                output.gc,
                1,
                x11::LineSolid as ffi::c_int,
                x11::CapButt as ffi::c_int,
                x11::JoinMiter as ffi::c_int,
            );
        }

        output
    }

    pub fn resize(&mut self, width: ffi::c_uint, height: ffi::c_uint) {
        self.width = width;
        self.height = height;
        if self.drawable != 0 {
            unsafe {
                x11::XFreePixmap(self.dpy.get_ptr(), self.drawable);
            }
        }

        self.drawable = unsafe {
            x11::XCreatePixmap(
                self.dpy.get_ptr(),
                self.root,
                width,
                height,
                x11::XDefaultDepth(self.dpy.get_ptr(), self.screen) as ffi::c_uint,
            )
        };
    }

    pub fn fontset_create(&mut self, fonts: &[String]) -> error::Result<()> {
        let mut ret = None;

        for font in fonts {
            let mut current = Box::new(Font::new(
                self.dpy,
                self.screen,
                Some(CString::new(font.as_str())?.as_c_str()),
                None,
            )?);
            current.next = ret;
            ret = Some(current);
        }

        self.fonts = ret;

        Ok(())
    }

    #[inline]
    pub fn get_font_height(&self) -> Option<ffi::c_uint> {
        self.fonts.as_ref().map(|fonts| fonts.height)
    }
}

impl Drop for Draw<'_> {
    fn drop(&mut self) {
        unsafe {
            x11::XFreePixmap(self.dpy.get_ptr(), self.drawable);
            x11::XFreeGC(self.dpy.get_ptr(), self.gc);
        }
    }
}
