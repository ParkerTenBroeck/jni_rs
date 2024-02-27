use std::{borrow::Cow, ffi::CStr};

pub mod array;
pub mod object;
pub mod primitive;


use primitive::*;
use object::*;

use crate::jni::JNIEnv;

pub union JValue{
    pub z: jni_bool,
    pub b: jni_byte,
    pub c: jni_char,
    pub s: jni_short,
    pub i: jni_int,
    pub l: jni_long,
    pub f: jni_float,
    pub d: jni_double,
    pub o: Option<JR<Object>>,
}

pub unsafe trait JavaType{
    fn signature() -> Cow<'static, str>;
}
pub unsafe trait JavaObject: JavaType where Self: Sized{
    fn get_class(env: &JNIEnv) -> Option<JR<JClass<Self>>>{
        let mut class = Self::signature();
        class.to_mut().push('\0');
        env.find_class(CStr::from_bytes_with_nul(class.as_bytes()).unwrap()).map(|v|unsafe{v.unchecked_cast()})
    }
}


pub struct UnknownType;