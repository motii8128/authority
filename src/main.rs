use async_std;
use authority::hrcs::*;

#[async_std::main]
async fn main()
{
    let mut node = Authority::new("127.0.0.1:8080").await.unwrap();

    loop {
        node.get_client().await;
        node.manual_controller().await;    
    }
}