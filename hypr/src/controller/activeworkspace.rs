use super::{
    invoke::{info::Info, Method},
    workspaces, Controller,
};

// they're identical
pub type ActiveWorkspace = workspaces::Workspace;

impl<'sock> Controller {
    pub async fn get_active_workspace(&self) -> anyhow::Result<ActiveWorkspace> {
        let active_workspace = self.invoke(Method::Info(Info::ActiveWorkspace)).await?;
        let active_workspace: ActiveWorkspace = serde_json::from_str(&active_workspace)?;

        Ok(active_workspace)
    }
}
