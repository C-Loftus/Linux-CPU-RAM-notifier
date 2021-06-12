extern crate libnotify;
use std::{env, process, thread};
mod lib;
use lib::{Config, CpuData, MemData};
use std::time::Duration;



fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem Parsing Arguments: {}", err);
        process::exit(1);
    });

    libnotify::init("myapp").unwrap();
    loop {
        let mut message = String::from("You are currently using : ");
        if config.monitor_cpu {
            let output = CpuData::get_cpu();
            let percent = CpuData::calc_output_percentage(output);
            message.push_str(&percent.to_string());
        }
        if config.monitor_ram {
            message.push_str(" and ");
            let mem_output = MemData::get_ram();
            message.push_str(&mem_output.total.to_string());
        }
        let n = libnotify::Notification::new(&message,
        Some("Optional Body"),
        None);
        n.show().unwrap();
        thread::sleep(Duration::from_millis(4000));

    }
    libnotify::uninit();
}

