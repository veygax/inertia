use ashpd::desktop::remote_desktop::{DeviceType, RemoteDesktop};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let portal = RemoteDesktop::new().await?;
    let session = portal.create_session().await?;

    portal
        .select_devices(
            &session,
            DeviceType::Keyboard | DeviceType::Pointer,
            None,
            ashpd::desktop::PersistMode::Application,
        )
        .await?;

    println!("Waiting for Portal prompt");

    portal.start(&session, None).await?;

    println!("Got tracking!");

    Ok(())
}
