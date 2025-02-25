use super::CodegenCx;
use crate::abi::ConvSpirvType;
use crate::builder_spirv::{SpirvConst, SpirvValue, SpirvValueExt};
use crate::spirv_type::SpirvType;
use rspirv::spirv::Word;
use rustc_codegen_ssa::mir::place::PlaceRef;
use rustc_codegen_ssa::traits::{ConstMethods, MiscMethods, StaticMethods};
use rustc_middle::bug;
use rustc_middle::mir::interpret::{AllocId, Allocation, GlobalAlloc, Pointer, ScalarMaybeUninit};
use rustc_middle::ty::layout::TyAndLayout;
use rustc_mir::interpret::Scalar;
use rustc_span::symbol::Symbol;
use rustc_span::{Span, DUMMY_SP};
use rustc_target::abi::{self, AddressSpace, HasDataLayout, Integer, LayoutOf, Primitive, Size};

impl<'tcx> CodegenCx<'tcx> {
    pub fn constant_u8(&self, span: Span, val: u8) -> SpirvValue {
        let ty = SpirvType::Integer(8, false).def(span, self);
        self.builder.def_constant(SpirvConst::U32(ty, val as u32))
    }

    pub fn constant_u16(&self, span: Span, val: u16) -> SpirvValue {
        let ty = SpirvType::Integer(16, false).def(span, self);
        self.builder.def_constant(SpirvConst::U32(ty, val as u32))
    }

    pub fn constant_i32(&self, span: Span, val: i32) -> SpirvValue {
        let ty = SpirvType::Integer(32, !self.kernel_mode).def(span, self);
        self.builder.def_constant(SpirvConst::U32(ty, val as u32))
    }

    pub fn constant_u32(&self, span: Span, val: u32) -> SpirvValue {
        let ty = SpirvType::Integer(32, false).def(span, self);
        self.builder.def_constant(SpirvConst::U32(ty, val))
    }

    pub fn constant_u64(&self, span: Span, val: u64) -> SpirvValue {
        let ty = SpirvType::Integer(64, false).def(span, self);
        self.builder.def_constant(SpirvConst::U64(ty, val))
    }

    pub fn constant_int(&self, ty: Word, val: u64) -> SpirvValue {
        match self.lookup_type(ty) {
            SpirvType::Integer(8, false) => self
                .builder
                .def_constant(SpirvConst::U32(ty, val as u8 as u32)),
            SpirvType::Integer(16, false) => self
                .builder
                .def_constant(SpirvConst::U32(ty, val as u16 as u32)),
            SpirvType::Integer(32, false) => {
                self.builder.def_constant(SpirvConst::U32(ty, val as u32))
            }
            SpirvType::Integer(64, false) => self.builder.def_constant(SpirvConst::U64(ty, val)),
            SpirvType::Integer(8, true) => self
                .builder
                .def_constant(SpirvConst::U32(ty, val as i64 as i8 as u32)),
            SpirvType::Integer(16, true) => self
                .builder
                .def_constant(SpirvConst::U32(ty, val as i64 as i16 as u32)),
            SpirvType::Integer(32, true) => self
                .builder
                .def_constant(SpirvConst::U32(ty, val as i64 as i32 as u32)),
            SpirvType::Integer(64, true) => self.builder.def_constant(SpirvConst::U64(ty, val)),
            SpirvType::Bool => match val {
                0 => self.builder.def_constant(SpirvConst::Bool(ty, false)),
                1 => self.builder.def_constant(SpirvConst::Bool(ty, true)),
                _ => self
                    .tcx
                    .sess
                    .fatal(&format!("Invalid constant value for bool: {}", val)),
            },
            SpirvType::Integer(128, _) => {
                let result = self.undef(ty);
                self.zombie_no_span(result.def_cx(self), "u128 constant");
                result
            }
            other => self.tcx.sess.fatal(&format!(
                "constant_int invalid on type {}",
                other.debug(ty, self)
            )),
        }
    }

    pub fn constant_f32(&self, span: Span, val: f32) -> SpirvValue {
        let ty = SpirvType::Float(32).def(span, self);
        self.builder
            .def_constant(SpirvConst::F32(ty, val.to_bits()))
    }

    pub fn constant_f64(&self, span: Span, val: f64) -> SpirvValue {
        let ty = SpirvType::Float(64).def(span, self);
        self.builder
            .def_constant(SpirvConst::F64(ty, val.to_bits()))
    }

    pub fn constant_float(&self, ty: Word, val: f64) -> SpirvValue {
        match self.lookup_type(ty) {
            SpirvType::Float(32) => self
                .builder
                .def_constant(SpirvConst::F32(ty, (val as f32).to_bits())),
            SpirvType::Float(64) => self
                .builder
                .def_constant(SpirvConst::F64(ty, val.to_bits())),
            other => self.tcx.sess.fatal(&format!(
                "constant_float invalid on type {}",
                other.debug(ty, self)
            )),
        }
    }

    pub fn constant_bool(&self, span: Span, val: bool) -> SpirvValue {
        let ty = SpirvType::Bool.def(span, self);
        self.builder.def_constant(SpirvConst::Bool(ty, val))
    }

    pub fn constant_composite(&self, ty: Word, val: Vec<Word>) -> SpirvValue {
        self.builder.def_constant(SpirvConst::Composite(ty, val))
    }

    pub fn constant_null(&self, ty: Word) -> SpirvValue {
        self.builder.def_constant(SpirvConst::Null(ty))
    }

    pub fn undef(&self, ty: Word) -> SpirvValue {
        self.builder.def_constant(SpirvConst::Undef(ty))
    }
}

impl<'tcx> ConstMethods<'tcx> for CodegenCx<'tcx> {
    fn const_null(&self, t: Self::Type) -> Self::Value {
        self.constant_null(t)
    }
    fn const_undef(&self, ty: Self::Type) -> Self::Value {
        self.undef(ty)
    }
    fn const_int(&self, t: Self::Type, i: i64) -> Self::Value {
        self.constant_int(t, i as u64)
    }
    fn const_uint(&self, t: Self::Type, i: u64) -> Self::Value {
        self.constant_int(t, i)
    }
    fn const_uint_big(&self, t: Self::Type, u: u128) -> Self::Value {
        self.constant_int(t, u as u64)
    }
    fn const_bool(&self, val: bool) -> Self::Value {
        self.constant_bool(DUMMY_SP, val)
    }
    fn const_i32(&self, i: i32) -> Self::Value {
        self.constant_i32(DUMMY_SP, i)
    }
    fn const_u32(&self, i: u32) -> Self::Value {
        self.constant_u32(DUMMY_SP, i)
    }
    fn const_u64(&self, i: u64) -> Self::Value {
        self.constant_u64(DUMMY_SP, i)
    }
    fn const_usize(&self, i: u64) -> Self::Value {
        let ptr_size = self.tcx.data_layout.pointer_size.bits() as u32;
        let t = SpirvType::Integer(ptr_size, false).def(DUMMY_SP, self);
        self.constant_int(t, i)
    }
    fn const_u8(&self, i: u8) -> Self::Value {
        self.constant_u8(DUMMY_SP, i)
    }
    fn const_real(&self, t: Self::Type, val: f64) -> Self::Value {
        self.constant_float(t, val)
    }

    fn const_str(&self, s: Symbol) -> (Self::Value, Self::Value) {
        let len = s.as_str().len();
        let str_ty = self
            .layout_of(self.tcx.types.str_)
            .spirv_type(DUMMY_SP, self);
        // FIXME(eddyb) include the actual byte data.
        (
            self.make_constant_pointer(DUMMY_SP, self.undef(str_ty)),
            self.const_usize(len as u64),
        )
    }
    fn const_struct(&self, elts: &[Self::Value], _packed: bool) -> Self::Value {
        // Presumably this will get bitcasted to the right type?
        let field_types = elts.iter().map(|f| f.ty).collect::<Vec<_>>();
        let (field_offsets, size, align) = crate::abi::auto_struct_layout(self, &field_types);
        let struct_ty = SpirvType::Adt {
            def_id: None,
            size,
            align,
            field_types,
            field_offsets,
            field_names: None,
            is_block: false,
        }
        .def(DUMMY_SP, self);
        self.constant_composite(struct_ty, elts.iter().map(|f| f.def_cx(self)).collect())
    }

    fn const_to_opt_uint(&self, v: Self::Value) -> Option<u64> {
        self.builder.lookup_const_u64(v)
    }
    fn const_to_opt_u128(&self, v: Self::Value, sign_ext: bool) -> Option<u128> {
        self.builder.lookup_const_u64(v).map(|v| {
            if sign_ext {
                v as i64 as i128 as u128
            } else {
                v as u128
            }
        })
    }

    fn scalar_to_backend(
        &self,
        scalar: Scalar,
        layout: &abi::Scalar,
        ty: Self::Type,
    ) -> Self::Value {
        match scalar {
            Scalar::Int(int) => {
                assert_eq!(int.size(), layout.value.size(self));
                let data = int.to_bits(int.size()).unwrap();

                match layout.value {
                    Primitive::Int(int_size, int_signedness) => match self.lookup_type(ty) {
                        SpirvType::Integer(width, spirv_signedness) => {
                            assert_eq!(width as u64, int_size.size().bits());
                            if !self.kernel_mode {
                                assert_eq!(spirv_signedness, int_signedness);
                            }
                            self.constant_int(ty, data as u64)
                        }
                        SpirvType::Bool => match data {
                            0 => self.constant_bool(DUMMY_SP, false),
                            1 => self.constant_bool(DUMMY_SP, true),
                            _ => self
                                .tcx
                                .sess
                                .fatal(&format!("Invalid constant value for bool: {}", data)),
                        },
                        other => self.tcx.sess.fatal(&format!(
                            "scalar_to_backend Primitive::Int not supported on type {}",
                            other.debug(ty, self)
                        )),
                    },
                    Primitive::F32 => {
                        let res = self.constant_f32(DUMMY_SP, f32::from_bits(data as u32));
                        assert_eq!(res.ty, ty);
                        res
                    }
                    Primitive::F64 => {
                        let res = self.constant_f64(DUMMY_SP, f64::from_bits(data as u64));
                        assert_eq!(res.ty, ty);
                        res
                    }
                    Primitive::Pointer => {
                        if data == 0 {
                            self.constant_null(ty)
                        } else {
                            let result = self.undef(ty);
                            // HACK(eddyb) we don't know whether this constant originated
                            // in a system crate, so it's better to always zombie.
                            self.zombie_even_in_user_code(
                                result.def_cx(self),
                                DUMMY_SP,
                                "pointer has non-null integer address",
                            );
                            result
                        }
                    }
                }
            }
            Scalar::Ptr(ptr) => {
                let (base_addr, _base_addr_space) = match self.tcx.global_alloc(ptr.alloc_id) {
                    GlobalAlloc::Memory(alloc) => {
                        let pointee = match self.lookup_type(ty) {
                            SpirvType::Pointer { pointee, .. } => pointee,
                            other => self.tcx.sess.fatal(&format!(
                                "GlobalAlloc::Memory type not implemented: {}",
                                other.debug(ty, self)
                            )),
                        };
                        let init = self.create_const_alloc(alloc, pointee);
                        let value = self.static_addr_of(init, alloc.align, None);
                        (value, AddressSpace::DATA)
                    }
                    GlobalAlloc::Function(fn_instance) => (
                        self.get_fn_addr(fn_instance.polymorphize(self.tcx)),
                        self.data_layout().instruction_address_space,
                    ),
                    GlobalAlloc::Static(def_id) => {
                        assert!(self.tcx.is_static(def_id));
                        assert!(!self.tcx.is_thread_local_static(def_id));
                        (self.get_static(def_id), AddressSpace::DATA)
                    }
                };
                let value = if ptr.offset.bytes() == 0 {
                    base_addr
                } else {
                    self.tcx
                        .sess
                        .fatal("Non-constant scalar_to_backend ptr.offset not supported")
                    // let offset = self.constant_u64(ptr.offset.bytes());
                    // self.gep(base_addr, once(offset))
                };
                if layout.value != Primitive::Pointer {
                    self.tcx
                        .sess
                        .fatal("Non-pointer-typed scalar_to_backend Scalar::Ptr not supported");
                // unsafe { llvm::LLVMConstPtrToInt(llval, llty) }
                } else {
                    match (self.lookup_type(value.ty), self.lookup_type(ty)) {
                        (
                            SpirvType::Pointer {
                                storage_class: a_space,
                                pointee: a,
                            },
                            SpirvType::Pointer {
                                storage_class: b_space,
                                pointee: b,
                            },
                        ) => {
                            if a_space != b_space {
                                // TODO: Emit the correct type that is passed into this function.
                                self.zombie_no_span(
                                    value.def_cx(self),
                                    "invalid pointer space in constant",
                                );
                            }
                            assert_ty_eq!(self, a, b);
                        }
                        _ => assert_ty_eq!(self, value.ty, ty),
                    }
                    value
                }
            }
        }
    }
    fn from_const_alloc(
        &self,
        layout: TyAndLayout<'tcx>,
        alloc: &Allocation,
        offset: Size,
    ) -> PlaceRef<'tcx, Self::Value> {
        assert_eq!(offset, Size::ZERO);
        let ty = layout.spirv_type(DUMMY_SP, self);
        let init = self.create_const_alloc(alloc, ty);
        let result = self.static_addr_of(init, alloc.align, None);
        PlaceRef::new_sized(result, layout)
    }

    fn const_ptrcast(&self, val: Self::Value, ty: Self::Type) -> Self::Value {
        if val.ty == ty {
            val
        } else {
            // constant ptrcast is not supported in spir-v
            let result = val.def_cx(self).with_type(ty);
            self.zombie_no_span(result.def_cx(self), "const_ptrcast");
            result
        }
    }
}

impl<'tcx> CodegenCx<'tcx> {
    // This function comes from https://doc.rust-lang.org/nightly/nightly-rustc/src/rustc_middle/ty/layout.rs.html,
    // and is a lambda in the `layout_raw_uncached` function in Version 1.50.0-nightly (eb4fc71dc 2020-12-17)
    pub fn primitive_to_scalar(&self, value: Primitive) -> abi::Scalar {
        let bits = value.size(self.data_layout()).bits();
        assert!(bits <= 128);
        abi::Scalar {
            value,
            valid_range: 0..=(!0 >> (128 - bits)),
        }
    }

    pub fn create_const_alloc(&self, alloc: &Allocation, ty: Word) -> SpirvValue {
        // println!(
        //     "Creating const alloc of type {} with {} bytes",
        //     self.debug_type(ty),
        //     alloc.len()
        // );
        let mut offset = Size::ZERO;
        let result = self.create_const_alloc2(alloc, &mut offset, ty);
        assert_eq!(
            offset.bytes_usize(),
            alloc.len(),
            "create_const_alloc must consume all bytes of an Allocation"
        );
        // println!("Done creating alloc of type {}", self.debug_type(ty));
        result
    }

    fn create_const_alloc2(&self, alloc: &Allocation, offset: &mut Size, ty: Word) -> SpirvValue {
        let ty_concrete = self.lookup_type(ty);
        *offset = offset.align_to(ty_concrete.alignof(self));
        // these print statements are really useful for debugging, so leave them easily available
        // println!("const at {}: {}", offset.bytes(), self.debug_type(ty));
        match ty_concrete {
            SpirvType::Void => self
                .tcx
                .sess
                .fatal("Cannot create const alloc of type void"),
            SpirvType::Bool
            | SpirvType::Integer(..)
            | SpirvType::Float(_)
            | SpirvType::Pointer { .. } => {
                let size = ty_concrete.sizeof(self).unwrap();
                let primitive = match ty_concrete {
                    SpirvType::Bool => Primitive::Int(Integer::fit_unsigned(0), false),
                    SpirvType::Integer(int_size, int_signedness) => {
                        let integer = match int_size {
                            8 => Integer::I8,
                            16 => Integer::I16,
                            32 => Integer::I32,
                            64 => Integer::I64,
                            128 => Integer::I128,
                            other => {
                                self.tcx
                                    .sess
                                    .fatal(&format!("invalid size for integer: {}", other));
                            }
                        };
                        Primitive::Int(integer, int_signedness)
                    }
                    SpirvType::Float(float_size) => match float_size {
                        32 => Primitive::F32,
                        64 => Primitive::F64,
                        other => {
                            self.tcx
                                .sess
                                .fatal(&format!("invalid size for float: {}", other));
                        }
                    },
                    SpirvType::Pointer { .. } => Primitive::Pointer,
                    unsupported_spirv_type => bug!(
                        "invalid spirv type internal to create_alloc_const2: {:?}",
                        unsupported_spirv_type
                    ),
                };
                // alloc_id is not needed by read_scalar, so we just use 0. If the context
                // refers to a pointer, read_scalar will find the the actual alloc_id. It
                // only uses the input alloc_id in the case that the scalar is uninitilized
                // as part of the error output
                // tldr, the pointer here is only needed for the offset
                let value = match alloc
                    .read_scalar(self, Pointer::new(AllocId(0), *offset), size)
                    .unwrap()
                {
                    ScalarMaybeUninit::Scalar(scalar) => {
                        self.scalar_to_backend(scalar, &self.primitive_to_scalar(primitive), ty)
                    }
                    ScalarMaybeUninit::Uninit => self.undef(ty),
                };
                *offset += size;
                value
            }
            SpirvType::Adt {
                size,
                field_types,
                field_offsets,
                ..
            } => {
                let base = *offset;
                let mut values = Vec::with_capacity(field_types.len());
                let mut occupied_spaces = Vec::with_capacity(field_types.len());
                for (&ty, &field_offset) in field_types.iter().zip(field_offsets.iter()) {
                    let total_offset_start = base + field_offset;
                    let mut total_offset_end = total_offset_start;
                    values.push(
                        self.create_const_alloc2(alloc, &mut total_offset_end, ty)
                            .def_cx(self),
                    );
                    occupied_spaces.push(total_offset_start..total_offset_end);
                }
                if let Some(size) = size {
                    *offset += size;
                } else {
                    assert_eq!(
                    offset.bytes_usize(),
                    alloc.len(),
                    "create_const_alloc must consume all bytes of an Allocation after an unsized struct"
                );
                }
                self.constant_composite(ty, values)
            }
            SpirvType::Opaque { name } => self.tcx.sess.fatal(&format!(
                "Cannot create const alloc of type opaque: {}",
                name
            )),
            SpirvType::Array { element, count } => {
                let count = self.builder.lookup_const_u64(count).unwrap() as usize;
                let values = (0..count)
                    .map(|_| {
                        self.create_const_alloc2(alloc, offset, element)
                            .def_cx(self)
                    })
                    .collect::<Vec<_>>();
                self.constant_composite(ty, values)
            }
            SpirvType::Vector { element, count } => {
                let total_size = ty_concrete
                    .sizeof(self)
                    .expect("create_const_alloc: Vectors must be sized");
                let final_offset = *offset + total_size;
                let values = (0..count)
                    .map(|_| {
                        self.create_const_alloc2(alloc, offset, element)
                            .def_cx(self)
                    })
                    .collect::<Vec<_>>();
                assert!(*offset <= final_offset);
                // Vectors sometimes have padding at the end (e.g. vec3), skip over it.
                *offset = final_offset;
                self.constant_composite(ty, values)
            }
            SpirvType::RuntimeArray { element } => {
                let mut values = Vec::new();
                while offset.bytes_usize() != alloc.len() {
                    values.push(
                        self.create_const_alloc2(alloc, offset, element)
                            .def_cx(self),
                    );
                }
                let result = self.constant_composite(ty, values);
                // TODO: Figure out how to do this. Compiling the below crashes both clspv *and* llvm-spirv:
                /*
                __constant struct A {
                    float x;
                    int y[];
                } a = {1, {2, 3, 4}};

                __kernel void foo(__global int* data, __constant int* c) {
                __constant struct A* asdf = &a;
                *data = *c + asdf->y[*c];
                }
                */
                // HACK(eddyb) we don't know whether this constant originated
                // in a system crate, so it's better to always zombie.
                self.zombie_even_in_user_code(
                    result.def_cx(self),
                    DUMMY_SP,
                    "constant runtime array value",
                );
                result
            }
            SpirvType::Function { .. } => self
                .tcx
                .sess
                .fatal("TODO: SpirvType::Function not supported yet in create_const_alloc"),
            SpirvType::Image { .. } => self.tcx.sess.fatal("Cannot create a constant image value"),
            SpirvType::Sampler => self
                .tcx
                .sess
                .fatal("Cannot create a constant sampler value"),
            SpirvType::SampledImage { .. } => self
                .tcx
                .sess
                .fatal("Cannot create a constant sampled image value"),
        }
    }
}
