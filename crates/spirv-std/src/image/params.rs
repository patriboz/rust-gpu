/// The access permissions for the image.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AccessQualifier {
    /// A read only image.
    ReadOnly = 0,
    /// A write only image.
    WriteOnly = 1,
    /// A readable and writable image.
    ReadWrite = 2,
}

/// Whether the image uses arrayed content.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Arrayed {
    /// The image uses not arrayed content.
    False = 0,
    /// The image uses arrayed content.
    True = 1,
}

/// The dimension of the image.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Dimensionality {
    /// 1D
    OneD = 0,
    /// 2D
    TwoD = 1,
    /// 3D
    ThreeD = 2,
    /// 2D Cubemap texture
    Cube = 3,
    /// 2D Rectangle texture
    Rect = 4,
    /// 1D Buffer texture
    Buffer = 5,
    /// Vulkan subpass buffer
    SubpassData = 6,
}

/// Marker trait for arguments that accept single scalar values or vectors
/// of scalars.
pub trait ImageCoordinate<const DIM: Dimensionality>: crate::sealed::Sealed {}

impl ImageCoordinate<{ Dimensionality::OneD }> for f32 {}
impl ImageCoordinate<{ Dimensionality::TwoD }> for glam::Vec2 {}
impl ImageCoordinate<{ Dimensionality::ThreeD }> for glam::Vec3 {}
impl ImageCoordinate<{ Dimensionality::ThreeD }> for glam::Vec3A {}
impl ImageCoordinate<{ Dimensionality::Rect }> for glam::Vec2 {}
impl ImageCoordinate<{ Dimensionality::Cube }> for glam::Vec2 {}
impl ImageCoordinate<{ Dimensionality::Buffer }> for f32 {}

/// Whether a given image contains [depth] information. **Note** Whether or not
/// to perform depth comparisons is a property of the sampling code, not of this
/// type.
///
/// [depth]: https://en.wikipedia.org/wiki/Depth_map
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ImageDepth {
    /// Indicates that the image does not contain depth information.
    False = 0,
    /// Indicates that the image contains depth information.
    True = 1,
    /// Indicates that is not known ahead of time whether the image has depth
    /// information or not.
    Unknown = 2,
}

/// The underlying internal representation of the image.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ImageFormat {
    /// Representation not known at compile time.
    Unknown = 0,
    /// RGBA channels, 32 bit floating point integer.
    Rgba32f = 1,
    /// RGBA channels, 16 bit floating point integer.
    Rgba16f = 2,
    /// RGBA channels, 16 bit floating point integer.
    R32f = 3,
    /// RGBA channels, 8 bit floating point integer.
    Rgba8 = 4,
    /// RGBA channels, 8 bit signed normalized integer.
    Rgba8Snorm = 5,
    /// Red+Green channels, 32 bit floating point integer.
    Rg32f = 6,
    /// Red+Green channels, 16 bit floating point integer.
    Rg16f = 7,
    /// 32 bit unsigned integer containing two 11 bit floating point integers
    /// for the Red and Green channels, and a 10 bit floating point integer for
    /// the Blue channel.
    R11fG11fB10f = 8,
    /// Red channel, 16 bit floating point.
    R16f = 9,
    /// RGBA channel, 16 bit floating point.
    Rgba16 = 10,
    /// 32 bit unsigned integer containing three 10 bit unsigned normalized
    /// integers for the Red, Green, and Blue channels; with a 2 unsigned
    /// normalized integer for the Alpha channel.
    Rgb10A2 = 11,
    /// Red+Green channels, 16 bit unsigned integer.
    Rg16 = 12,
    /// Red+Green channels, 8 bit unsigned integer.
    Rg8 = 13,
    /// Red+Green channels, 8 bit unsigned integer.
    R16 = 14,
    /// Red channel, 8 bit unsigned integer.
    R8 = 15,
    /// RGBA channels, 16 bit signed normalized integer.
    Rgba16Snorm = 16,
    /// RGB channels, 16 bit signed normalized integer.
    Rg16Snorm = 17,
    /// Red+Green channels, 8 bit signed normalized integer.
    Rg8Snorm = 18,
    /// Red channel, 16 bit signed normalized integer.
    R16Snorm = 19,
    /// Red channel, 16 bit signed normalized integer.
    R8Snorm = 20,
    /// RGBA channels, 32 bit signed integer.
    Rgba32i = 21,
    /// RGBA channels, 16 bit signed integer.
    Rgba16i = 22,
    /// RGBA channels, 8 bit signed integer.
    Rgba8i = 23,
    /// Red channel, 32 bit signed integer.
    R32i = 24,
    /// Red+Green channels, 32 bit signed integer.
    Rg32i = 25,
    /// Red+Green channels, 16 bit signed integer.
    Rg16i = 26,
    /// Red+Green channels, 8 bit signed integer.
    Rg8i = 27,
    /// Red channel, 16 bit signed integer.
    R16i = 28,
    /// Red channel, 8 bit signed integer.
    R8i = 29,
    /// RGBA channels, 32 bit unsigned integer.
    Rgba32ui = 30,
    /// RGBA channels, 16 bit unsigned integer.
    Rgba16ui = 31,
    /// RGBA channels, 8 bit unsigned integer.
    Rgba8ui = 32,
    /// Red channel, 32 bit unsigned integer.
    R32ui = 33,
    /// 32 bit unsigned integer containing three 10 bit unsigned integers for
    /// the Red, Green, and Blue channels, and a 2 bit unsigned integer for the
    /// Alpha channel.
    Rgb10a2ui = 34,
    /// Red+Green channels, 32 bit unsigned integer.
    Rg32ui = 35,
    /// Red+Green channels, 16 bit unsigned integer.
    Rg16ui = 36,
    /// Red+Green channels, 8 bit unsigned integer.
    Rg8ui = 37,
    /// Red channel, 16 bit unsigned integer.
    R16ui = 38,
    /// Red channel, 8 bit unsigned integer.
    R8ui = 39,
}

/// Whether the image uses arrayed content.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Multisampled {
    /// The image contains single-sampled content.
    False = 0,
    /// The image contains multisampled content.
    True = 1,
}

/// Whether or not the image will be accessed in combination with a sampler.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Sampled {
    /// Indicates that it is not known ahead of time whether the image will use
    /// a sampler or not.
    Unknown = 0,
    /// The image will be used with a sampler.
    Yes = 1,
    /// The image will not be used with a sampler.
    No = 2,
}
