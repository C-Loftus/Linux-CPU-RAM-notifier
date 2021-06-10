extern crate libnotify;
use std::{env, process, thread};
mod lib;
use lib::Config;
use std::time::Duration;



fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem Parsing Arguments: {}", err);
        process::exit(1);
    });

    libnotify::init("myapp").unwrap();

    // Init libnotify
    // Create a new notification (doesn't show it yet)
    let n = libnotify::Notification::new("Summary",
                                         Some("Optional Body"),
                                         None);

    // Show the notification
    n.show().unwrap();
    thread::sleep(Duration::from_millis(4000));
    // Update the existent notification
    n.update("I am another notification", None, None).unwrap();
    // Show the updated notification
    n.show().unwrap();
    // We are done, deinit
    libnotify::uninit();
}
