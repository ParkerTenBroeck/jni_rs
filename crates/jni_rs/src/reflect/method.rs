use std::{ffi::*, marker::PhantomData, ptr::NonNull};

use crate::types::*;
use crate::jni::JNIEnv;
use crate::reflect::method::object::*;
use crate::reflect::method::primitive::*;

pub struct MethodSignature<Ret, Args>{
    _p: PhantomData<(Ret, Args)>
}

pub struct ConstructorMethod;
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct JmethodID<Type = UnknownType, Ret = UnknownType, Args = UnknownType>{
    _p: NonNull<c_void>,
    _ph: PhantomData<(Type, Ret, Args)>
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Constructor<Type, Args>{
    _ph: PhantomData<(Type, Args)>
}

impl<Class, Ret, Args> JmethodID<Class, Ret, Args>{
    pub unsafe fn cast<NT, NRet, NArgs>(&self) -> JmethodID<NT, NRet, NArgs>{
        JmethodID { _p: self._p, _ph: PhantomData }
    }
}

macro_rules! impl_apply {
    // Empty case
    () => {
        impl_apply!(+);
    };
    ($first_generic:ident $($other_generics:ident)*) => {
        impl_apply!($($other_generics)*);
        impl_apply!(+$first_generic $($other_generics)*);
    };
    (+$($other_generics:ident)*) => {

        impl<T: JavaType, $($other_generics: JavaType,)*> MethodSignature<T, ($($other_generics,)*)>{
            pub fn signature() -> String{
                let mut sig = String::new();
                sig.push('(');
                $(sig.push_str(&$other_generics::signature());)*
                sig.push(')');
                sig.push_str(&T::signature());
                sig
            }
        }
        
        impl<Type: JavaObject + Copy, $($other_generics: JavaType,)*> Constructor<Type, ($($other_generics,)*)>{
            #[allow(non_snake_case)]
            pub fn new_object(
                env: &JNIEnv,
                $($other_generics: $other_generics,)*
            ) -> Option<JR<Type>>{
                let class = Type::get_class(env)?;
                JmethodID::<_, _, ($($other_generics,)*)>::get_constructor(env, class)?.new_object(env, class, $($other_generics,)*)
            }
        }
        
        impl<Type: JavaObject, $($other_generics: JavaType,)*> JmethodID<Type, ConstructorMethod, ($($other_generics,)*)>{
            #[allow(non_snake_case)]
            pub fn new_object(
                &self, env: &JNIEnv, class: JR<JClass<Type>>,
                $($other_generics: $other_generics,)*
            ) -> Option<JR<Type>> {
                unsafe{
                    (env.methods.new_object)(env, class.type_erasure(), self.cast(), $($other_generics,)*).map(|v|v.unchecked_cast())
                }
            }

            pub fn get_constructor(env: &JNIEnv, class: JR<JClass<Type>>) -> Option<Self>{
                let mut sig = MethodSignature::<jni_void, ($($other_generics,)*)>::signature();
                sig.push('\0');
                let sig = CStr::from_bytes_with_nul(sig.as_bytes()).unwrap();
                env.get_method_id(class.type_erasure(), c"<init>", sig).map(|v|unsafe{v.cast()})
            }
        }
    };
}

impl_apply!(A B C D E F G H I J K L M);