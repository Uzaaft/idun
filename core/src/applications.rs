use icrate::{
    objc2::rc::{Id, Shared},
    AppKit::{NSRunningApplication, NSWorkspace},
    Foundation::NSArray,
};

type IdArray<T, O = Shared> = Id<NSArray<T, O>, O>;

fn get_running_applications() -> IdArray<NSRunningApplication> {
    let ws = unsafe { icrate::AppKit::NSWorkspace::sharedWorkspace() };
    let running_applications = unsafe { ws.as_ref().runningApplications() };
    running_applications
}

pub fn get_active_applications() -> IdArray<NSRunningApplication> {
    // PERF: This should be iter and a map/filter instead of a for loop
    let mut active_apps: IdArray<NSRunningApplication> = IdArray::default();
    for running_app in get_running_applications().iter() {
        let is_active = unsafe { running_app.isActive() };
        dbg!(&is_active);
        if is_active {
            active_apps = unsafe { active_apps.arrayByAddingObject(running_app) };
        };
    }
    active_apps
}

pub fn get_focused_app() -> Id<NSRunningApplication, Shared> {
    unsafe {
        NSWorkspace::sharedWorkspace()
            .as_ref()
            .frontmostApplication()
            .unwrap()
    }
}
