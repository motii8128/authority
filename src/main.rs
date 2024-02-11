use async_std;
use hrcs::hrcs::*;

#[async_std::main]
async fn main()
{
    let mut hrcs = HRCS::new("127.0.0.1:8080").await.unwrap();

    loop {
        hrcs.get_client().await;
        hrcs.manual_controller().await;        
    }
}