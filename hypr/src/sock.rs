use anyhow::anyhow;
use tokio::net::UnixStream;

pub enum SocketTypes {
    Listener,
    Controller,
}

pub async fn new_hyprctl_socket(
    xdg_runtime_dir: &str,
    hypr_instance_signature: &str,
    socket_variant: SocketTypes,
) -> anyhow::Result<UnixStream> {
    let socket_path = match socket_variant {
        SocketTypes::Listener => ".socket2.sock",
        SocketTypes::Controller => ".socket.sock",
    };

    let socket_path = format!(
        "{}/hypr/{}/{}",
        xdg_runtime_dir, hypr_instance_signature, socket_path
    );

    let Ok(stream) = UnixStream::connect(socket_path).await else {
        return Err(anyhow!("failed to establish socket"));
    };

    Ok(stream)
}
