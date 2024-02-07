use std::fmt::Error;

use async_std::net::UdpSocket;
use crate::log;

pub struct HRCS
{
    node_:UdpSocket,
    addr_:String,
    destination_:Vec<String>,
    id_table_:Vec<String>
}

impl HRCS {
    pub async fn new(addr:&str)->Result<HRCS, Error>
    {
        match UdpSocket::bind(addr).await{
            Ok(socket)=>{
                log::log_info("Initialized Node".to_string());
                Ok(HRCS{
                    node_:socket, 
                    addr_:addr.to_string(), 
                    destination_:Vec::<String>::new(),
                    id_table_:Vec::<String>::new()
                })
            }
            Err(e)=>{
                log::log_err("Failed to initialize Node".to_string());
                log::log_err(e.to_string());
                Err(std::fmt::Error)
            }
        }
    }
    pub async fn get_client(&mut self)
    {
        let mut buf = [0_u8; 512];
        match self.node_.recv_from(&mut buf).await {
            Ok(data)=>{
                match self.destination_.iter().position(|x| *x == data.1.to_string())
                {
                    Some(_)=>{

                    }
                    None=>{
                        self.destination_.push(data.1.to_string());
                        let dest_id = String::from_utf8_lossy(&buf[..data.0]).to_string();
                        self.id_table_.push(dest_id);
                        
                        log::log_info(format!("Connect {}", data.1.to_string()));

                        let send_str = format!("[HRCS]:TRUE");
                        let send_data = send_str.as_bytes();
                        match self.node_.send_to(send_data, data.1.to_string()).await
                        {
                            Ok(_)=>{
                                log::log_info("Send Response".to_string())
                            }
                            Err(_)=>{
                                log::log_err("Failed to send response".to_string())
                            }
                        }
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