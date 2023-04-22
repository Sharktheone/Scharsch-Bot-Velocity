extern crate jni;
use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};
use jni::objects::JClass;

mod jni_utils;
mod config;
mod websocket;
mod events;

use crate::jni_utils::{call_stacking, convert_string, JniFn, JSTRING};
use crate::config::load::load_config;
use crate::websocket::websocket::connect_ws;

//      de.scharschbot.velocity.plugin.Events
// Java_de_scharschbot_velocity_plugin_Events_onInitialize
#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onInitialize(env: JNIEnv, class: JClass) {
    println!("Loading Config!");
    let config = match load_config(){
        Ok(config) => config,
        Err(err) => {
            println!("Error loading config: {}", err);
            return;
        }
    };
    connect_ws(env, &class, config).expect("Error connecting to websocket");
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerJoin(env: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(env, event);

    println!("Player Joined: {}!", name);
}


#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerLeave(env: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(env, event);

    println!("Player Left: {} :(", name);
}


#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerChat(env: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(env, event);

    println!("Player Chat: {}!", name);
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onProxyShutdown(env: JNIEnv, _class: JClass, event: JObject) {
    // TODO: Close websocket
}

fn extract_player<'a, 'b>(mut env: JNIEnv, event: JObject) -> String {
    let fns = [
        JniFn {
            name: "getPlayer",
            input: &[],
            output: "Lcom.velocitypowered.api.proxy.Player;",
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




