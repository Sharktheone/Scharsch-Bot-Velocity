use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};


#[allow(unused)]
pub(crate) const JVOID: &str = "V";

#[allow(unused)]
pub(crate) const JBOOLEAN: &str = "Z";

#[allow(unused)]
pub(crate) const JBYTE: &str = "B";

#[allow(unused)]
pub(crate) const JCHAR: &str = "C";

#[allow(unused)]
pub(crate) const JSHORT: &str = "S";

#[allow(unused)]
pub(crate) const JINT: &str = "I";

#[allow(unused)]
pub(crate) const JLONG: &str = "J";

#[allow(unused)]
pub(crate) const JFLOAT: &str = "F";

#[allow(unused)]
pub(crate) const JDOUBLE: &str = "D";

#[allow(unused)]
pub(crate) const JSTRING: &str = "Ljava/lang/String;";

#[allow(unused)]

pub struct JniFn<'a> {
    pub(crate) name: &'a str,
    pub(crate) input: &'a [String],
    pub(crate) output: &'a str,
    pub(crate) args: &'a [JValue<'a, 'a>],
}

fn assemble_signature(input: &[String], output: &String) -> String {
    let mut signature = String::from("(");
    for i in input {
        signature.push_str(make_signature(i).as_str());
    }
    signature.push_str(")");
    signature.push_str(make_signature(output).as_str());
    return signature;
}

fn make_signature(sig: &String) -> String {
    let mut sig = sig.replace(".", "/");

    if sig.contains("/") {
        if !sig.starts_with("L") {
            sig = format!("L{}", sig);
        }
        if !sig.ends_with(";") {
            sig = format!("{};", sig);
        }
    }

    return sig
}

pub fn call_stacking<'a, 'b>(env: &mut JNIEnv<'a>, obj: JObject<'b>, jfn: &[JniFn<'a>]) -> JObject<'a> {
    let mut obj = obj;
    for f in jfn {
        let signature = assemble_signature(f.input, &f.output.to_string());
        obj = match env.call_method(obj, &f.name, signature, f.args) {
            Ok(name) => name.l().unwrap(),
            Err(e) => {
                eprintln!("Error calling jni method {}: {}", f.name, e);
                return JObject::null();
            }
        };
    }
    return unsafe { JObject::from_raw(obj.as_raw()) }
}
pub fn convert_string(env: &mut JNIEnv, obj: JObject) -> String {
    match env.get_string(JString::from(obj).as_ref()) {
        Ok(s) => s.into(),
        Err(e) => {
            eprintln!("Error getting string: {}", e);
            return String::from("");
        }
    }
}