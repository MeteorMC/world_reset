mod api_connect;
mod config_load;

use api_connect::{signal::server_signal, status::server_status, delete::server_files_delete, webhook::discord_webhook};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    config_load::load_env_file();
    let server_status = server_status()?;
    if server_status == "running" {
        server_signal("stop".to_string());
        sleep(Duration::from_secs(5));
    }
    let file_res = server_files_delete("/".to_string(), Some(&["world".to_string(), "mohist-config/worlds.yml".to_string()]));
    if file_res.is_ok() {
        server_signal("start".to_string());
        discord_webhook("ワールドリセット完了".to_string(), "ワールドリセットが完了しました\n少し待ってから参加してください".to_string(), true);
    } else {
        discord_webhook("ワールドリセットに失敗".to_string(), "ワールドリセットに失敗しました".to_string(), false);
        panic!();
    }
    Ok(())
}
