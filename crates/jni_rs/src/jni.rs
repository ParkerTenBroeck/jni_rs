use std::{ffi::{c_char, CStr}, num::NonZeroI32, os::raw::c_void};


use crate::reflect::{field::JfieldID, method::JmethodID};

use self::array::{ArrReleaseMode, JArray};

use super::types::primitive::*;
use super::types::object::*;
use super::types::*;


#[repr(transparent)]
#[derive(Debug)]
pub struct JErr(NonZeroI32);

#[derive(Debug)]
#[must_use]
pub enum JResult{
    Ok,
    Err(JErr)
}

#[repr(C)]
pub struct JavaVM{

}

#[repr(C)]
#[derive(Debug)]
pub struct JNIEnv{
   pub methods: &'static JNIEnvMethods,
}


#[repr(C)]
#[derive(Debug)]
pub struct JNIEnvMethods{
    pub _unused1: *mut (),
    pub _unused2: *mut (),
    pub _unused3: *mut (),
    pub _unused4: *mut (),
    pub get_version: extern "C" fn(&JNIEnv) -> jni_int,
    
    pub define_class: unsafe extern "C" fn(&JNIEnv, name: *const c_char, loader: JR<Object>, class_buf: *const i8, buf_len: jni_size) -> JR<JClass>,
    pub find_class: unsafe extern "C" fn(&JNIEnv, name: *const c_char) -> Option<JR<JClass>>,
    
    pub from_reflection_method: unsafe extern "C" fn(&JNIEnv, JR<Object>) -> Option<JmethodID>,
    pub from_reflection_field: unsafe extern "C" fn(&JNIEnv, JR<Object>) -> Option<JfieldID>,
    pub to_reflection_method: extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, is_static: jni_bool) -> Option<JR<Object>>,

    pub get_superclass: extern "C" fn(&JNIEnv, JR<JClass>) -> Option<JR<JClass>>,
    pub is_assignable_from: extern "C" fn(&JNIEnv, from: JR<JClass>, to: JR<JClass>) -> jni_bool,

    pub to_reflection_field: extern "C" fn(&JNIEnv, JR<JClass>, JfieldID, is_static: jni_bool) -> Option<JR<Object>>,

    pub throw: extern "C" fn(&JNIEnv, JR<JThrowable>) -> JResult,
    pub throw_new: unsafe extern "C" fn(&JNIEnv, JR<JClass>, message: *const c_char) -> JResult,
    pub exception_occured: extern "C" fn(&JNIEnv) -> Option<JR<JThrowable>>,
    pub exception_describe: extern "C" fn(&JNIEnv),
    pub exception_clear: extern "C" fn(&JNIEnv),
    pub fatal_error: unsafe extern "C" fn(&JNIEnv, *const c_char),

    pub push_local_frame: unsafe extern "C" fn(&JNIEnv, jni_int) -> JResult,
    pub pop_local_frame: unsafe extern "C" fn(&JNIEnv, JR<Object>) -> JR<Object>, //no idea how this shit works

    pub new_global_ref: extern "C" fn(&JNIEnv, JR<Object>) -> JR<Object, GlobalRef>,
    pub delete_global_ref: extern "C" fn(&JNIEnv, JR<Object, GlobalRef>),
    pub delete_local_ref: extern "C" fn(&JNIEnv, JR<Object, LocalRef>),
    pub is_same_object: extern "C" fn(&JNIEnv, JR<Object>, JR<Object>) -> jni_bool,
    pub new_local_ref: extern "C" fn(&JNIEnv, JR<Object>) -> JR<Object, LocalRef>,
    pub ensure_local_capacity: extern "C" fn(&JNIEnv, capacity: jni_int) -> JResult,

    pub alloc_object: unsafe extern "C" fn(&JNIEnv, JR<JClass>) -> Option<JR<Object>>,
    pub new_object: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, ...) -> Option<JR<Object>>,
    pub new_object_v: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, *const JValue) -> Option<JR<Object>>,
    pub new_object_a: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, std::ffi::VaList) -> Option<JR<Object>>,

    pub get_object_class: extern "C" fn(&JNIEnv, JR<Object>) -> JR<JClass>,
    pub is_instance_of: extern "C" fn(&JNIEnv, JR<Object>, JR<Object>) -> jni_bool,

    pub get_method_id: extern "C" fn(&JNIEnv, JR<JClass>, name: *const c_char, sig: *const c_char) -> Option<JmethodID>,
    
    pub call_object_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, ...) -> Option<JR<Object>>,
    pub call_object_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, *const JValue) -> Option<JR<Object>>,
    pub call_object_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, std::ffi::VaList) -> Option<JR<Object>>,
    pub call_boolean_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, ...) -> jni_bool,
    pub call_boolean_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, *const JValue) -> jni_bool,
    pub call_boolean_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, std::ffi::VaList) -> jni_bool,
    pub call_byte_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, ...) -> jni_byte,
    pub call_byte_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, *const JValue) -> jni_byte,
    pub call_byte_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, std::ffi::VaList) -> jni_byte,
    pub call_char_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, ...) -> jni_char,
    pub call_char_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, *const JValue) -> jni_char,
    pub call_char_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, std::ffi::VaList) -> jni_char,
    pub call_short_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, ...) -> jni_short,
    pub call_short_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, *const JValue) -> jni_short,
    pub call_short_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, std::ffi::VaList) -> jni_short,
    pub call_int_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, ...) -> jni_int,
    pub call_int_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, *const JValue) -> jni_int,
    pub call_int_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, std::ffi::VaList) -> jni_int,
    pub call_long_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, ...) -> jni_long,
    pub call_long_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, *const JValue) -> jni_long,
    pub call_long_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, std::ffi::VaList) -> jni_long,
    pub call_float_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, ...) -> jni_float,
    pub call_float_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, *const JValue) -> jni_float,
    pub call_float_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, std::ffi::VaList) -> jni_float,
    pub call_double_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, ...) -> jni_double,
    pub call_double_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, *const JValue) -> jni_double,
    pub call_double_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, std::ffi::VaList) -> jni_double,
    pub call_void_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, ...),
    pub call_void_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, *const JValue),
    pub call_void_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JmethodID, std::ffi::VaList),
    
    pub call_non_virtual_object_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, ...) -> Option<JR<Object>>,
    pub call_non_virtual_object_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, *const JValue) -> Option<JR<Object>>,
    pub call_non_virtual_object_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, std::ffi::VaList) -> Option<JR<Object>>,
    pub call_non_virtual_boolean_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, ...) -> jni_bool,
    pub call_non_virtual_boolean_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, *const JValue) -> jni_bool,
    pub call_non_virtual_boolean_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_bool,
    pub call_non_virtual_byte_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, ...) -> jni_byte,
    pub call_non_virtual_byte_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, *const JValue) -> jni_byte,
    pub call_non_virtual_byte_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_byte,
    pub call_non_virtual_char_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, ...) -> jni_char,
    pub call_non_virtual_char_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, *const JValue) -> jni_char,
    pub call_non_virtual_char_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_char,
    pub call_non_virtual_short_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, ...) -> jni_short,
    pub call_non_virtual_short_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, *const JValue) -> jni_short,
    pub call_non_virtual_short_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_short,
    pub call_non_virtual_int_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, ...) -> jni_int,
    pub call_non_virtual_int_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, *const JValue) -> jni_int,
    pub call_non_virtual_int_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_int,
    pub call_non_virtual_long_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, ...) -> jni_long,
    pub call_non_virtual_long_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, *const JValue) -> jni_long,
    pub call_non_virtual_long_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_long,
    pub call_non_virtual_float_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, ...) -> jni_float,
    pub call_non_virtual_float_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, *const JValue) -> jni_float,
    pub call_non_virtual_float_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_float,
    pub call_non_virtual_double_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, ...) -> jni_double,
    pub call_non_virtual_double_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, *const JValue) -> jni_double,
    pub call_non_virtual_double_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_double,
    pub call_non_virtual_void_method: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, ...),
    pub call_non_virtual_void_method_v: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, *const JValue),
    pub call_non_virtual_void_method_a: unsafe extern "C" fn(&JNIEnv, JR<Object>, JR<JClass>, JmethodID, std::ffi::VaList),

    pub get_field_id: extern "C" fn(&JNIEnv, JR<JClass>, name: *const c_char, sig: *const c_char) -> Option<JfieldID>,

    pub get_object_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID) -> Option<JR<Object, GlobalRef>>,
    pub get_boolean_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID) -> jni_bool,
    pub get_byte_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID) -> jni_byte,
    pub get_char_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID) -> jni_char,
    pub get_short_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID) -> jni_short,
    pub get_int_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID) -> jni_int,
    pub get_long_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID) -> jni_long,
    pub get_float_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID) -> jni_float,
    pub get_double_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID) -> jni_double,

    pub set_object_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID, Option<JR<Object, GlobalRef>>),
    pub set_boolean_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID, jni_bool),
    pub set_byte_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID, jni_byte),
    pub set_char_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID, jni_char),
    pub set_short_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID, jni_short),
    pub set_int_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID, jni_int),
    pub set_long_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID, jni_long),
    pub set_float_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID, jni_float),
    pub set_double_field: unsafe extern "C" fn(&JNIEnv, JR<Object>, JfieldID, jni_double),

    pub get_static_method_id: extern "C" fn(&JNIEnv, JR<JClass>, name: *const c_char, sig: *const c_char) -> Option<JmethodID>,

    pub call_static_object_method: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, ...) -> Option<JR<Object>>,
    pub call_static_object_method_v: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, *const JValue) -> Option<JR<Object>>,
    pub call_static_object_method_a: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, std::ffi::VaList) -> Option<JR<Object>>,
    pub call_static_boolean_method: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, ...) -> jni_bool,
    pub call_static_boolean_method_v: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, *const JValue) -> jni_bool,
    pub call_static_boolean_method_a: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_bool,
    pub call_static_byte_method: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, ...) -> jni_byte,
    pub call_static_byte_method_v: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, *const JValue) -> jni_byte,
    pub call_static_byte_method_a: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_byte,
    pub call_static_char_method: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, ...) -> jni_char,
    pub call_static_char_method_v: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, *const JValue) -> jni_char,
    pub call_static_char_method_a: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_char,
    pub call_static_short_method: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, ...) -> jni_short,
    pub call_static_short_method_v: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, *const JValue) -> jni_short,
    pub call_static_short_method_a: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_short,
    pub call_static_int_method: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, ...) -> jni_int,
    pub call_static_int_method_v: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, *const JValue) -> jni_int,
    pub call_static_int_method_a: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_int,
    pub call_static_long_method: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, ...) -> jni_long,
    pub call_static_long_method_v: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, *const JValue) -> jni_long,
    pub call_static_long_method_a: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_long,
    pub call_static_float_method: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, ...) -> jni_float,
    pub call_static_float_method_v: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, *const JValue) -> jni_float,
    pub call_static_float_method_a: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_float,
    pub call_static_double_method: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, ...) -> jni_double,
    pub call_static_double_method_v: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, *const JValue) -> jni_double,
    pub call_static_double_method_a: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, std::ffi::VaList) -> jni_double,
    pub call_static_void_method: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, ...),
    pub call_static_void_method_v: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, *const JValue),
    pub call_static_void_method_a: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JmethodID, std::ffi::VaList),

    pub get_static_field_id: extern "C" fn(&JNIEnv, JR<JClass>, name: *const c_char, sig: *const c_char) -> Option<JfieldID>,

    pub get_static_object_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID) -> Option<JR<Object>>,
    pub get_static_boolean_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID) -> jni_bool,
    pub get_static_byte_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID) -> jni_byte,
    pub get_static_char_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID) -> jni_char,
    pub get_static_short_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID) -> jni_short,
    pub get_static_int_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID) -> jni_int,
    pub get_static_long_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID) -> jni_long,
    pub get_static_float_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID) -> jni_float,
    pub get_static_double_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID) -> jni_double,

    pub set_static_object_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID, Option<JR<Object>>),
    pub set_static_boolean_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID, jni_bool),
    pub set_static_byte_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID, jni_byte),
    pub set_static_char_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID, jni_char),
    pub set_static_short_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID, jni_short),
    pub set_static_int_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID, jni_int),
    pub set_static_long_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID, jni_long),
    pub set_static_float_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID, jni_float),
    pub set_static_double_field: unsafe extern "C" fn(&JNIEnv, JR<JClass>, JfieldID, jni_double),

    pub new_string: unsafe extern "C" fn(&JNIEnv, unicode: *const jni_char, jni_size),

    pub get_string_length: unsafe extern "C" fn(&JNIEnv, JR<JString>) -> jni_size,
    pub get_string_chars: unsafe extern "C" fn(&JNIEnv, JR<JString>, is_copy: *mut jni_bool) -> *const jni_char,
    pub release_string_chars: unsafe extern "C" fn(&JNIEnv, JR<JString>, *const jni_char),

    pub new_string_utf: unsafe extern "C" fn(&JNIEnv, *const i8) -> Option<JR<JString>>,
    pub get_string_utf_length: unsafe extern "C" fn(&JNIEnv, JR<JString>) -> jni_size,
    pub get_string_utf_chars: unsafe extern "C" fn(&JNIEnv, JR<JString>, is_copy: *mut jni_bool) -> *const i8,
    pub release_string_utf_chars: unsafe extern "C" fn(&JNIEnv, JR<JString>, *const i8),

    pub get_array_length: extern "C" fn(&JNIEnv, JArray<()>) -> jni_size,

    pub new_object_array: extern "C" fn(&JNIEnv, jni_size) -> Option<JArray<Option<JR<Object>>>>,
    pub get_object_array_element: unsafe extern "C" fn(&JNIEnv, JArray<Option<JR<Object>>>, jni_size) -> Option<JR<Object>>,
    pub set_object_array_element: unsafe extern "C" fn(&JNIEnv, JArray<Option<JR<Object>>>, jni_size, Option<JR<Object>>),

    pub new_boolean_array: extern "C" fn(&JNIEnv, jni_size) -> Option<JArray<jni_bool>>,
    pub new_byte_array: extern "C" fn(&JNIEnv, jni_size) -> Option<JArray<jni_byte>>,
    pub new_char_array: extern "C" fn(&JNIEnv, jni_size) -> Option<JArray<jni_char>>,
    pub new_short_array: extern "C" fn(&JNIEnv, jni_size) -> Option<JArray<jni_short>>,
    pub new_int_array: extern "C" fn(&JNIEnv, jni_size) -> Option<JArray<jni_int>>,
    pub new_long_array: extern "C" fn(&JNIEnv, jni_size) -> Option<JArray<jni_long>>,
    pub new_float_array: extern "C" fn(&JNIEnv, jni_size) -> Option<JArray<jni_float>>,
    pub new_double_array: extern "C" fn(&JNIEnv, jni_size) -> Option<JArray<jni_double>>,

    pub get_boolean_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_bool>, is_copy: *mut jni_bool) -> *mut jni_bool,
    pub get_byte_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_byte>, is_copy: *mut jni_bool) -> *mut jni_byte,
    pub get_char_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_char>, is_copy: *mut jni_bool) -> *mut jni_char,
    pub get_short_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_short>, is_copy: *mut jni_bool) -> *mut jni_short,
    pub get_int_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_int>, is_copy: *mut jni_bool) -> *mut jni_int,
    pub get_long_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_long>, is_copy: *mut jni_bool) -> *mut jni_long,
    pub get_float_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_float>, is_copy: *mut jni_bool) -> *mut jni_float,
    pub get_double_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_double>, is_copy: *mut jni_bool) -> *mut jni_double,

    pub release_boolean_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_bool>, *mut jni_bool, ArrReleaseMode),
    pub release_byte_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_byte>, *mut jni_byte, ArrReleaseMode),
    pub release_char_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_char>, *mut jni_char, ArrReleaseMode),
    pub release_short_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_short>, *mut jni_short, ArrReleaseMode),
    pub release_int_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_int>, *mut jni_int, ArrReleaseMode),
    pub release_long_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_long>, *mut jni_long, ArrReleaseMode),
    pub release_float_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_float>, *mut jni_float, ArrReleaseMode),
    pub release_double_array_elements: unsafe extern "C" fn(&JNIEnv, JArray<jni_double>, *mut jni_double, ArrReleaseMode),

    pub get_boolean_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_bool>, start: jni_size, len: jni_size, *mut jni_bool),
    pub get_byte_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_byte>, start: jni_size, len: jni_size, *mut jni_byte),
    pub get_char_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_char>, start: jni_size, len: jni_size, *mut jni_char),
    pub get_short_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_short>, start: jni_size, len: jni_size, *mut jni_short),
    pub get_int_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_int>, start: jni_size, len: jni_size, *mut jni_int),
    pub get_long_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_long>, start: jni_size, len: jni_size, *mut jni_long),
    pub get_float_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_float>, start: jni_size, len: jni_size, *mut jni_float),
    pub get_double_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_double>, start: jni_size, len: jni_size, *mut jni_double),

    pub set_boolean_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_bool>, start: jni_size, len: jni_size, *const jni_bool),
    pub set_byte_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_byte>, start: jni_size, len: jni_size, *const jni_byte),
    pub set_char_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_char>, start: jni_size, len: jni_size, *const jni_char),
    pub set_short_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_short>, start: jni_size, len: jni_size, *const jni_short),
    pub set_int_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_int>, start: jni_size, len: jni_size, *const jni_int),
    pub set_long_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_long>, start: jni_size, len: jni_size, *const jni_long),
    pub set_float_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_float>, start: jni_size, len: jni_size, *const jni_float),
    pub set_double_array_region: unsafe extern "C" fn(&JNIEnv, JArray<jni_double>, start: jni_size, len: jni_size, *const jni_double),

    pub register_natives: unsafe extern "C" fn(&JNIEnv), //TODO
    pub unregister_natives: unsafe extern "C" fn(&JNIEnv), //TODO

    pub monitor_enter: extern "C" fn(&JNIEnv, JR<Object>) -> JResult,
    pub monitor_exit: unsafe extern "C" fn(&JNIEnv, JR<Object>) -> JResult,

    pub get_java_vm: unsafe extern "C" fn(&JNIEnv, *mut *mut JavaVM) -> JResult,

    pub get_string_region: unsafe extern "C" fn(&JNIEnv), //TODO
    pub get_string_uft_region: unsafe extern "C" fn(&JNIEnv), //TODO

    pub get_primitive_array_critical: unsafe extern "C" fn(&JNIEnv, JArray<c_void>, is_copy: *mut jni_bool) -> *mut c_void,
    pub release_primitive_array_critical: unsafe extern "C" fn(&JNIEnv, JArray<c_void>, *mut c_void, ArrReleaseMode),

    pub get_string_critical: unsafe extern "C" fn(&JNIEnv, JR<JString>, is_copy: *mut jni_bool) -> *const jni_char,
    pub release_string_critical: unsafe extern "C" fn(&JNIEnv, JR<JString>, *const jni_char),

    pub new_weak_global_ref: extern "C" fn(&JNIEnv, JR<Object>) -> JR<Object, WeakGlobalRef>,
    pub delete_weak_global_ref: extern "C" fn(&JNIEnv, JR<Object, WeakGlobalRef>),

    pub exception_check: extern "C" fn(&JNIEnv) -> jni_bool,

    pub new_direct_byte_buffer: unsafe extern "C" fn(&JNIEnv, start: *mut c_void, cap: jni_long) -> Option<JR<Object>>,
    pub get_direct_buffer_address: unsafe extern "C" fn(&JNIEnv, JR<Object>) -> *mut c_void,
    pub get_direct_buffer_capacity: unsafe extern "C" fn(&JNIEnv, JR<Object>) -> jni_long,

    pub get_object_ref_type: extern "C" fn(&JNIEnv, JR<Object>) -> RefType
}

impl JNIEnv{
    pub fn get_version(&self) -> Result<[jni_short; 2], JErr>{
        let ret = (self.methods.get_version)(self);
        
        if ret >= 0{
            Ok([(ret >> 16) as i16, ret as i16])
        }else{
            Err(JErr(NonZeroI32::new(ret).unwrap()))
        }
    
    }

    pub fn find_class(&self, msg: &CStr) -> Option<JR<JClass>>{
        unsafe { (self.methods.find_class)(self, msg.as_ptr()) }
    }

    pub fn fatal_error(&self, msg: &CStr){
        unsafe{ (self.methods.fatal_error)(self, msg.as_ptr()) }
    }
}

impl JNIEnv{
}

impl JNIEnv{
    pub fn throw(&self, throwable: JR<JThrowable>) -> JResult{
        (self.methods.throw)(self, throwable)
    }

    pub fn throw_new(&self, class: JR<JClass>, message: &CStr) -> JResult{
        unsafe { (self.methods.throw_new)(self, class, message.as_ptr()) }
    }
}

impl JNIEnv{
    pub fn get_method_id(&self, class: JR<JClass>, name: &CStr, sig: &CStr) -> Option<JmethodID>{
        (self.methods.get_method_id)(self, class, name.as_ptr(), sig.as_ptr())
    }
}

impl JNIEnv{
    pub unsafe fn alloc_object(&self, class: JR<JClass>) -> Option<JR<Object>>{
        unsafe{ (self.methods.alloc_object)(self, class) }
    }

    // pub unsafe extern "C" fn new_object(&self, class: JR<JClass>, constructor: JmethodID, v: ...) -> Option<JR<Object>>{
    //         // std::ops::Fn::call(|| unsafe{self.methods.new_object}, (args))
    //         (self.methods.new_object)(self, class, constructor, v)
    // }
}