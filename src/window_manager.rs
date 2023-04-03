use std::ffi::{self, CStr};

use crate::atom::*;
use crate::display::XDisplay;
use crate::util;

pub struct WindowManager {
    dpy: XDisplay,
    atoms: AtomContainer,
    screen: ffi::c_int,
    sw: ffi::c_int,
    sh: ffi::c_int,
    root: x11::Window,
}

macro_rules! cstr {
    ($bytes: literal) => {
        CStr::from_bytes_with_nul_unchecked($bytes).as_ptr()
    };
}

impl WindowManager {
    pub fn setup(dpy: XDisplay) -> Self {
        // clean up any zombies immediately
        util::sigchld(0);

        // init screen
        let screen = unsafe { x11::XDefaultScreen(dpy.get_raw()) };
        let sw = unsafe { x11::XDisplayWidth(dpy.get_raw(), screen) };
        let sh = unsafe { x11::XDisplayHeight(dpy.get_raw(), screen) };
        let root = unsafe { x11::XRootWindow(dpy.get_raw(), screen) };

        // init atoms
        let mut atoms = AtomContainer::default();
        atoms.utf8string.write(unsafe {
            x11::XInternAtom(dpy.get_raw(), cstr!(b"UTF8_STRING\0"), x11::False as i32)
        });
        atoms.wmatom.insert(WmAtomType::WmProtocols, unsafe {
            x11::XInternAtom(dpy.get_raw(), cstr!(b"WM_PROTOCOLS\0"), x11::False as i32)
        });
        atoms.wmatom.insert(WmAtomType::WmDelete, unsafe {
            x11::XInternAtom(
                dpy.get_raw(),
                cstr!(b"WM_DELETE_WINDOW\0"),
                x11::False as i32,
            )
        });
        atoms.wmatom.insert(WmAtomType::WmState, unsafe {
            x11::XInternAtom(dpy.get_raw(), cstr!(b"WM_STATE\0"), x11::False as i32)
        });
        atoms.wmatom.insert(WmAtomType::WmTakeFocus, unsafe {
            x11::XInternAtom(dpy.get_raw(), cstr!(b"WM_TAKE_FOCUS\0"), x11::False as i32)
        });

        Self {
            dpy,
            atoms,
            screen,
            sw,
            sh,
            root,
        }
    }
}
