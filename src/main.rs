use async_std;
use hrcs::server::*;

#[async_std::main]
async fn main()
{
    let hrcs = HRCS::init_hrcs("192.168.11.61").await.unwrap();

    hrcs.connect_client().await;
}