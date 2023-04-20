use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use ws::{connect, listen, Handler, Sender, Result, Message as WSMessage, Handshake, CloseCode};


pub struct WSClient<'a> {
    sender: Sender,
    env:&'a mut JNIEnv<'a>,
    class:&'a JObject<'a>,
}

impl <'a> Handler for WSClient<'a> {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let client:*const WSClient = self;
        let client_pointer = client as i64;
        if let Err(err) = store_ws(&mut self.env, self.class, client_pointer) {
            println!("Error storing ws pointer: {}", err);
        }
        Ok(())
    }

    fn on_message(&mut self, msg: WSMessage) -> Result<()> {
        println!("Got message: {}", msg);
        self.sender.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Connection closed due to ({:?}) {}", code, reason);
    }
}

fn store_ws(env: &mut JNIEnv<'_>, class:&JObject, ptr:i64) ->std::result::Result<(),jni::errors::Error>{
    env.set_field(class, "ws", "J", JValue::Long(ptr))
}