use crate::controller::invoke::{info::Info, Method};
use serde::{Deserialize, Serialize};

use super::Controller;

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    pub id: u8,
    pub name: String,
    pub monitor: String,

    #[serde(rename = "monitorID")]
    pub monitor_id: u8,
    pub windows: u32,

    #[serde(rename = "hasfullscreen")]
    pub has_full_screen: bool,

    #[serde(rename = "lastwindow")]
    pub last_window: String,

    #[serde(rename = "lastwindowtitle")]
    pub last_window_title: String,
}

impl Controller {
    pub async fn get_workspaces(&self) -> anyhow::Result<Vec<Workspace>> {
        let workspaces = self.invoke(Method::Info(Info::Workspaces)).await?;
        let workspaces: Vec<Workspace> = serde_json::from_str(workspaces.as_str())?;

        Ok(workspaces)
    }
}
