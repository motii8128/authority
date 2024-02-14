use async_std;
use authority::node::Authority;

#[async_std::main]
async fn main()
{
    let mut node = Authority::new("127.0.0.1:8080", false).await.unwrap();

    loop {
        node.connect_viewer().await;
    }
}