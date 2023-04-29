extern crate jni;

mod logger;
mod util;

use std::ops::Deref;
use jni::JNIEnv;
use jni::objects::{JObject};
use jni::objects::JClass;
use scharschbot_core::config::load::load_config;
use scharschbot_core::websocket::websocket::connect_ws;
use scharschbot_core::plugin::logger::{info, error};
use scharschbot_core::events::mc_events::{player_join, player_leave, player_chat};
use scharschbot_core::{set_class};

use crate::util::{extract, extract_message};
//      de.scharschbot.velocity.plugin.Events
// Java_de_scharschbot_velocity_plugin_Events_onInitialize
#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onInitialize(mut env: JNIEnv<'static>, class: JClass<'static>) {
    set_class(class);
    logger::set();
    info(&mut env, format!("Loading Config!"));
    let config = match load_config(){
        Ok(config) => {
            config
        },
        Err(err) => {
            error(&mut env,format!("Error loading config: {}", err));
            return;
        }
    };

    info(&mut env,"Connecting to websocket!".to_string());

    match connect_ws(config){
        Ok(_) => info(&mut env,format!("Connected to websocket!")),
        Err(err) => error(&mut env,format!("Error connecting to websocket: {}", err)),
    };
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerJoin(mut env: JNIEnv<'static>, _class: JClass<'static>, event: JObject) {
    let (name, server) = extract(&mut env, event);
    player_join(name, server);
}


#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerLeave(mut env: JNIEnv<'static>, _class: JClass<'static>, event: JObject) {
    let (name, server) = extract(&mut env, event);
    player_leave(name, server);
}


#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerChat(mut env: JNIEnv<'static>, _class: JClass<'static>, event: JObject) {
    let (name, server) = extract(&mut env, unsafe { JObject::from_raw(event.as_ref().deref().clone())}); // TODO: Find better way of cloning JObject
    let message = extract_message(&mut env, event);
    player_chat(name, message, server);
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onProxyShutdown(_env: JNIEnv, _class: JClass, _event: JObject) {
    // TODO: Close websocket
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




