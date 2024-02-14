use colored::{self, Colorize};

pub fn log_info(msg:String)
{
    println!("[Authority]:{}", msg.blue());
}

pub fn log_err(msg:String)
{
    println!("[Authority]:{}", msg.red());
}