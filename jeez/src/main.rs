mod constants;
use log::{debug, warn};
use serde_json::json;
use state::StateUpdate;
use tokio::sync::mpsc;

macro_rules! continue_on_err {
    ($predicate:expr, $err_patt:expr) => {
        match $predicate {
            Ok(ret) => ret,
            Err(e) => {
                warn!("{}: {}", $err_patt, e);
                continue;
            }
        }
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let constants = constants::Constants::new();

    let mut hypr = hypr::Hypr::new(
        &constants.xdg_runtime_dir,
        &constants.hyprland_instance_signature,
    )
    .await;

    // initialize global state with some default values
    let initial_total_workspaces =
        (1..hypr.controller().get_workspaces().await?.len() + 1 as usize).collect();
    let initial_current_workspace = hypr.controller().get_active_workspace().await?.id as u32;
    let initial_current_app_name = hypr.controller().get_active_window().await?;
    let initial_current_app_name = format!(
        "{} / {}",
        initial_current_app_name.class, initial_current_app_name.title
    );

    let mut state = state::State {
        total_workspaces: initial_total_workspaces,
        current_workspace: initial_current_workspace,
        current_app_name: initial_current_app_name,
        current_volume: Default::default(),
        current_brightness: Default::default(),
    };

    // print initial state
    println!("{}", json!(state));

    let (tx, mut rx) = mpsc::channel::<state::Events>(1024);

    tokio::spawn(async move {
        let tx = tx.clone();
        loop {
            let next_event = continue_on_err!(
                hypr.next().await,
                "received hypr events but could not decode"
            );

            tx.send(state::Events::Hypr(next_event)).await.unwrap();
        }
    });

    while let Some(event) = rx.recv().await {
        let state_update = continue_on_err!(state.update_from_event(event), "state update failed");

        match state_update {
            StateUpdate::Updated => {
                warn!("state updated");
                println!("{}", json!(state));
            }
            StateUpdate::Nop => {
                debug!("event received, but no matching updater found");
                continue;
            }
        }
    }

    Ok(())
}
