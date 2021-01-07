//! Image types

mod params;

use glam::Vec4;

pub use params::{
    AccessQualifier, Arrayed, Dimensionality, ImageCoordinate, ImageDepth, ImageFormat,
    Multisampled, Sampled,
};

macro_rules! basic_image_type {
    ($($(#[$($meta:meta)+])* $dim:path => $name:ident),+ $(,)?) => {
        $(
            $(#[$($meta)+])*
            pub type $name = Image<
            { $dim },
            { ImageDepth::Unknown },
            { Arrayed::False },
            { Multisampled::False },
            { Sampled::Unknown },
            { ImageFormat::Unknown },
            { None }
            >;
        )+
    }
}

basic_image_type! {
    /// A convenience type alias for a one dimensional image.
    Dimensionality::OneD => Image1d,
    /// A convenience type alias for a two dimensional image.
    Dimensionality::TwoD => Image2d,
    /// A convenience type alias for a three dimensional image.
    Dimensionality::ThreeD => Image3d,
    /// A convenience type alias for a cube buffer image.
    Dimensionality::Cube => ImageCube,
    /// A convenience type alias for a rectangle buffer image.
    Dimensionality::Rect => ImageRect,
    /// A convenience type alias for a buffer image.
    Dimensionality::Buffer => ImageBuffer,
}

/// An opaque image type. Corresponds to `OpTypeImage`.
#[allow(unused_attributes)]
#[spirv(image)]
#[derive(Copy, Clone)]
pub struct Image<
    const DIM: Dimensionality,
    const DEPTH: ImageDepth,
    const ARRAYED: Arrayed,
    const MULTISAMPLED: Multisampled,
    const SAMPLED: Sampled,
    const FORMAT: ImageFormat,
    const ACCESS_QUALIFIER: Option<AccessQualifier>,
> {
    _x: u32,
}

impl<
        const DIM: Dimensionality,
        const DEPTH: ImageDepth,
        const ARRAYED: Arrayed,
        const MULTISAMPLED: Multisampled,
        const SAMPLED: Sampled,
        const FORMAT: ImageFormat,
        const ACCESS_QUALIFIER: Option<AccessQualifier>,
    > Image<DIM, DEPTH, ARRAYED, MULTISAMPLED, SAMPLED, FORMAT, ACCESS_QUALIFIER>
{
    pub fn sample(&self, sampler: Sampler, coord: impl ImageCoordinate<{ DIM }>) -> Vec4 {
        #[cfg(not(target_arch = "spirv"))]
        {
            let _ = sampler;
            let _ = coord;
            panic!("Image sampling not supported on CPU");
        }

        #[cfg(target_arch = "spirv")]
        unsafe {
            let mut result = Default::default();
            asm!(
                "%typeSampledImage = OpTypeSampledImage typeof*{1}",
                "%image = OpLoad typeof*{1} {1}",
                "%sampler = OpLoad typeof*{2} {2}",
                "%coord = OpLoad typeof*{3} {3}",
                "%sampledImage = OpSampledImage %typeSampledImage %image %sampler",
                "%result = OpImageSampleImplicitLod typeof*{0} %sampledImage %coord",
                "OpStore {0} %result",
                in(reg) &mut result,
                in(reg) self,
                in(reg) &sampler,
                in(reg) &coord
            );
            result
        }
    }
}

/// An opaque reference to settings that describe how to access, filter, or
/// sample an image.
#[allow(unused_attributes)]
#[spirv(sampler)]
#[derive(Copy, Clone)]
pub struct Sampler {
    _x: u32,
}

/// An image combined with a sampler, enabling filtered accesses of the
/// image’s contents.
#[allow(unused_attributes)]
#[spirv(sampled_image)]
#[derive(Copy, Clone)]
pub struct SampledImage<I> {
    _image: I,
}

impl<
        const DIM: Dimensionality,
        const DEPTH: ImageDepth,
        const ARRAYED: Arrayed,
        const MULTISAMPLED: Multisampled,
        const SAMPLED: Sampled,
        const FORMAT: ImageFormat,
        const ACCESS_QUALIFIER: Option<AccessQualifier>,
    > SampledImage<Image<DIM, DEPTH, ARRAYED, MULTISAMPLED, SAMPLED, FORMAT, ACCESS_QUALIFIER>>
{
    pub fn sample(&self, coord: impl ImageCoordinate<{ DIM }>) -> Vec4 {
        #[cfg(not(target_arch = "spirv"))]
        {
            let _ = coord;
            panic!("Image sampling not supported on CPU");
        }
        #[cfg(target_arch = "spirv")]
        unsafe {
            let mut result = Default::default();
            asm!(
                "%sampledImage = OpLoad typeof*{1} {1}",
                "%coord = OpLoad typeof*{2} {2}",
                "%result = OpImageSampleImplicitLod typeof*{0} %sampledImage %coord",
                "OpStore {0} %result",
                in(reg) &mut result,
                in(reg) self,
                in(reg) &coord
            );
            result
        }
    }
}
