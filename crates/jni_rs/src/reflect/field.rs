use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};

use crate::types::UnknownType;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct JfieldID<ClassType = UnknownType, FieldType = UnknownType>{
    _f: NonNull<c_void>,
    _t: PhantomData<(ClassType, FieldType)>
}