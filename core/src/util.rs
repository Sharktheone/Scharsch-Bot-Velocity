use std::ops::Deref;
use jni::JNIEnv;
use jni::objects::{JObject};
use scharschbot_core::jni_utils::{call_stacking, convert_string, JniFn, JSTRING};

pub(crate) fn extract_player(mut env: &mut JNIEnv, event: JObject) -> String {
    let fns = [
        JniFn {
            name: "getPlayer",
            input: &[],
            output: "com.velocitypowered.api.proxy.Player",
            args: &[],
        },
        JniFn {
            name: "getUsername",
            input: &[],
            output: JSTRING,
            args: &[],
        }
    ];
    let player_obj = call_stacking(&mut env, event, &fns);

    convert_string(&mut env, player_obj)
}

pub(crate) fn extract_player_server<'a, 'b>(mut env: &mut JNIEnv, event: JObject) -> String {
    let fns = [
        JniFn {
            name: "getPlayer",
            input: &[],
            output: "com.velocitypowered.api.proxy.Player",
            args: &[],
        },
        JniFn {
            name: "getCurrentServer",
            input: &[],
            output: "java.util.Optional", //  public abstract java.util.Optional<com.velocitypowered.api.proxy.ServerConnection> getCurrentServer()
            args: &[],
        },
        JniFn {
            name: "toString",
            input: &[],
            output: JSTRING,
            args: &[],
        }
    ];

    let server_obj = call_stacking(&mut env, event, &fns);

    convert_string(&mut env, server_obj)
}

pub(crate) fn extract_message(mut env: &mut JNIEnv, event: JObject) -> String {
    let fns = [
        JniFn {
            name: "getMessage",
            input: &[],
            output: JSTRING,
            args: &[],
        }
    ];
    let message_obj = call_stacking(&mut env, event, &fns);

    convert_string(&mut env, message_obj)
}

pub(crate) fn extract(mut env: &mut JNIEnv, event: JObject) -> (String, String) {
    let name = extract_player(&mut env, unsafe { JObject::from_raw(event.as_ref().deref().clone())}); // TODO: Find better way of cloning JObject
    let server = extract_player_server(&mut env, event);
    (name, server)
}