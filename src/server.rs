use std::fmt::Error;
use std::time::Duration;

use async_std::net::UdpSocket;
use crate::log;

struct Client
{
    pub name:String,
    pub ip_addr:String,
}

pub struct HRCS
{
    node_:UdpSocket,
    addr_:String,
    participants:Vec<Client>
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
                    participants:Vec::<Client>::new()
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
        // self.destination_.iter().position(|x| *x == data.1.to_string())
        let mut buf = [0_u8; 512];
        match self.node_.recv_from(&mut buf).await {
            Ok(data)=>{
                match self.participants.iter().position(|x| *x.ip_addr == data.1.to_string())
                {
                    Some(_)=>{

                    }
                    None=>{
                        std::thread::sleep(Duration::from_millis(1000));

                        let dest_id = String::from_utf8_lossy(&buf[..data.0]).to_string();
                        
                        log::log_info(format!("Connect {}", dest_id));

                        let new_cli = Client{ip_addr:data.1.to_string(), name:dest_id};
                        self.participants.push(new_cli);

                        let send_str = format!("[HRCS]:TRUE");
                        let send_data = send_str.as_bytes();
                        for _ in 0..2
                        {
                            match self.node_.send_to(send_data, data.1.to_string()).await
                            {
                                Ok(_)=>{
                                    log::log_info(format!("Send Response to {}", data.1.to_string()))
                                }
                                Err(_)=>{
                                    log::log_err("Failed to send response".to_string())
                                }
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
    pub fn get_local_addr(&self)->String
    {
        let re = self.addr_.clone();

        re
    }
    pub fn client_list(&self)
    {
        log::log_info("Show client list".to_string());
        for u in 0..self.participants.len()
        {
            let n = &self.participants.get(u).unwrap().name;
            log::log_info(n.clone());
        }
    }
}