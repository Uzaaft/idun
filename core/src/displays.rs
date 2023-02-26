use icrate::{
    objc2::rc::{Id, Shared},
    AppKit::NSScreen,
    Foundation::NSArray,
};

pub fn get_all_screens() -> Id<NSArray<NSScreen, Shared>, Shared> {
    unsafe { NSScreen::screens() }
}

pub fn get_focused_screen() -> Id<NSScreen, Shared> {
    unsafe { NSScreen::mainScreen().unwrap() }
}
