use serde::{Serialize, Deserialize};
use serde_json;

use crate::log;

#[derive(Serialize, Deserialize)]
pub struct Vector
{
    pub x:f32,
    pub y:f32,
    pub rotation:f32
}

impl Vector {
    pub fn new()->Vector
    {
        Vector{
            x:0.0,
            y:0.0,
            rotation:0.0
        }
    }
}

pub struct Robot
{
    pub current_position:Vector,
    pub target_position:Vector,
    enable_feedback:bool
}

impl Robot {
    pub fn new(enable_feedback:bool)->Robot
    {
        let c = Vector::new();
        let t = Vector::new();
        Robot{
            current_position:c,
            target_position:t,
            enable_feedback:enable_feedback,
        }
    }
    pub fn manual_robot(&mut self, x:f32, y:f32, rotation:f32)->String
    {
        if self.enable_feedback
        {
            self.target_position.x += x;
            self.target_position.y += y;
            self.target_position.rotation += rotation;

            let mut vec = Vector::new();
            vec.x = self.target_position.x - self.current_position.x;
            vec.y = self.target_position.y - self.current_position.y;
            vec.rotation = self.target_position.rotation - self.current_position.rotation;

            serde_json::to_string(&vec).unwrap()
        }
        else {
            let v = Vector
            {
                x:x,
                y:y,
                rotation:rotation
            };

            serde_json::to_string(&v).unwrap()
        }
    }
    pub fn set_current_position(&mut self, position:String)
    {
        match serde_json::from_str::<Vector>(&position)
        {
            Ok(get)=>{
                self.current_position = get;
            }
            Err(_)=>{
                log::log_err("Diff Message type".to_string());
            }
        }
    }
}