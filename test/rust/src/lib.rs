// jmacros::make_me_a_binding!(
//     public class Main{
//         public static void main(String... args);
//         public static long printHellow();
//     }
// );

use jni_rs::{jni::JavaVM, types::primitive};




#[export_name = "JNI_OnLoad"]
pub extern "C" fn jni_onload(_vm: *mut JavaVM, _reserved: *mut ()) -> primitive::jni_int{
    println!("hello from rust");
    10<<16
}

// #[derive(Clone, Copy, Debug)]
// pub struct Main;
// make_class!(Main, "Main");

// #[export_name = "Java_Main_testNew"]
// pub extern "C" fn test_new(env: &jni::JNIEnv) -> Option<JR<Main>>{
//     Constructor::<_, ()>::new_object(env)
// }



// #[export_name = "Java_Main_test33"]
// pub extern "C" fn test33(env: &jni::JNIEnv) -> Option<JObject>{
//     let class = env.find_class(c"Main")?;
//     let main = unsafe{
//         env.get_method_id(class, c"<init>", c"()V")?.cast::<JClass, Constructor, (jni_int, jni_long)>()
//     }
//     .new_object(env, class, 12, 33);
//     // concat!(bruh::thing, "asd");
//     // let thuing = constcat::concat!("(", <test as bruh>::thing, ")V");
    
//     // let instance = env.new_class();
//     main
// }


// #[export_name = "Java_Main_testMethodId"]
// pub extern "C" fn test_method_id(env: &jni::JNIEnv, _obj: JR<Object>, mut arr: JArray<jni_int>) -> JArray<jni_short>{
//     let mut slice = arr.slice_mut(env);
//     let mut ret = JArray::<jni_short>::new(env, slice.len() as i32).unwrap();
//     let mut new_slice = ret.slice_mut(env);

//     for (i, item) in slice.iter_mut().enumerate(){
//         new_slice[i] = *item as i16;
//         *item *= 2;
//     }

//     drop(new_slice);

//     ret
// }

// #[export_name = "Java_Main_bruh"]
// pub extern "C" fn test(env: &jni::JNIEnv, obj: jni::objects::JObject, thing: *mut [*mut ();6]) -> jni::primitives::jni_long{
//     // println!("{:?}", )
//     // println!("{:?}, {:?}, {:?}", env, obj, thing);
//     // println!("{:?}", unsafe{thing.read()});
//     println!("version: {:?}", env.get_version());
//     33
// }

// #[export_name = "Java_Main_throwMe"]
// pub extern "C" fn throw_test(env: &jni::JNIEnv, _obj: jni::objects::JObject, obj: jni::objects::JThrowable){
//     _ = dbg!(env.throw_new(env.find_class(c"java/lang/Throwable").unwrap(), c"bruh"));
// }

// #[export_name = "Java_Main_cret"]
// pub extern "C" fn test4(env: &jni::JNIEnv, _obj: jni::objects::JObject) -> Option<jni::objects::JClass>{
//     // dbg!(env.find_class(c"asdasd"));
//     dbg!(env.find_class(c"Main"))
// }

// #[export_name = "Java_Main_bruh2"]
// pub extern "C" fn test2(env: &jni::JNIEnv, _obj: jni::objects::JObject){
//     let _class = env.find_class(c"asd");
//     env.fatal_error(c"FATIAL ERROR");
// }