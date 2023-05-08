// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::{CustomMenuItem, Menu, Submenu};


//There are No Hotkeys because googling suggests there's a webview issue in Windows and I ran into it too when I attempted it(https://github.com/tauri-apps/wry/issues/451)
fn main() {
    tauri::Builder::default()
    .menu(Menu::new().add_submenu(Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("new", "New"))
            .add_item(CustomMenuItem::new("open", "Open"))
            .add_item(CustomMenuItem::new("save", "Save"))
            .add_item(CustomMenuItem::new("close", "Close"))
    )))
    .on_menu_event(|event| match event.menu_item_id() {
        "new" => {
            let _ = event.window().emit("new-event", "new-event").unwrap();//These send events to listeners in Editor.Svelte
        }

        "open" => {
            let _ = event.window().emit("open-event", "open-event").unwrap();

        }

        "save" => {
            let _ = event.window().emit("save-event", "save-event").unwrap();
        }

        
        "close" => {
            std::process::exit(0);
        }
        _ => {}

    })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
