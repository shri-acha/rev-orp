mod proxy_server;
mod mock_server;

use tokio;

#[tokio::main]
async fn main() {
    tokio::join!(
        async {
            if let Err(e) = mock_server::run_mock_server().await {
                println!("[ERROR] - error running mock server! {e}");
            }
        },
        async {
            if let Err(e) = proxy_server::run_proxy_server().await {
                println!("[ERROR] - error running proxy server! {e}");
            }
        }
    );
}
