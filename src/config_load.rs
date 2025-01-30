use std::fs;
use std::env;

pub fn load_env_file() {
    let contents = fs::read_to_string(".env").expect(".envファイルが見つかりません");
    for line in contents.lines() {
        if let Some((key, value)) = line.split_once('=') {
            env::set_var(key.trim(), value.trim());
        }
    }
}
