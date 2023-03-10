// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod config;

use config::Config;
use heimdall::{configure_logger, spawn_command};
use std::{collections::HashMap, process::Command};
use tracing::{debug, info};

use global_hotkey::{
    hotkey::{Code, HotKey},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

fn main() {
    configure_logger();
    debug!("Starting Heimdall");

    let event_loop = EventLoopBuilder::new().build();

    let hotkeys_manager = GlobalHotKeyManager::new().unwrap();

    let key_command_map: HashMap<u32, String> = Config::read_config()
        .unwrap()
        .bindings
        .iter()
        .map(|hotkey| {
            let key: HotKey = hotkey.to_string().parse().unwrap();
            info!(
                "Registering hotkey: {:?} with command {:?}",
                key, hotkey.command
            );
            hotkeys_manager.register(key).unwrap();
            (key.id(), hotkey.command.to_string())
        })
        .collect();

    let global_hotkey_channel = GlobalHotKeyEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        control_flow.set_wait();

        if let Ok(event) = global_hotkey_channel.try_recv() {
            info!("Received hotkey event: {:?}", event);
            info!("Command: {:?}", key_command_map.get(&event.id));
            spawn_command!(key_command_map.get(&event.id).unwrap().to_string());
        }
    })
}
