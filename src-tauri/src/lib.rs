mod commands;
mod models;
mod parser;
mod math;

use commands::evaluate::evaluate_expression;
use commands::convert::convert_units;
use commands::history::{get_history, clear_history};
use commands::memory::{
    memory_store, memory_recall, memory_clear,
    memory_add, memory_subtract,
    set_angle_mode, set_precision,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            evaluate_expression,
            convert_units,
            get_history,
            clear_history,
            memory_store,
            memory_recall,
            memory_clear,
            memory_add,
            memory_subtract,
            set_angle_mode,
            set_precision,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
