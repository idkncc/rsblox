use tauri::{api, AppHandle, Manager, Runtime};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrayGame {
    pub id: u64,
    pub title: String,
}
pub fn get_system_tray(games: Vec<TrayGame>) -> SystemTray {
    let mut tray_menu = SystemTrayMenu::new();

    for game in games.iter() {
        let game_item = CustomMenuItem::new(format!("game-{}", game.id), game.title.to_string());
        tray_menu = tray_menu.add_item(game_item);
    }

    if games.len() > 0 {
        tray_menu = tray_menu.add_native_item(SystemTrayMenuItem::Separator)
    }

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    tray_menu = tray_menu.add_item(quit);

    SystemTray::new()
        .with_menu(tray_menu)
        .with_title("rsblox")
        .with_tooltip("replacement of roblox website")
}

pub fn on_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            if id == "quit" {
                std::process::exit(0);
            } else if id.starts_with("game-") {
                let id: u64 = (&id[5..]).parse().unwrap();

                api::shell::open(
                    &app.shell_scope(),
                    format!("roblox://experiences/start?placeId={}", id),
                    None,
                )
                .unwrap()
            }
        }
        _ => {}
    }
}
