use std::collections::HashMap;
use std::ffi::CStr;
use std::mem::MaybeUninit;

use crate::display::Display;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum WmAtomType {
    Protocols,
    Delete,
    State,
    TakeFocus,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetAtomType {
    Supported,
    WmName,
    WmState,
    WmCheck,
    SystemTray,
    SystemTrayOp,
    SystemTrayOrientation,
    SystemTrayOrientationHorz,
    WmFullScreen,
    ActiveWindow,
    WmWindowType,
    WmWindowTypeDiag,
    ClientList,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum XAtomType {
    Manager,
    Xembed,
    XembedInfo,
}

pub struct AtomContainer {
    utf8string: MaybeUninit<x11::Atom>,
    wmatom: HashMap<WmAtomType, x11::Atom>,
    netatom: HashMap<NetAtomType, x11::Atom>,
    xatom: HashMap<XAtomType, x11::Atom>,
}

impl Default for AtomContainer {
    fn default() -> Self {
        let wmatom = HashMap::<WmAtomType, x11::Atom>::with_capacity(25);
        let netatom = HashMap::<NetAtomType, x11::Atom>::with_capacity(25);
        let xatom = HashMap::<XAtomType, x11::Atom>::with_capacity(25);

        Self {
            utf8string: MaybeUninit::uninit(),
            wmatom,
            netatom,
            xatom,
        }
    }
}

impl AtomContainer {
    pub fn set_utf8string(&mut self, dpy: &Display, atom_name: &CStr) {
        self.utf8string.write(unsafe {
            x11::XInternAtom(dpy.get_ptr(), atom_name.as_ptr(), x11::False as i32)
        });
    }
    pub fn insert_wmatom(&mut self, dpy: &Display, r#type: WmAtomType, atom_name: &CStr) {
        self.wmatom.insert(r#type, unsafe {
            x11::XInternAtom(dpy.get_ptr(), atom_name.as_ptr(), x11::False as i32)
        });
    }

    pub fn insert_netatom(&mut self, dpy: &Display, r#type: NetAtomType, atom_name: &CStr) {
        self.netatom.insert(r#type, unsafe {
            x11::XInternAtom(dpy.get_ptr(), atom_name.as_ptr(), x11::False as i32)
        });
    }

    pub fn insert_xatom(&mut self, dpy: &Display, r#type: XAtomType, atom_name: &CStr) {
        self.xatom.insert(r#type, unsafe {
            x11::XInternAtom(dpy.get_ptr(), atom_name.as_ptr(), x11::False as i32)
        });
    }
}
