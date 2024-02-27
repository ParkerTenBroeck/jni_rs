use std::{marker::PhantomData, ops::{Deref, DerefMut}};
use super::*;

#[repr(u32)]
pub enum ArrReleaseMode{
    Reg = 0,
    Commit = 1,
    Abort = 2,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct JArray<T, Ref: RefTypeT = UnknownRef>{
    obj: JR<Object, Ref>,
    _type: PhantomData<T>,
}

unsafe impl<T: JavaType, Ref: RefTypeT> JavaType for JArray<T, Ref>{
    fn signature() -> std::borrow::Cow<'static, str> {
        let mut sig = T::signature();
        sig.to_mut().insert(0, '[');
        sig
    }
}
unsafe impl<T: JavaType, Ref: RefTypeT> JavaObject for JArray<T, Ref>{}


impl<T, Ref: RefTypeT> JArray<T, Ref> {
    pub unsafe fn ref_cast(&self) -> JArray<T> {
        JArray { obj: self.obj.unchecked_ref_cast(), _type: PhantomData }
    }

    pub unsafe fn type_cast<N>(&self) -> JArray<N, Ref>{
        JArray { obj: self.obj.unchecked_ref_cast(), _type: PhantomData }
    }

    pub fn len(&self, env: &JNIEnv) -> jni_size{
        (env.methods.get_array_length)(env, JArray { obj: unsafe { self.obj.unchecked_ref_cast() } , _type: PhantomData })
    }
}



#[derive(Debug)]
pub struct JArraySlice<'a, T, Ref: RefTypeT, IsMut>{
    slice: &'a mut [T],
    copied: bool,
    obj: &'a JArray<T, Ref>,
    env: &'a JNIEnv,
    release: unsafe extern "C" fn(&JNIEnv, JArray<T>, *mut T, ArrReleaseMode),
    _mut: PhantomData<IsMut>
}

impl<'a, T, Ref: RefTypeT, IsMut> JArraySlice<'a, T, Ref, IsMut>{
    pub fn is_copy(&self) -> bool{
        self.copied
    }

    pub unsafe fn release(&self, action: ArrReleaseMode){
        (self.release)(self.env, self.obj.ref_cast(), self.slice.as_ptr().cast_mut(), action)
    }
}

impl<'a, T, Ref: RefTypeT, IsMut> Deref for JArraySlice<'a, T, Ref, IsMut>{
    type Target = [T];

    fn deref(&self) -> &Self::Target {    
        self.slice
    }
}

impl<'a, T, Ref: RefTypeT> DerefMut for JArraySlice<'a, T, Ref, &'a mut JArray<T, Ref>>{
    fn deref_mut(&mut self) -> &mut Self::Target {    
        self.slice
    }
}

impl<'a, T, Ref: RefTypeT, IsMut> Drop for JArraySlice<'a, T, Ref, IsMut>{
    fn drop(&mut self) {
        unsafe{
            self.release(ArrReleaseMode::Reg)
        }
    }
}

impl<'a, T, Ref: RefTypeT> JArraySlice<'a, T, Ref, &'a mut JArray<T, Ref>>{
    pub fn abort(self){
        unsafe{
            self.release(ArrReleaseMode::Abort);
        }
        std::mem::forget(self);
    }

    pub fn commit(&self){
        unsafe{
            self.release(ArrReleaseMode::Commit);
        }
    }
}

macro_rules! impl_arr {

    ($type:ty, $new:ident) => {
        impl<Ref: RefTypeT> JArray<$type, Ref>{
            pub fn new(env: &$crate::jni::JNIEnv, len: $crate::types::jni_size) -> Option<JArray<$type>>{
                (env.methods.$new)(env, len)
            }
        }
    };
    ($type:ty, $new:ident, $get:ident, $release:ident) => {
        impl_arr!($type, $new);
        impl<Ref: RefTypeT> JArray<$type, Ref>{

            pub fn slice<'a>(&'a self, env: &'a $crate::jni::JNIEnv) -> JArraySlice<'a, $type, Ref, &'a Self>{
                let mut copied: jni_bool = false;
                let len = self.len(env);
                let slice = unsafe{
                    let ptr = (env.methods.$get)(env, self.ref_cast(), (&mut copied) as *mut jni_bool);
                    std::slice::from_raw_parts_mut(ptr, len as usize)
                };
                JArraySlice{
                    slice,
                    copied,
                    obj: self,
                    env: env,
                    release: env.methods.$release,
                    _mut: PhantomData
                }
            }

            pub fn slice_mut<'a>(&'a mut self, env: &'a $crate::jni::JNIEnv) -> JArraySlice<'a, $type, Ref, &'a mut Self>{
                let mut copied: jni_bool = false;
                let len = self.len(env);
                let slice = unsafe{
                    let ptr = (env.methods.$get)(env, self.ref_cast(), (&mut copied) as *mut jni_bool);
                    std::slice::from_raw_parts_mut(ptr, len as usize)
                };
                JArraySlice{
                    slice,
                    copied,
                    obj: self,
                    env: env,
                    release: env.methods.$release,
                    _mut: PhantomData
                }
            }


            pub unsafe fn slice_critical<'a>(&'a self, env: &'a $crate::jni::JNIEnv) -> JArraySlice<'a, $type, Ref, &'a Self>{
                let mut copied: jni_bool = false;
                let len = self.len(env);
                let slice = unsafe{
                    let ptr = (env.methods.get_primitive_array_critical)(env, self.ref_cast().type_cast(), (&mut copied) as *mut jni_bool);
                    std::slice::from_raw_parts_mut(ptr.cast(), len as usize)
                };
                JArraySlice{
                    slice,
                    copied,
                    obj: self,
                    env: env,
                    release: unsafe { std::mem::transmute(env.methods.release_primitive_array_critical) },
                    _mut: PhantomData
                }
            }

            pub unsafe fn slice_mut_critical<'a>(&'a mut self, env: &'a $crate::jni::JNIEnv) -> JArraySlice<'a, $type, Ref, &'a mut Self>{
                let mut copied: jni_bool = false;
                let len = self.len(env);
                let slice = unsafe{
                    let ptr = (env.methods.get_primitive_array_critical)(env, self.ref_cast().type_cast(), (&mut copied) as *mut jni_bool);
                    std::slice::from_raw_parts_mut(ptr.cast(), len as usize)
                };
                JArraySlice{
                    slice,
                    copied,
                    obj: self,
                    env: env,
                    release: unsafe { std::mem::transmute(env.methods.release_primitive_array_critical) },
                    _mut: PhantomData
                }
            }
        }
    };
}

impl_arr!(Option<JR<Object>>, new_object_array);
impl_arr!(jni_bool, new_boolean_array, get_boolean_array_elements, release_boolean_array_elements);
impl_arr!(jni_long, new_long_array, get_long_array_elements, release_long_array_elements);
impl_arr!(jni_int, new_int_array, get_int_array_elements, release_int_array_elements);
impl_arr!(jni_short, new_short_array, get_short_array_elements, release_short_array_elements);
impl_arr!(jni_byte, new_byte_array, get_byte_array_elements, release_byte_array_elements);
impl_arr!(jni_char, new_char_array, get_char_array_elements, release_char_array_elements);
impl_arr!(jni_float, new_float_array, get_float_array_elements, release_float_array_elements);
impl_arr!(jni_double, new_double_array, get_double_array_elements, release_double_array_elements);