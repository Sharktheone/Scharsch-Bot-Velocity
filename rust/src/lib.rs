extern crate jni;

use jni::JNIEnv;
use jni::objects::JString;
use jni::objects::JClass;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Plugin_test(mut env: JNIEnv, _: JClass, jname: JString) {
    let name: String = env.get_string(&jname).expect("invalid string input").into();

    println!("Hello, {}!", name);
}