use async_std::net::UdpSocket;
use crate::log;

pub async fn init_hrcs(addr:&str)->Result<UdpSocket, ()>
{
    match UdpSocket::bind(addr).await{
        Ok(socket)=>{
            log::log_info("Initialize Node".to_string());
            Ok(socket)
        }
        Err(_)=>{
            log::log_err("Failed to initialize Node".to_string());
            Err(())
        }
    }
}