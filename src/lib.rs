use std::{fs, error::Error, env};

extern crate systemstat;
use systemstat::{System, Platform};

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let mut monitor_ram = false;
        let mut monitor_cpu = false;
        if args.len() < 2 {
            return Err("User did not specify what to monitor");
        }
        let length = args.len() - 1;
        for i in 0..length {
            if args[i] == "-r" {
                monitor_ram = true;
            }
            if args[i] == "-c" {
                monitor_cpu = true;
            }
        }

        return Ok(Config {monitor_ram, monitor_cpu})
    }
}


pub struct Config {
    pub monitor_ram: bool,
    pub monitor_cpu: bool,
}

pub struct MemData {
    pub total: u64,
    pub free: u64,
}

pub fn get_ram() -> MemData {
    let sys = System::new();
    
    let output = match sys.memory().unwrap() {
        Ok(mem) => MemData {total : systemstat::data::
            ByteSize::gb(mem.total.as_u64()).as_u64(), 
            free: (systemstat::data::ByteSize::gb
                (mem.free.as_u64())).as_u64() },

        Err(x) => {
            MemData { total : 0, free : 0};
            eprintln!("Error Getting Memory");
            }
        }
    output 
    }

pub fn get_cpu() -> u32 {
    let sys = System::new();
    0
    }


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_config() {
        let args = vec!["temp first arg, usually will be executable".to_string(), "-r".to_string(), "-c".to_string()];
        let test = Config { monitor_ram: true,
             monitor_cpu: true
            };
        assert_eq!(test.monitor_ram, Config::new(&args).unwrap().monitor_ram);
    }

    #[test]
    fn valid_file() {
        get_ram();
        ()

    }

}

//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";

//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     }
// }