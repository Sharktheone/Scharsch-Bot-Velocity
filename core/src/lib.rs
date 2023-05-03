extern crate jni;

mod logger;
mod util;

use jni::JNIEnv;
use jni::objects::{JObject};
use jni::objects::JClass;
use scharschbot_core::config::load::load_config;
use scharschbot_core::websocket::websocket::connect_ws;
use scharschbot_core::plugin::logger::{info, error, error_no_env};
use scharschbot_core::events::mc_events::{player_join, player_leave, player_chat};
use scharschbot_core::jni_utils::set_vm;

use crate::util::{extract, extract_message};

pub static mut CLASS: Option<JClass<'static>> = None;

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onInitialize(env: JNIEnv<'static>, class: JClass<'static>) {
    let vm = match env.get_java_vm() {
        Ok(vm) => vm,
        Err(err) => {
            error_no_env(format!("Error getting java vm: {}", err));
            return;
        }
    };
    set_vm(vm);

    CLASS = Some(class);

    logger::set();
    info(format!("Loading Config!"));
    match load_config() {
        Ok(config) => {
            config
        }
        Err(err) => {
            error(format!("Error loading config: {}", err));
            return;
        }
    };

    info("Connecting to websocket!".to_string());

    match connect_ws() {
        Ok(_) => info(format!("Closed websocket")),
        Err(err) => error(format!("Error connecting to websocket: {}", err)),
    };
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerJoin(_: JNIEnv, _class: JClass, event: JObject) {
    let (name, server) = extract(&event);
    player_join(name, server);
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerLeave(_: JNIEnv, _class: JClass, event: JObject) {
    let (name, server) = extract(&event);
    player_leave(name, server);
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerChat(_: JNIEnv, _: JClass, event: JObject) {
    let (name, server) = extract(&event);
    let message = extract_message(event);
    player_chat(name, message, server);
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onProxyShutdown(_env: JNIEnv, _class: JClass, _event: JObject) {
    // TODO: Close websocket
}




