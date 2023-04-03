use std::collections::HashMap;
use std::mem::MaybeUninit;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum WmAtomType {
    WmProtocols,
    WmDelete,
    WmState,
    WmTakeFocus,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetAtomType {
    NetSupported,
    NetWmName,
    NetWmState,
    NetWmCheck,
    NetSystemTray,
    NetSystemTrayOp,
    NetSystemTrayOrientation,
    NetSystemTrayOrientationHorz,
    NetWmFullScreen,
    NetActiveWindow,
    NetWmWindowType,
    NetWmWindowTypeDiag,
    NetClientList,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum XAtomType {
    Manager,
    Xembed,
    XembedInfo,
}

pub struct AtomContainer {
    pub utf8string: MaybeUninit<x11::Atom>,
    pub wmatom: HashMap<WmAtomType, x11::Atom>,
    pub netatom: HashMap<NetAtomType, x11::Atom>,
    pub xatom: HashMap<XAtomType, x11::Atom>,
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
