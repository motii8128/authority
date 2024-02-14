use async_std;
use authority::node::Authority;

#[async_std::main]
async fn main()
{
    let mut node = Authority::new("127.0.0.1:8080", true).await.unwrap();

    loop {
        node.get_client_task().await;
        node.manual_wheel_controller().await;
        node.get_robot_position().await;
    }
}