// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod config;

use std::str::FromStr;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

// Read config file from XDG_CONFIG_HOME. Fallback to ~/.config/heimdall/config.toml
fn read_config() -> config::Config {
    // This can probably be done better
    let config_path = std::env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap()))
        + "/heimdall/config.toml";
    let config_file = std::fs::read_to_string(config_path).unwrap_or(
        // Just a default config for now
        r#"
        [[bindings]]
        key = "C"
        modifiers = ["Ctrl", "Shift"]
        command = "echo hello"
        [[bindings]]
        key = "D"
        modifiers = ["Ctrl"]
        command = "echo hello"
        "#
        .to_string(),
    );
    config::Config::from_str(&config_file).unwrap()
}

fn main() {
    let event_loop = EventLoopBuilder::new().build();
    let config = read_config();

    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        // Just print registered hotkeys for now
        if let Ok(event) = global_hotkey_channel.try_recv() {
            println!("{event:?}");
        }
    })
}
