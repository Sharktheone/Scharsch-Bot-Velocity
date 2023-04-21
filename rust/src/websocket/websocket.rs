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

fn get_ws<'a>(env: &mut JNIEnv<'a>, class:&JObject) ->std::result::Result<*const WSClient<'a>, String>{
    match env.get_field(class, "ws", "J") {
        Ok(ptr_val) => {
            match ptr_val.j(){
                Ok(ptr_j) => {
                    if ptr_j == 0 {
                        Err("No ws pointer stored".to_string())
                    } else {
                        let ptr:*const WSClient = ptr_j as *const WSClient;
                        Ok(ptr as *const WSClient)
                    }
                }
                Err(err) => Err(format!("Error getting ws pointer: {}", err))
            }
        },
        Err(err) => Err(format!("Error getting ws pointer: {}", err)),
    }
}