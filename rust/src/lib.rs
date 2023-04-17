extern crate jni;

use jni::JNIEnv;
use jni::objects::{JObject, JString, JValue};
use jni::objects::JClass;

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Plugin_test(mut env: JNIEnv, _: JClass, jname: JString) {
    let name: String = env.get_string(&jname).expect("invalid string input").into();

    println!("Hello, {}!", name);
}

#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Plugin_onPlayerJoin(mut env: JNIEnv, _class: JClass, event: JClass) {
    println!("A player Joined!");
    let player_obj = match env.call_method(event, "getPlayer", "()Lcom/velocitypowered/api/proxy/Player;", &[]) {
        Ok(obj) => obj.l().unwrap(),
        Err(e) => {
            eprintln!("Error getting player object: {}", e);
            return;
        }
    };

    let player_name = match env.call_method(player_obj, "getUsername", "()Ljava/lang/String;", &[]) {
        Ok(name) => name.l().unwrap(),
        Err(e) => {
            eprintln!("Error getting player name: {}", e);
            return;
        }
    };

    let name: String = match env.get_string(JString::from(player_name).as_ref()) {
        Ok(s) => s.into(),
        Err(e) => {
            eprintln!("Error getting string: {}", e);
            return;
        }
    };

    println!("Player Joined: {}!", name);
}


#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Plugin_onPlayerLeave(mut env: JNIEnv, _class: JClass, event: JClass) {
    println!("A player Joined!");
    let player_obj = match env.call_method(event, "getPlayer", "()Lcom/velocitypowered/api/proxy/Player;", &[]) {
        Ok(obj) => obj.l().unwrap(),
        Err(e) => {
            eprintln!("Error getting player object: {}", e);
            return;
        }
    };

    let player_name = match env.call_method(player_obj, "getUsername", "()Ljava/lang/String;", &[]) {
        Ok(name) => name.l().unwrap(),
        Err(e) => {
            eprintln!("Error getting player name: {}", e);
            return;
        }
    };

    let name: String = match env.get_string(JString::from(player_name).as_ref()) {
        Ok(s) => s.into(),
        Err(e) => {
            eprintln!("Error getting string: {}", e);
            return;
        }
    };

    println!("Player Left: {}!", name);
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




