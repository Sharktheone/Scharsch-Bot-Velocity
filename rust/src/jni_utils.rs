use jni::JNIEnv;
use jni::objects::{JObject, JValue};


#[allow(unused)]
const JVOID: &str = "V";

#[allow(unused)]
const JBOOLEAN: &str = "Z";

#[allow(unused)]
const JBYTE: &str = "B";

#[allow(unused)]
const JCHAR: &str = "C";

#[allow(unused)]
const JSHORT: &str = "S";

#[allow(unused)]
const JINT: &str = "I";

#[allow(unused)]
const JLONG: &str = "J";

#[allow(unused)]
const JFLOAT: &str = "F";

#[allow(unused)]
const JDOUBLE: &str = "D";

#[allow(unused)]
const JSTRING: &str = "Ljava/lang/String;";

#[allow(unused)]

pub struct JniFn<'a> {
    pub(crate) name: String,
    pub(crate) input: &'a [String],
    pub(crate) output: String,
    pub(crate) args: &'a [JValue<'a, 'a>],
}

fn assemble_signature(input: &[String], output: &String) -> String {
    let mut signature = String::from("(");
    for i in input {
        signature.push_str(i);
    }
    signature.push_str(")");
    signature.push_str(output);
    return signature;
}


pub fn call_stacking<'a>(env: &mut JNIEnv<'a>, obj: JObject<'a>, jfn: &[JniFn<'a>]) -> JObject<'a> {
    let mut obj = obj;
    for f in jfn {
        let signature = assemble_signature(f.input, &f.output);
        obj = match env.call_method(obj, &f.name, signature, f.args) {
            Ok(name) => name.l().unwrap(),
            Err(e) => {
                eprintln!("Error calling jni method {}: {}", f.name, e);
                return JObject::null();
            }
        };
    }
    return obj
}