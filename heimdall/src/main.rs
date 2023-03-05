// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod config;

use config::Config;
use std::{collections::HashMap, process::Command};

use anyhow::Result;
use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager};
use winit::event_loop::EventLoopBuilder;

fn register_keys() -> Result<HashMap<u32, String>> {
    let user_config = Config::read_config()?;
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
    Ok(key_command_map)
}

fn main() -> Result<()> {
    let event_loop = EventLoopBuilder::new().build();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();
    let key_command_map = register_keys()?;
    event_loop.run(move |_event, _, control_flow| {
        control_flow.set_wait();

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
    })
}
