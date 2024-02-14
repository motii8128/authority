use std::fmt::Error;
use std::time::Duration;
use async_std::net::UdpSocket;
use crate::{log, robot_control::*};
use dualshock_driver::DualShock4;

pub const SIM_IP:&str = "127.0.0.1:6565";

pub const SEARCH_CLIENT:u8 = 0x00;
pub const START_ROBOT:u8 = 0xff;

struct Client
{
    pub ip_addr:String
}

pub struct Authority
{
    sock:UdpSocket,
    addr_:String,
    participants:Client,
    robot:Robot,
    controller:DualShock4,
    cycle:u8
}

impl Authority {
    pub async fn new(addr:&str, enable_feedback:bool)->Result<Authority, Error>
    {
        match UdpSocket::bind(addr).await{
            Ok(socket)=>{
                log::log_info("Initialized Node".to_string());
                log::log_info(format!("enable_feedback:{}", enable_feedback));
                let con = DualShock4::new().unwrap();
                Ok(Authority{
                    sock:socket,
                    addr_:addr.to_string(), 
                    participants:Client {ip_addr: "None".to_string()},
                    robot:Robot::new(enable_feedback),
                    controller:con,
                    cycle:SEARCH_CLIENT,
                })
            }
            Err(e)=>{
                log::log_err("Failed to initialize Node".to_string());
                log::log_err(e.to_string());
                Err(std::fmt::Error)
            }
        }
    }
    pub async fn get_client_task(&mut self)
    {
        if self.cycle == SEARCH_CLIENT
        {
            let mut buf = [0_u8; 512];
            match self.sock.recv_from(&mut buf).await {
                Ok(data)=>{
                    std::thread::sleep(Duration::from_millis(1000));

                    let dest_id = String::from_utf8_lossy(&buf[..data.0]).to_string();
                            
                    log::log_info(format!("Connect {}", dest_id));

                    let new_cli = Client{ip_addr:data.1.to_string()};
                    self.participants = new_cli;

                    let send_str = format!("[HostPC]:TRUE");
                    let send_data = send_str.as_bytes();
                    for _ in 0..2
                    {
                        match self.sock.send_to(send_data, data.1.to_string()).await
                        {
                            Ok(_)=>{
                            log::log_info(format!("Send Response to {}", data.1.to_string()))
                            }
                            Err(_)=>{
                                log::log_err("Failed to send response".to_string())
                            }
                        }
                    }
                    self.cycle = START_ROBOT;
                }
                Err(_)=>{
                    log::log_err("Failed to connect client".to_string());
                }
            }
        }
    }
    pub fn get_local_addr(&self)->String
    {
        let re = self.addr_.clone();

        re
    }
    pub async fn manual_wheel_controller(&mut self)
    {
        if self.cycle == START_ROBOT
        {
            let _ = self.controller.read().unwrap();
            let x = self.controller.sticks.left_x;
            let y = self.controller.sticks.left_y;
            let rotation = self.controller.sticks.right_x;

            let vec = self.robot.manual_robot(x, y, rotation);

            match self.sock.send_to(vec.as_bytes(), self.participants.ip_addr.as_str()).await {
                Ok(_)=>{
                    log::log_info(format!("Send {}", vec));
                }
                Err(_)=>{
                    log::log_err("Send error".to_string());
                }
            }
        }
    }
    pub async fn connect_viewer(&mut self)
    {
        let _ = self.controller.read().unwrap();
            let x = self.controller.sticks.left_x;
            let y = self.controller.sticks.left_y;
            let rotation = self.controller.sticks.right_x;

            let vec = self.robot.manual_robot(x, y, rotation);

            match self.sock.send_to(vec.as_bytes(), SIM_IP).await {
                Ok(_)=>{
                    log::log_info(format!("Send {}", vec));
                }
                Err(_)=>{
                    log::log_err("Send error".to_string());
                }
            }
    }
    pub async fn get_robot_position(&mut self)
    {
        if self.cycle == START_ROBOT
        {
            let mut buf = [0_u8; 256];
            match self.sock.recv_from(&mut buf).await {
                Ok((size, dest))=>{
                    if dest.to_string() == self.participants.ip_addr
                    {
                        let get_data = &buf[..size];
                        let msg = String::from_utf8_lossy(get_data).to_string();
                        
                        self.robot.set_current_position(msg);
                    }
                    else {
                        log::log_err("Recieve from another micro_controller".to_string());
                    }
                }
                Err(_)=>{
                    log::log_err("Failed to get position from client".to_string());
                }
            }
        }
    }
    
}