use core::str;

mod activewindow;
mod activeworkspace;
mod workspaces;

use crate::sock::{new_hyprctl_socket, SocketTypes};
use log::info;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const HYPRLAND_HYPRCTL_MAX_RESPONSE_BYTES: usize = 8192;

pub struct Controller(String, String);

impl Controller {
    pub async fn new(xdg_runtime_dir: &str, hypr_instance_signature: &str) -> Self {
        Controller(
            xdg_runtime_dir.to_string(),
            hypr_instance_signature.to_string(),
        )
    }

    // invokes hyprctl-esque controls via socket
    // note that this is by design a synchronous call (as per the socket impl)
    // therefore it's safe to fire & read by one line
    pub async fn invoke<'event>(
        &self,
        invoke_method: invoke::Method<'event>,
    ) -> anyhow::Result<String> {
        let Controller(xdg_runtime_dir, hypr_instance_signature) = self;

        // create one time socket
        let mut socket = new_hyprctl_socket(
            xdg_runtime_dir,
            hypr_instance_signature,
            SocketTypes::Controller,
        )
        .await
        .unwrap();

        let write_buf = match invoke_method {
            invoke::Method::Dispatch(dispatch_args) => format!("-j dispatch {}", dispatch_args),
            invoke::Method::Notify(icon, time_ms, color, message) => {
                format!("-j {} {} {} {}", icon, time_ms, color, message)
            }
            invoke::Method::DismissNotify(dismiss) => format!("-j dismissnotify {}", dismiss),
            invoke::Method::Info(inf) => format!("j/{}", inf),
        };

        info!(">> hyprctl {}", &write_buf);

        let write_as_bytes = write_buf.as_bytes();
        let _ = socket.write_all(write_as_bytes).await.unwrap();

        socket.readable().await.unwrap();

        // try read all
        let mut read_buf = bytes::BytesMut::with_capacity(HYPRLAND_HYPRCTL_MAX_RESPONSE_BYTES);
        let response_size = socket.read_buf(&mut read_buf).await.unwrap();

        info!(
            "<< hyprctl {} .. response size {}",
            &write_buf, response_size
        );

        Ok(str::from_utf8(read_buf.as_ref()).unwrap().into())
    }
}

pub mod invoke {
    // note: not a complete list; add as you go
    pub enum Method<'invoke> {
        Dispatch(&'invoke str),
        Notify(
            notify::Icon,
            notify::TimeMS,
            notify::Color<'invoke>,
            notify::Message<'invoke>,
        ),
        DismissNotify(dismiss_notify::Dismiss),
        Info(info::Info<'invoke>),
    }

    pub mod notify {
        use std::fmt;

        use strum_macros::Display;

        #[derive(Display)]
        #[strum(serialize_all = "lowercase")]
        pub enum Icon {
            NoIcon,
            Warning,
            Info,
            Hint,
            Error,
            Confused,
            Ok,
        }

        pub struct TimeMS(u32);
        impl fmt::Display for TimeMS {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        #[derive(Display)]
        pub enum Color<'c> {
            #[strum(to_string = "rgb({0})")]
            RGB(&'c str),

            #[strum(to_string = "rgba({0})")]
            RGBA(&'c str),
        }

        #[derive(Display)]
        pub enum Message<'c> {
            #[strum(to_string = "{0}")]
            Default(&'c str),

            #[strum(to_string = "fontsize:{0} {1}")]
            WithFontSize(u32, &'c str),
        }
    }

    pub mod dismiss_notify {
        use strum_macros::Display;

        #[derive(Display)]
        pub enum Dismiss {
            #[strum(to_string = "-1")]
            All,

            #[strum(to_string = "{0}")]
            Recent(u32),
        }
    }

    pub mod info {
        use strum_macros::Display;

        #[derive(Display)]
        #[strum(serialize_all = "lowercase")]
        pub enum Info<'i> {
            Version,
            Monitors,
            Workspaces,
            ActiveWorkspace,
            WorkspaceRules,
            Clients,
            Devices,
            Decorations(u32),
            Binds,
            ActiveWindow,
            Layers,
            Splash,
            GetOption(&'i str),
            CursorPos,
            Animations,
            Instances,
            Layouts,
            ConfigErrors,
            RollingLog,
            Locked,
        }
    }
}
