use jni::objects::{JObject};
use scharschbot_core::jni_utils::{call_stacking, convert_string, JniFn, JSTRING};

pub(crate) fn extract_player(event: &JObject) -> String {
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
    let player_obj = call_stacking(event, &fns);

    convert_string(&player_obj)
}

pub(crate) fn extract_player_server(event: &JObject) -> String {
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
            output: "java.util.Optional",
            args: &[],
        },
        JniFn {
            name: "toString",
            input: &[],
            output: JSTRING,
            args: &[],
        }
    ];

    let server_obj = call_stacking(event, &fns);

    convert_string(&server_obj)
}

pub(crate) fn extract_message(event: JObject) -> String {
    let fns = [
        JniFn {
            name: "getMessage",
            input: &[],
            output: JSTRING,
            args: &[],
        }
    ];
    let message_obj = call_stacking(&event, &fns);

    convert_string(&message_obj)
}

pub(crate) fn extract(event: &JObject) -> (String, String) {
    let name = extract_player(event);
    let server = extract_player_server(event);
    (name, server)
}