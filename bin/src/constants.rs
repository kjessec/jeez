use std::env;

pub struct Constants {
    pub xdg_runtime_dir: String,
    pub hyprland_instance_signature: String,
}

impl Constants {
    pub fn new() -> Self {
        let xdg_runtime_dir: String =
            env::var("XDG_RUNTIME_DIR").expect("env XDG_RUNTIME_DIR not set");
        let hyprland_instance_signature: String = env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .expect("env HYPRLAND_INSTANCE_SIGNATURE not set");

        Self {
            xdg_runtime_dir: xdg_runtime_dir.clone(),
            hyprland_instance_signature: hyprland_instance_signature.clone(),
        }
    }
}
