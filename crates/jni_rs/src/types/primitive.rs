#[allow(non_camel_case_types)]
pub type jni_void = core::ffi::c_void;
/// idk if this is UB or not but we will find out evetually
#[allow(non_camel_case_types)]
pub type jni_bool = bool;
#[allow(non_camel_case_types)]
pub type jni_long = i64;
#[allow(non_camel_case_types)]
pub type jni_int = i32;
#[allow(non_camel_case_types)]
pub type jni_short = i16;
#[allow(non_camel_case_types)]
pub type jni_byte = i8;
#[allow(non_camel_case_types)]
pub type jni_char = u16;
#[allow(non_camel_case_types)]
pub type jni_float = f32;
#[allow(non_camel_case_types)]
pub type jni_double = f64;

#[allow(non_camel_case_types)]
pub type jni_size = jni_int;
use std::borrow::Cow;

macro_rules! primitive_impl {
    ($type:ty, $sig:literal) => {
        unsafe impl super::JavaType for $type{
            fn signature() -> Cow<'static, str>{
                Cow::Borrowed($sig)
            }
        }
    };
}
primitive_impl!(jni_bool, "Z");
primitive_impl!(jni_byte, "B");
primitive_impl!(jni_char, "C");
primitive_impl!(jni_short, "S");
primitive_impl!(jni_int, "I");
primitive_impl!(jni_long, "J");
primitive_impl!(jni_float, "F");
primitive_impl!(jni_double, "D");
primitive_impl!(jni_void, "V");