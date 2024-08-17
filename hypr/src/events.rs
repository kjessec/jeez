use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HyprctlEvents {
    Workspace {
        workspace_name: String,
    },
    WorkspaceV2 {
        workspace_id: String,
        workspace_name: String,
    },
    FocusedMon {
        mon_name: String,
        workspace_name: String,
    },
    ActiveWindow {
        window_class: String,
        window_title: String,
    },
    ActiveWindowV2 {
        window_address: String,
    },
    FullScreen(bool),
    MonitorRemoved {
        monitor_name: String,
    },
    MonitorAdded {
        monitor_name: String,
    },
    MonitorAddedV2 {
        monitor_id: String,
        monitor_name: String,
        monitor_description: String,
    },
    CreateWorkspace {
        workspace_name: String,
    },
    CreateWorkspaceV2 {
        workspace_id: String,
        workspace_name: String,
    },
    DestroyWorkspace {
        workspace_name: String,
    },
    DestroyWorkspaceV2 {
        workspace_id: String,
        workspace_name: String,
    },
    MoveWorkspace {
        workspace_name: String,
        mon_name: String,
    },
    MoveWorkspaceV2 {
        workspace_id: String,
        workspace_name: String,
        mon_name: String,
    },
    RenameWorkspace {
        workspace_id: String,
        new_name: String,
    },
    ActiveSpecial {
        workspace_name: String,
        mon_name: String,
    },
    ActiveLayout {
        keyboard_name: String,
        layout_name: String,
    },
    OpenWindow {
        window_address: String,
        workspace_name: String,
        window_class: String,
        window_title: String,
    },
    CloseWindow {
        window_address: String,
    },
    MoveWindow {
        window_address: String,
        workspace_name: String,
    },
    MoveWindowV2 {
        window_address: String,
        workspace_id: String,
        workspace_name: String,
    },
    OpenLayer {
        namespace: String,
    },
    CloseLayer {
        namespace: String,
    },
    Submap {
        submap_name: String,
    },
    ChangeFloatingMode {
        window_address: String,
        floating: String,
    },
    Urgent {
        window_address: String,
    },
    Minimize {
        window_address: String,
        minimized: String,
    },
    Screencast {
        state: u8,
        owner: u8,
    },
    WindowTitle {
        window_address: String,
    },
    WindowTitleV2 {
        window_address: String,
        window_title: String,
    },
    ToggleGroup {
        state: u8,
        handle: Vec<String>,
    },
    MoveIntoGroup {
        window_address: String,
    },
    MoveOutOfGroup {
        window_address: String,
    },
    IgnoreGroupLock(u8),
    LockGroups(u8),
    ConfigReloaded,
    Pin {
        window_address: String,
        pin_state: String,
    },
}

impl HyprctlEvents {
    pub fn decode_from_string(other: String) -> anyhow::Result<Self> {
        let split: Vec<&str> = other.split(">>").collect();
        let event_name = split.get(0).expect("invalid event name").to_owned();
        let event_args: Vec<&str> = split
            .get(1)
            .expect("invalid event args")
            .split(",")
            .collect();

        let res = match event_name {
            "workspace" => HyprctlEvents::Workspace {
                workspace_name: event_args.get(0).context("no item")?.to_string(),
            },
            "workspacev2" => HyprctlEvents::WorkspaceV2 {
                workspace_id: event_args.get(0).context("no item")?.to_string(),
                workspace_name: event_args.get(1).context("no item")?.to_string(),
            },
            "focusedmon" => HyprctlEvents::FocusedMon {
                mon_name: event_args.get(0).context("no item")?.to_string(),
                workspace_name: event_args.get(1).context("no item")?.to_string(),
            },
            "activewindow" => HyprctlEvents::ActiveWindow {
                window_class: event_args.get(0).context("no item")?.to_string(),
                window_title: event_args.get(1).context("no item")?.to_string(),
            },
            "activewindowv2" => HyprctlEvents::ActiveWindowV2 {
                window_address: event_args.get(0).context("no item")?.to_string(),
            },
            "fullscreen" => HyprctlEvents::FullScreen(
                event_args
                    .get(0)
                    .context("no item")?
                    .to_string()
                    .parse()
                    .context("invalid bool")?,
            ),
            "monitorremoved" => HyprctlEvents::MonitorRemoved {
                monitor_name: event_args.get(0).context("no item")?.to_string(),
            },
            "monitoradded" => HyprctlEvents::MonitorAdded {
                monitor_name: event_args.get(0).context("no item")?.to_string(),
            },
            "monitoraddedv2" => HyprctlEvents::MonitorAddedV2 {
                monitor_id: event_args.get(0).context("no item")?.to_string(),
                monitor_name: event_args.get(1).context("no item")?.to_string(),
                monitor_description: event_args.get(2).context("no item")?.to_string(),
            },
            "createworkspace" => HyprctlEvents::CreateWorkspace {
                workspace_name: event_args.get(0).context("no item")?.to_string(),
            },
            "createworkspacev2" => HyprctlEvents::CreateWorkspaceV2 {
                workspace_id: event_args.get(0).context("no item")?.to_string(),
                workspace_name: event_args.get(1).context("no item")?.to_string(),
            },
            "destroyworkspace" => HyprctlEvents::DestroyWorkspace {
                workspace_name: event_args.get(0).context("no item")?.to_string(),
            },
            "destroyworkspacev2" => HyprctlEvents::DestroyWorkspaceV2 {
                workspace_id: event_args.get(0).context("no item")?.to_string(),
                workspace_name: event_args.get(1).context("no item")?.to_string(),
            },
            "moveworkspace" => HyprctlEvents::MoveWorkspace {
                workspace_name: event_args.get(0).context("no item")?.to_string(),
                mon_name: event_args.get(1).context("no item")?.to_string(),
            },
            "moveworkspacev2" => HyprctlEvents::MoveWorkspaceV2 {
                workspace_id: event_args.get(0).context("no item")?.to_string(),
                workspace_name: event_args.get(1).context("no item")?.to_string(),
                mon_name: event_args.get(2).context("no item")?.to_string(),
            },
            "renameworkspace" => HyprctlEvents::RenameWorkspace {
                workspace_id: event_args.get(0).context("no item")?.to_string(),
                new_name: event_args.get(1).context("no item")?.to_string(),
            },
            "openlayer" => HyprctlEvents::OpenLayer {
                namespace: event_args.get(0).context("no item")?.to_string(),
            },
            "closelayer" => HyprctlEvents::CloseLayer {
                namespace: event_args.get(0).context("no item")?.to_string(),
            },
            "changefloatingmode" => HyprctlEvents::ChangeFloatingMode {
                window_address: event_args.get(0).context("no item")?.to_string(),
                floating: event_args.get(1).context("no item")?.to_string(),
            },
            // "togglegroup" => HyprctlEvents::ToggleGroup {
            //     state: event_args
            //         .get(0)
            //         .context("no item")?
            //         .to_string()
            //         .parse()
            //         .context("invalid u8")?,
            //     handle: event_args
            //         .get(1)
            //         .context("no item")?
            //         .to_string()
            //         .split(" ")
            //         .collect(),
            // },
            "ignore_grouplock" => HyprctlEvents::IgnoreGroupLock(
                event_args
                    .get(0)
                    .context("no item")?
                    .to_string()
                    .parse()
                    .context("invalid u8")?,
            ),
            "lockgroups" => HyprctlEvents::LockGroups(
                event_args
                    .get(0)
                    .context("no item")?
                    .to_string()
                    .parse()
                    .context("invalid u8")?,
            ),

            // add all enum variants
            e => return Err(anyhow!("unsupported event {:?}", e)),
        };

        Ok(res)
    }
}
