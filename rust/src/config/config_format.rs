use serde::{Deserialize, Serialize};


//https://users.rust-lang.org/t/concatenate-const-strings/51712
macro_rules! combine {
    ($A:expr, $B:expr) => {{
        const A: &str = $A;
        const B: &str = $B;
        const LEN: usize = A.len() + B.len();
        const fn combined() -> [u8; LEN] {
            let mut out = [0u8; LEN];
            out = copy_slice(A.as_bytes(), out, 0);
            out = copy_slice(B.as_bytes(), out, A.len());
            out
        }
        const fn copy_slice(input: &[u8], mut output: [u8; LEN], offset: usize) -> [u8; LEN] {
            let mut index = 0;
            loop {
                output[offset+index] = input[index];
                index += 1;
                if index == input.len() { break }
            }
            output
        }
        const RESULT: &[u8] = &combined();
        // how bad is the assumption that `&str` and `&[u8]` have the same layout?
        const RESULT_STR: &str = unsafe { std::mem::transmute(RESULT) };
        RESULT_STR
    }}
}


pub const PLUGIN_ROOT_DIR: &str = "plugins";
pub const PLUGIN_NAME: &str = "scharschbot";

pub const PLUGIN_CONFIG_DIR: &str = combine!(
    PLUGIN_ROOT_DIR,
    combine!(
    "/",
    PLUGIN_NAME
    )
);

pub const CONFIG_FILE: &str = "config.json";

pub const CONFIG_PATH: &str = combine!(
    PLUGIN_CONFIG_DIR,
    combine!(
    "/",
    CONFIG_FILE
    )
);


#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    host: String,
    port: u16,
    user: String,
    password: String,
    server_name: String,
    #[serde(flatten)]
    server_name_overrides: ServerNameOverride,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerNameOverride {
    server_name: String,
    server_name_override: String,
}