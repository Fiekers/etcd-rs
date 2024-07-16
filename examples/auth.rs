use ya_etcd_rs::{AuthOp, Client, ClientConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = ClientConfig::new(
        [
            "http://127.0.0.1:12379".into(),
            "http://127.0.0.1:22379".into(),
            "http://127.0.0.1:32379".into(),
        ]
        .to_vec(),
    )
    .auth("".to_owned(), "".to_owned());

    let cli = Client::new(cfg).await?;
    let resp = cli.auth_status().await?;
    if resp.enabled {
        println!("auth is enabled...");
        cli.auth_disable().await?;
        let resp = cli.auth_status().await?;
        assert!(!resp.enabled);
        cli.auth_enable().await?;
        let resp = cli.auth_status().await?;
        assert!(resp.enabled);
    }

    Ok(())
}
