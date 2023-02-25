use icrate::AppKit::NSScreen;

use crate::types::IdArray;

pub fn get_all_screens() -> IdArray<NSScreen> {
    unsafe { NSScreen::screens() }
}
