pub mod input;
pub mod keys;
    
use std::process::Command;
use std::str::from_utf8;
use std::fs::{File, OpenOptions};
use std::io::Read;

use input::Input;

#[derive(Debug)]
pub struct InputReader {
    file: File
}

impl InputReader {
    pub fn new(path: String) -> InputReader {
        InputReader{file: OpenOptions::new().read(true).write(false).append(false).open(path).expect("could not open device file")}
    }
    pub fn current_state(&mut self) -> Input{
        let mut buf: [u8; 24] = [0 as u8; 24];
        self.file.read(&mut buf).expect("error reading file");
        Input::from_read(&buf)
    }
}

pub fn get_input_devices() -> Vec<String>{
    let command = "grep -E 'Handlers|EV' /proc/bus/input/devices | grep -B1 120013 | grep -Eo event[0-9]+".to_string();
    let result = Command::new("sh").arg("-c").arg(command).output().expect("could not execute command to search for input devices");
    let result_str = from_utf8(&result.stdout).expect("could not get results");
        
    result_str.trim().split('\n').map(|s| "/dev/input/".to_string() + s).collect()
}