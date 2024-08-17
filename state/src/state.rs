use std::collections::BTreeSet;

use hypr::events::HyprctlEvents;
use log::info;
use serde::{Deserialize, Serialize};

use crate::Events;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    pub total_workspaces: BTreeSet<usize>,
    pub current_workspace: u32,
    pub current_app_name: String,

    pub current_volume: u32,
    pub current_brightness: u32,
}

pub enum StateUpdate {
    Updated,
    Nop,
}

impl State {
    pub fn update_from_event(&mut self, event: Events) -> anyhow::Result<StateUpdate> {
        match event {
            Events::Hypr(event) => match event {
                HyprctlEvents::WorkspaceV2 { workspace_id, .. } => {
                    self.current_workspace = workspace_id.parse().unwrap();
                    Ok(StateUpdate::Updated)
                }
                HyprctlEvents::ActiveWindow { window_title, .. } => {
                    self.current_app_name = window_title.to_string();
                    Ok(StateUpdate::Updated)
                }
                HyprctlEvents::CreateWorkspaceV2 { workspace_id, .. } => {
                    let workspace_id: usize = workspace_id.parse().unwrap();
                    self.total_workspaces.insert(workspace_id);

                    // self.current_workspace = workspace_id as u32;
                    Ok(StateUpdate::Updated)
                }
                HyprctlEvents::DestroyWorkspaceV2 { workspace_id, .. } => {
                    let workspace_id: usize = workspace_id.parse().unwrap();
                    self.total_workspaces.remove(&workspace_id);

                    Ok(StateUpdate::Updated)
                }
                HyprctlEvents::MoveWorkspaceV2 { workspace_id, .. } => {
                    let next_workspace: u32 = workspace_id.parse().unwrap();
                    self.current_workspace = next_workspace;
                    Ok(StateUpdate::Updated)
                }
                e => {
                    info!("?? not handling unknown state update {:?}", e);
                    Ok(StateUpdate::Nop)
                }
            },
        }
    }
}
