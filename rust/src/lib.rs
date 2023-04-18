extern crate jni;

use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::objects::JClass;
use std::fs;

fn load_config() {
    let paths = fs::read_dir("plugins/scharschbot/").unwrap();
    for path in paths {
        println!("Path: {:?}", path);
    }
}

//      de.scharschbot.velocity.plugin.Events
// Java_de_scharschbot_velocity_plugin_Events_onInitialize
#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onInitialize(mut _env: JNIEnv, _class: JClass) {
    println!("Loading Config!");
    load_config()
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerJoin(mut env: JNIEnv, _class: JClass, event: JObject) {
       let name = extract_player(env, event);

    println!("Player Joined: {}!", name);
}


#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Events_onPlayerLeave(mut env: JNIEnv, _class: JClass, event: JObject) {
    let name = extract_player(env, event);

    println!("Player Left: {}!", name);
}

fn extract_player(mut env: JNIEnv, event: JObject) -> String {
    let player_obj = match env.call_method(event, "getPlayer", "()Lcom/velocitypowered/api/proxy/Player;", &[]) {
        Ok(obj) => obj.l().unwrap(),
        Err(e) => {
            eprintln!("Error getting player object: {}", e);
            return String::from("");
        }
    };

    let player_name = match env.call_method(player_obj, "getUsername", "()Ljava/lang/String;", &[]) {
        Ok(name) => name.l().unwrap(),
        Err(e) => {
            eprintln!("Error getting player name: {}", e);
            return String::from("");
        }
    };

    match env.get_string(JString::from(player_name).as_ref()) {
        Ok(s) => s.into(),
        Err(e) => {
            eprintln!("Error getting string: {}", e);
            return String::from("");
        }
    }
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




