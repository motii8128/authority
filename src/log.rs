use colored::{self, Colorize};

pub fn log_info(msg:String)
{
    println!("[HRCS]:{}", msg.blue());
}

pub fn log_err(msg:String)
{
    println!("[HRCS]:{}", msg.red());
}