use std::process::Command;
use std::thread;
use std::time::Duration;

struct Led {
    device: String,
}

impl Led {
    fn new(&self, device: &str) -> Self {
        Self {
            self.device = device.to_string();
        }
    }
    
    fn blink(&self, delay: Duration) {
        loop{
            self.brightness(0);
            thread::sleep(delay);
            self.brightness(255);
            thread::sleep(delay);
        }

    }

    fn brightness(&self, strength: u8){
        let command = format!("brightnessctl -q"); 
        Command::new(&command)
            .arg(format!("--device={}",self.device))
            .args(&["set", "100%"])
            .spawn.expect("Failed to execute command");
    }
}

fn main() {
    let light = Led::new("tpacpi::lid_logo_dot"); 
    //blink_lid_light(10000,Duration::from_millis(50));

    light.blink(Duration::from_millis(100));
}

