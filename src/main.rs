use macroquad;
use authority::node::Authority;

#[macroquad::main("3D")]
async fn main()
{
    let mut node = Authority::new("127.0.0.1:8080", false).await.unwrap();

    loop {
        node.get_client_task().await;
        node.manual_wheel_controller().await;
    }
}