/// Sealed trait to ensure certain traits can't be implemented outside
/// of `spirv-std`.
pub trait Sealed {}

impl Sealed for f32 {}
impl Sealed for glam::Vec2 {}
impl Sealed for glam::Vec3 {}
impl Sealed for glam::Vec3A {}
impl Sealed for glam::Vec4 {}
