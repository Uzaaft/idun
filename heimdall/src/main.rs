// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod config;

use std::{collections::HashMap, process::Command, str::FromStr};

use anyhow::Result;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

// Read config file from XDG_CONFIG_HOME. Fallback to ~/.config/heimdall/config.toml
// TODO: Add better error handling
fn read_config() -> Result<config::Config> {
    let config_path = std::env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap()))
        + "/heimdall/config.toml";
    let config_file = std::fs::read_to_string(config_path)?.to_string();
    Ok(config::Config::from_str(&config_file)?)
}

fn register_keys() -> Result<(GlobalHotKeyManager, HashMap<u32, String>)> {
    let user_config = read_config()?;
    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();
    let key_command_map = user_config
        .bindings
        .iter()
        .map(|hotkey| {
            let key: HotKey = hotkey.to_string().parse().unwrap();
            hotkeys_manager.register(key).unwrap();
            (key.id(), hotkey.command.to_string())
        })
        .collect();
    Ok((hotkeys_manager, key_command_map))
}

fn main() -> Result<()> {
    let event_loop = EventLoopBuilder::new().build();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();
    let (hotkeys_manager, key_command_map) = register_keys()?;
    event_loop.run(move |_event, _, control_flow| {
        // Just print registered hotkeys for now
        if let Ok(event) = global_hotkey_channel.try_recv() {
            println!("Hotkey pressed: {:?}", event);
            println!("Hotkey command: {:?}", key_command_map.get(&event.id));
            // Run the command in the shell
            Command::new("sh")
                .arg("-c")
                .arg(key_command_map.get(&event.id).unwrap())
                .spawn()
                .unwrap();
        }
    });
    Ok(())
}
