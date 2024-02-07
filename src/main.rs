use async_std;
use hrcs::server::*;

#[async_std::main]
async fn main()
{
    let mut hrcs = HRCS::new("192.168.11.61:8080").await.unwrap();

    let _ = hrcs.get_client();
}