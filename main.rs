extern crate surf;
use std::convert::TryInto;
use surf::{Client, Config};
use surf::http::auth::BasicAuth;


#[tokio::main]
async fn main() -> Result<(), http_types::Error> {


    
    
    let auth = BasicAuth::new("antisys", "XXXXXX");
    let client: Client = Config::new()
    .add_header(auth.name(), auth.value())?
    .try_into()?;
    let data = serde_json::json!({ "name": "test", "jsonrpc": "1.0", "id":"surf-test-0", "method": "getblockchaininfo", "params": [] });
    let string = client.post("http://127.0.0.1:18332").body(http_types::Body::from_json(&data)?).recv_string().await?;
    println!("{}", string);
    let data = serde_json::json!({ "name": "test1", "jsonrpc": "1.0", "id":"surf-test-1", "method": "getchaintips", "params": [] });
    let string = client.post("http://127.0.0.1:18332").body(http_types::Body::from_json(&data)?).recv_string().await?;
    println!("{}", string);
    let data = serde_json::json!({ "id":"surf-test-3", "method": "getconnectioncount", "params": [] });
    let string = client.post("http://127.0.0.1:18332").body(http_types::Body::from_json(&data)?).recv_string().await?;
    println!("{}", string);
    let data = serde_json::json!({ "id":"surf-test-3", "method": "stop", "params": [] });
    let string = client.post("http://127.0.0.1:18332").body(http_types::Body::from_json(&data)?).recv_string().await?;
    println!("{}", string);
    Ok(())
}
