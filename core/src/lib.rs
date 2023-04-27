extern crate jni;

mod logger;

use std::ops::Deref;
use jni::JNIEnv;
use jni::objects::{JObject};
use jni::objects::JClass;

use scharschbot_core::jni_utils::{call_stacking, convert_string, JniFn, JSTRING};
use scharschbot_core::config::load::load_config;
use scharschbot_core::websocket::websocket::connect_ws;
use scharschbot_core::plugin::logger::{info, error, info_no_env};
use scharschbot_core::events::mc_events::{player_join, player_leave, player_chat};

//      de.scharschbot.velocity.plugin.Events
// Java_de_scharschbot_velocity_plugin_Events_onInitialize
#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onInitialize(mut env: JNIEnv, class: JClass) {
    logger::set(&mut env, &class);
    info(&mut env, &class, format!("Loading Config!"));
    let config = match load_config(){
        Ok(config) => {
            config
        },
        Err(err) => {
            error(&mut env, &class, format!("Error loading config: {}", err));
            return;
        }
    };

    info(&mut env, &class, "Connecting to websocket!".to_string());

    match connect_ws(&mut env, &class, config){
        Ok(_) => info(&mut env, &class, format!("Connected to websocket!")),
        Err(err) => error(&mut env, &class, format!("Error connecting to websocket: {}", err)),
    };
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerJoin(mut env: JNIEnv, class: JClass, event: JObject) {
    let (name, server) = extract(&mut env, event);
    player_join(&mut env, &class, name, server);
}


#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerLeave(mut env: JNIEnv, class: JClass, event: JObject) {
    let (name, server) = extract(&mut env, event);
    player_leave(&mut env, &class, name, server);
}


#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerChat(mut env: JNIEnv, class: JClass, event: JObject) {
    let (name, server) = extract(&mut env, unsafe { JObject::from_raw(event.as_ref().deref().clone())}); // TODO: Find better way of cloning JObject
    let message = extract_message(&mut env, event);
    player_chat(&mut env, &class, name, message, server);
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onProxyShutdown(_env: JNIEnv, _class: JClass, _event: JObject) {
    // TODO: Close websocket
}

fn extract_player(mut env: &mut JNIEnv, event: JObject) -> String {
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

fn extract_player_server<'a, 'b>(mut env: &mut JNIEnv, event: JObject) -> String {
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

fn extract_message(mut env: &mut JNIEnv, event: JObject) -> String {
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

fn extract(mut env: &mut JNIEnv, event: JObject) -> (String, String) {
    let name = extract_player(&mut env, unsafe { JObject::from_raw(event.as_ref().deref().clone())}); // TODO: Find better way of cloning JObject
    let server = extract_player_server(&mut env, event);
    (name, server)
}


// TODO: Config Loader

// TODO: PlayerJoinEvent
// TODO: PlayerQuitEvent
// TODO: PlayerChatEvent

// TODO: WS EVENTS:


// TODO: AUTH: auth

// TODO: SEND: sendPlayers
// TODO: SEND: playerJoined
// TODO: SEND: playerLeft
// TODO: SEND: chatMessage
// TODO: SEND: reportPlayer
// TODO: SEND: banPlayer
// TODO: SEND: unbanPlayer
// TODO: SEND: kickPlayer
// (TODO: SEND: playerAdvancement)
// (TODO: SEND: playerDeath)


// TODO: RECEIVE: sendChatMessage
// TODO: RECEIVE: sendPlayers
// TODO: RECEIVE: kickPlayer
// TODO: RECEIVE: banPlayer
// TODO: RECEIVE: unbanPlayer
// TODO: RECEIVE: sendCommand




