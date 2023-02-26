use icrate::{
    objc2::rc::{Id, Shared},
    AppKit::{NSScreen, NSWindow, NSWorkspace},
    Foundation::NSArray,
};

use crate::types::{IdArray, IdObject};

#[derive(Debug)]
// TODO: This is just what I have for now. It'll expand as I get aware of more needs
pub struct State {
    pub screens: Id<NSArray<NSScreen, Shared>, Shared>,
    pub windows: Id<NSArray<NSWindow, Shared>, Shared>,
    pub workspace: Id<NSWorkspace, Shared>,
}
