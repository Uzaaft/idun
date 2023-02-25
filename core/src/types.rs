use icrate::{
    objc2::rc::{Id, Shared},
    Foundation::NSArray,
};

pub type IdArray<T, O = Shared> = Id<NSArray<T, O>, O>;
pub type IdObject<T, O = Shared> = Id<T, O>;
