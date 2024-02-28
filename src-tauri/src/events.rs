use tauri::State;

use crate::settings::Settings;

pub fn on_exit(settings: State<Settings>) {
    settings.save().expect("Unable to save settings");
}
