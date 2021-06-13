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
        let cpu_message = String::from("User didn't specify to track CPU");
        if config.monitor_cpu {
            let mut cpu_message = String::from("You are currently using : ");
            let output = CpuData::get_cpu();
            let percent = CpuData::calc_output_percentage(output);
            cpu_message.push_str(&percent.to_string());
            cpu_message.push_str(" percent of the cpu");
        }
        let ram_message = String::from("User didn't specify to track RAM ");
        if config.monitor_ram {
            let mut ram_message = String::from("You are current using : ");
            ram_message.push_str(" and ");
            let mem_output = MemData::get_ram();
            ram_message.push_str(&mem_output.total.to_string());
            ram_message.push_str(" bytes of ram");
        }
        let n = libnotify::Notification::new(&cpu_message,
        Some(ram_message),
        None);
        n.show().unwrap();
        thread::sleep(Duration::from_millis(4000));

    }
    libnotify::uninit();
}

