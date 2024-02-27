use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};


use super::*;

#[macro_export]
macro_rules! make_class {
    ($type:ident $(<$( $lt:ident ),+>)?, $path:literal) => {
        unsafe impl$(<$( $lt ),+>)? $crate::types::JavaType for $type $(<$( $lt ),+>)?{
            fn signature() -> std::borrow::Cow<'static, str>{
                const LEN: usize = $path.len()+2;
                const ARR: [u8; LEN] = {
                    let mut arr: [u8; LEN] = [0; LEN];
                    let mut base: usize = 0;
                    
                    arr[0] = b'L';
                    base += 1;

                    while base - 1 < $path.len() {
                        let mut c = $path.as_bytes()[base-1];
                        if c == b'.'{
                            c = b'/';
                        }else {
                            match c{
                                b'a'..=b'z'|b'A'..=b'Z'|b'0'..=b'9'|b'_' => {}
                                _ => panic!("Invalid Path character"),
                            }
                        }
                        arr[base] = c;
                        base += 1;
                    }

                    arr[base] = b';';
                    base += 1;
                    if base != LEN { panic!("invalid length"); }
                    arr
                };
                const VAL: &'static str = unsafe { core::str::from_utf8_unchecked(&ARR) };
                std::borrow::Cow::Borrowed(VAL)
            }
        }       
        unsafe impl$(<$( $lt ),+>)? $crate::types::JavaObject for $type $(<$( $lt ),+>)?{}
    };
}



#[derive(Debug, Clone, Copy)]
pub struct JString;
make_class!(JString, "java.lang.String");

#[derive(Debug, Clone, Copy)]
pub struct JClass<Class = UnknownType> {
    _p: PhantomData<Class>
}
make_class!(JClass<Class>, "java.lang.Class");

impl<Class> JR<JClass<Class>>{
    pub fn type_erasure(self) -> JR<JClass<UnknownType>>{
        unsafe { self.unchecked_cast() }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JThrowable;
make_class!(JThrowable, "java.lang.Throwable");



#[derive(Debug, Clone, Copy)]
pub struct UnknownRef;
#[derive(Debug, Clone, Copy)]
pub struct LocalRef;
#[derive(Debug, Clone, Copy)]
pub struct GlobalRef;
#[derive(Debug, Clone, Copy)]
pub struct WeakGlobalRef;
pub trait RefTypeT{}
impl RefTypeT for UnknownRef{}
impl RefTypeT for LocalRef{}
impl RefTypeT for GlobalRef{}
impl RefTypeT for WeakGlobalRef{}


#[derive(Debug, Clone, Copy)]
pub struct Object;
make_class!(Object, "java.lang.Object");

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct JR<Type: JavaObject, Ref: RefTypeT = UnknownRef>{
    _pt: NonNull<c_void>,
    _p: PhantomData<(Type, Ref)>
}
impl<Type: JavaObject, Ref: RefTypeT> JR<Type, Ref>{
    pub fn obj(self) -> JR<Object, Ref>{
        JR { _pt: self._pt, _p: PhantomData }
    }

    pub unsafe fn unchecked_cast<NType: JavaObject, NRef: RefTypeT>(self) -> JR<NType, NRef>{
        JR { _pt: self._pt, _p: PhantomData }
    }

    pub unsafe fn unchecked_type_cast<NType: JavaObject>(self) -> JR<NType, Ref>{
        JR { _pt: self._pt, _p: PhantomData }
    }

    pub unsafe fn unchecked_ref_cast<NRef: RefTypeT>(&self) -> JR<Type, NRef> {
        JR { _pt: self._pt, _p: PhantomData }
    }
}

unsafe impl<Type: JavaObject, Ref: RefTypeT> JavaType for JR<Type, Ref>{
    fn signature() -> std::borrow::Cow<'static, str> {
        Type::signature()
    }
}
unsafe impl<Type: JavaObject, Ref: RefTypeT> JavaObject for JR<Type, Ref>{}

unsafe impl<Type: JavaObject, Ref: RefTypeT> JavaType for Option<JR<Type, Ref>>{
    fn signature() -> std::borrow::Cow<'static, str> {
        Type::signature()
    }
}
unsafe impl<Type: JavaObject, Ref: RefTypeT> JavaObject for Option<JR<Type, Ref>>{}

#[repr(u32)]
pub enum RefType{
    Invalid = 0,
    Local = 1,
    Global = 2,
    WeakGlobal = 3,
}