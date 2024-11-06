#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod docs {
        #[allow(dead_code)]
        pub mod operations {
            #[allow(dead_code, clippy::all)]
            pub mod operands {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_add_cabi<T: Guest>(arg0: i32, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::add(arg0 as u32, arg1 as u32);
                    _rt::as_i32(result0)
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_mul_cabi<T: Guest>(arg0: i32, arg1: i32) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::mul(arg0 as u32, arg1 as u32);
                    _rt::as_i32(result0)
                }
                pub trait Guest {
                    fn add(a: u32, b: u32) -> u32;
                    fn mul(a: u32, b: u32) -> u32;
                }
                #[doc(hidden)]
                macro_rules! __export_docs_operations_operands_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "docs:operations/operands@0.1.0#add"] unsafe extern "C" fn
                        export_add(arg0 : i32, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_add_cabi::<$ty > (arg0, arg1) } #[export_name =
                        "docs:operations/operands@0.1.0#mul"] unsafe extern "C" fn
                        export_mul(arg0 : i32, arg1 : i32,) -> i32 { $($path_to_types)*::
                        _export_mul_cabi::<$ty > (arg0, arg1) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_docs_operations_operands_0_1_0_cabi;
            }
        }
    }
}
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_operation_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::docs::operations::operands::__export_docs_operations_operands_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::docs::operations::operands);
    };
}
#[doc(inline)]
pub(crate) use __export_operation_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:docs:operations@0.1.0:operation:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 234] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07k\x01A\x02\x01A\x02\x01\
B\x03\x01@\x02\x01ay\x01by\0y\x04\0\x03add\x01\0\x04\0\x03mul\x01\0\x04\x01\x1ed\
ocs:operations/operands@0.1.0\x05\0\x04\x01\x1fdocs:operations/operation@0.1.0\x04\
\0\x0b\x0f\x01\0\x09operation\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0d\
wit-component\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}