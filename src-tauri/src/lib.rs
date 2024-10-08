use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {

            use tauri_plugin_notification::NotificationExt;


            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let open_i = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&quit_i, &open_i])?;

            let tray = TrayIconBuilder::new()
                .menu(&menu)
                .menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0)
                    }

                    "open" => {
                        println!("open menu item was clicked");

                        app.notification()
                            .builder()
                            .title("Tauri")
                            .body("Tauri is awesome")
                            .show()
                            .unwrap();

                        let webview_window = tauri::WebviewWindowBuilder::new(
                            app,
                            "external", /* the unique window label */
                            tauri::WebviewUrl::External("https://tauri.app/".parse().unwrap())
                        )
                        .build();
                    }

                 
                    
    
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
