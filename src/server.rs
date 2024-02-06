use std::fmt::Error;

use async_std::net::UdpSocket;
use crate::log;

pub struct HRCS
{
    node_:UdpSocket,
    addr_:String,
    destination_:String
}

impl HRCS {
    pub async fn init_hrcs(addr:&str)->Result<HRCS, Error>
    {
        match UdpSocket::bind(addr).await{
            Ok(socket)=>{
                log::log_info("Initialize Node".to_string());
                Ok(HRCS{node_:socket, addr_:addr.to_string(), destination_:"ZERO".to_string()})
            }
            Err(_)=>{
                log::log_err("Failed to initialize Node".to_string());
                Err(std::fmt::Error)
            }
        }
    }
    pub async fn connect_client(mut self)
    {
        let mut buf = [0_u8; 512];
        match self.node_.recv_from(&mut buf).await {
            Ok(data)=>{
                self.destination_ = data.1.to_string();
                
                log::log_info(format!("Connect {}", self.destination_));

                let send_str = format!("[HRCS]:TRUE");
                let send_data = send_str.as_bytes();
                match self.node_.send_to(send_data, self.destination_).await
                {
                    Ok(_)=>{
                        log::log_info("Send Response".to_string())
                    }
                    Err(_)=>{
                        log::log_err("Failed to send response".to_string())
                    }
                }
            }
            Err(_)=>{
                log::log_err("Failed to connect client".to_string());
            }
        }
    }
    pub fn get_local_addr(self)->String
    {
        let re = self.addr_;

        re
    }
}