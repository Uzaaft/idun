use icrate::AppKit::NSScreen;

use crate::types::{IdArray, IdObject};

pub fn get_all_screens() -> IdArray<NSScreen> {
    unsafe { NSScreen::screens() }
}

pub fn get_focused_screen() -> IdObject<NSScreen> {
    unsafe { NSScreen::mainScreen().unwrap() }
}
