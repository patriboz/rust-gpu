use glam::{Vec2, Vec3A, Vec4};

#[allow(unused_attributes)]
#[spirv(sampler)]
#[derive(Copy, Clone)]
pub struct Sampler {
    _x: u32,
}

#[allow(unused_attributes)]
#[spirv(image(
    // sampled_type is hardcoded to f32 for now
    dim = "Dim2D",
    depth = 0,
    arrayed = 0,
    multisampled = 0,
    sampled = 1,
    image_format = "Unknown"
))]
#[derive(Copy, Clone)]
pub struct Image2d {
    _x: u32,
}

impl Image2d {
    #[spirv_std_macros::gpu_only]
    pub fn sample(&self, sampler: Sampler, coord: Vec2) -> Vec4 {
        unsafe {
            let mut result = Default::default();
            asm!(
                "%image = OpLoad _ {1}",
                "%sampler = OpLoad _ {2}",
                "%coord = OpLoad _ {3}",
                "%sampledImage = OpSampledImage _ %image %sampler",
                "%result = OpImageSampleImplicitLod _ %sampledImage %coord",
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

#[allow(unused_attributes)]
#[spirv(image(
    // sampled_type is hardcoded to f32 for now
    dim = "Dim2D",
    depth = 0,
    arrayed = 1,
    multisampled = 0,
    sampled = 1,
    image_format = "Unknown"
))]
#[derive(Copy, Clone)]
pub struct Image2dArray {
    _x: u32,
}

impl Image2dArray {
    #[spirv_std_macros::gpu_only]
    pub fn sample(&self, sampler: Sampler, coord: Vec3A) -> Vec4 {
        unsafe {
            let mut result = Default::default();
            asm!(
                "%image = OpLoad _ {1}",
                "%sampler = OpLoad _ {2}",
                "%coord = OpLoad _ {3}",
                "%sampledImage = OpSampledImage _ %image %sampler",
                "%result = OpImageSampleImplicitLod _ %sampledImage %coord",
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

#[allow(unused_attributes)]
#[spirv(sampled_image)]
#[derive(Copy, Clone)]
pub struct SampledImage<I> {
    _image: I,
}

impl SampledImage<Image2d> {
    #[spirv_std_macros::gpu_only]
    pub fn sample(&self, coord: Vec2) -> Vec4 {
        unsafe {
            let mut result = Default::default();
            asm!(
                "%sampledImage = OpLoad _ {1}",
                "%coord = OpLoad _ {2}",
                "%result = OpImageSampleImplicitLod _ %sampledImage %coord",
                "OpStore {0} %result",
                in(reg) &mut result,
                in(reg) self,
                in(reg) &coord
            );
            result
        }
    }
}
