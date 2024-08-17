pub mod events;
mod sock;

mod controller;
mod listener;

use anyhow::anyhow;
use controller::{invoke::Method, Controller};
use events::HyprctlEvents;
use listener::Listener;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};

pub use controller::invoke::info::*;

pub struct Hypr {
    listener: Lines<BufReader<Listener>>,
    controller: Controller,
}

impl Hypr {
    pub async fn new(xdg_runtime_dir: &str, hyprland_instance_signature: &str) -> Self {
        let listener = Listener::new(&xdg_runtime_dir, &hyprland_instance_signature).await;
        let controller = Controller::new(&xdg_runtime_dir, &hyprland_instance_signature).await;

        let listener_as_bufread = BufReader::new(listener).lines();

        Self {
            listener: listener_as_bufread,
            controller,
        }
    }

    pub fn controller(&self) -> &Controller {
        &self.controller
    }

    pub async fn next(&mut self) -> anyhow::Result<HyprctlEvents> {
        // read line
        let next_line = self
            .listener
            .next_line()
            .await?
            .ok_or(anyhow!("empty line fed"))?;

        // parse events
        let ev = HyprctlEvents::decode_from_string(next_line)?;

        Ok(ev)
    }

    pub async fn invoke<'inv>(&self, method: Method<'inv>) -> anyhow::Result<String> {
        self.controller.invoke(method).await
    }
}
