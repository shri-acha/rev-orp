mod proxy_server;
fn main(){

    // Run proxy
    if let Err(e) = proxy_server::run_proxy_server() {
        println!("[ERROR] - error running proxy server! {e}");
    }

}
