use crate::sock::{new_hyprctl_socket, SocketTypes};
use tokio::{io::AsyncRead, net::UnixStream};
pub struct Listener(pub UnixStream);

impl Listener {
    pub async fn new(xdg_runtime_dir: &str, hypr_instance_signature: &str) -> Self {
        Listener(
            new_hyprctl_socket(
                xdg_runtime_dir,
                hypr_instance_signature,
                SocketTypes::Listener,
            )
            .await
            .unwrap(),
        )
    }
}

// jeez
impl AsyncRead for Listener {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.get_mut().0).poll_read(cx, buf)
    }
}
