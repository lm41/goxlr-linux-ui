#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod client;

struct Client {

}

#[tauri::command]
async fn test_command(param: String) -> Result<String, String> {
    println!("{}", param);
    //Err("Failed".into())
    Ok("Hello From Rust!".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
