use serde::{Deserialize, Serialize};

use crate::Info;

use super::{invoke::Method, Controller};

#[derive(Serialize, Deserialize)]
pub struct ActiveWindow {
    pub class: String,
    pub title: String,
}

impl Controller {
    pub async fn get_active_window(&self) -> anyhow::Result<ActiveWindow> {
        let active_window = self.invoke(Method::Info(Info::ActiveWindow)).await?;
        let active_window: ActiveWindow = serde_json::from_str(active_window.as_str())?;

        Ok(active_window)
    }
}
