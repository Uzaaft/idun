// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod config;

use config::Config;
use std::{collections::HashMap, process::Command};

use anyhow::Result;
use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn register_keys(hotkeys_manager: GlobalHotKeyManager) -> Result<HashMap<u32, String>> {
    let user_config = Config::read_config()?;
    let key_command_map = user_config
        .bindings
        .iter()
        .map(|hotkey| {
            let key: HotKey = hotkey.to_string().parse().unwrap();
            dbg!(&key);
            hotkeys_manager.register(key).unwrap();
            (key.id(), hotkey.command.to_string())
        })
        .collect();
    Ok(key_command_map)
}

fn main() {
    let event_loop = EventLoopBuilder::new().build();

    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();

    let hotkey3 = HotKey::new(None, Code::KeyE);
    let user_config = Config::read_config().unwrap();
    let key_command_map: HashMap<u32, String> = user_config
        .bindings
        .iter()
        .map(|hotkey| {
            let key: HotKey = hotkey.to_string().parse().unwrap();
            dbg!(&key);
            hotkeys_manager.register(key).unwrap();
            (key.id(), hotkey.command.to_string())
        })
        .collect();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if let Ok(event) = global_hotkey_channel.try_recv() {
            println!("Received hotkey event: {:?}", event);
            println!("Command: {:?}", key_command_map.get(&event.id));
            Command::new("sh")
                .arg("-c")
                .arg(key_command_map.get(&event.id).unwrap().to_string())
                .spawn()
                .expect("Failed to execute command");
        }
    })
}
