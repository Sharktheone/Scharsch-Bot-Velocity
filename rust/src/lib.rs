extern crate jni;

use jni::JNIEnv;
use jni::objects::JString;
use jni::objects::JClass;

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Java_de_scharschbot_velocity_plugin_Plugin_test(mut env: JNIEnv, _: JClass, jname: JString) {
    let name: String = env.get_string(&jname).expect("invalid string input").into();

    println!("Hello, {}!", name);
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




