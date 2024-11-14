#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod rust_comp {
            #[allow(dead_code, clippy::all)]
            pub mod rust_example_comp {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
                pub enum Operation {
                    Add,
                    Sub,
                    Mul,
                    Pow,
                    Div,
                }
                impl ::core::fmt::Debug for Operation {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            Operation::Add => f.debug_tuple("Operation::Add").finish(),
                            Operation::Sub => f.debug_tuple("Operation::Sub").finish(),
                            Operation::Mul => f.debug_tuple("Operation::Mul").finish(),
                            Operation::Pow => f.debug_tuple("Operation::Pow").finish(),
                            Operation::Div => f.debug_tuple("Operation::Div").finish(),
                        }
                    }
                }
                impl Operation {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> Operation {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => Operation::Add,
                            1 => Operation::Sub,
                            2 => Operation::Mul,
                            3 => Operation::Pow,
                            4 => Operation::Div,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct Expression {
                    pub first: u32,
                    pub operation: Operation,
                    pub second: u32,
                }
                impl ::core::fmt::Debug for Expression {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Expression")
                            .field("first", &self.first)
                            .field("operation", &self.operation)
                            .field("second", &self.second)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_eval_cabi<T: Guest>(
                    arg0: i32,
                    arg1: i32,
                    arg2: i32,
                ) -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::eval(Expression {
                        first: arg0 as u32,
                        operation: Operation::_lift(arg1 as u8),
                        second: arg2 as u32,
                    });
                    _rt::as_i32(result0)
                }
                pub trait Guest {
                    fn eval(expr: Expression) -> u32;
                }
                #[doc(hidden)]
                macro_rules! __export_component_rust_comp_rust_example_comp_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:rust-comp/rust-example-comp#eval"] unsafe extern "C"
                        fn export_eval(arg0 : i32, arg1 : i32, arg2 : i32,) -> i32 {
                        $($path_to_types)*:: _export_eval_cabi::<$ty > (arg0, arg1, arg2)
                        } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_rust_comp_rust_example_comp_cabi;
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
macro_rules! __export_rust_calculator_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::rust_comp::rust_example_comp::__export_component_rust_comp_rust_example_comp_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::component::rust_comp::rust_example_comp);
    };
}
#[doc(inline)]
pub(crate) use __export_rust_calculator_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:rust-comp:rust-calculator:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 328] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xc2\x01\x01A\x02\x01\
A\x02\x01B\x06\x01m\x05\x03add\x03sub\x03mul\x03pow\x03div\x04\0\x09operation\x03\
\0\0\x01r\x03\x05firsty\x09operation\x01\x06secondy\x04\0\x0aexpression\x03\0\x02\
\x01@\x01\x04expr\x03\0y\x04\0\x04eval\x01\x04\x04\x01%component:rust-comp/rust-\
example-comp\x05\0\x04\x01#component:rust-comp/rust-calculator\x04\0\x0b\x15\x01\
\0\x0frust-calculator\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-com\
ponent\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
