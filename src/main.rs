use std::process::Command;
use std::thread;
use std::time::Duration;

struct Led {
    device: String,
    tu: Duration,
}

impl Led {
    pub fn new(device: &str) -> Self {
        Self {
            device: device.to_string(),
            tu: Duration::from_millis(300),
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

    pub fn morse(&self, mut word: String) {
        word = word.to_uppercase();
        loop{
            for letter in word.chars() {

                if letter == ' ' {
                    thread::sleep(self.tu*3);
                    continue;
                }

                let morsed_letter = self.morsify(letter);
                
                for i in 0..8 {
                    let val = (morsed_letter >> i) & 1;
                    if val>0 {
                        self.line();
                    } else {
                        self.dot();
                    }
                    thread::sleep(self.tu);
                }

                thread::sleep(self.tu*3);
            }
            thread::sleep(self.tu*10);
        }
    }

    fn morsify(&self, letter: char) -> u8{
        match letter {
            'A' => 0b01,
            'B' => 0b1000,
            'C' => 0b1010,
            'D' => 0b100,
            'E' => 0b0,
            'F' => 0b0010,
            'G' => 0b110,
            'H' => 0b0000,
            'I' => 0b00,
            'J' => 0b0111,
            'K' => 0b101,
            'L' => 0b0100,
            'M' => 0b11,
            'N' => 0b10,
            'O' => 0b111,
            'P' => 0b0110,
            'Q' => 0b1101,
            'R' => 0b010,
            'S' => 0b000,
            'T' => 0b1,
            'U' => 0b001,
            'V' => 0b0001,
            'W' => 0b011,
            'X' => 0b1001,
            'Y' => 0b1011,
            'Z' => 0b1100,
            _   => 0b0000000
        }
    }

    fn dot(&self){
        self.brightness(255);
        thread::sleep(self.tu);
        self.brightness(0);
    }

    fn line(&self){
        self.brightness(255);
        thread::sleep(self.tu*3);
        self.brightness(0);
    }

    fn brightness(&self, strength: u8){
        let command = format!("brightnessctl");
        
        Command::new(&command)
            .arg("-q")
            .arg(format!("--device={}",self.device))
            .args(
                &["set", &strength.to_string()]
            )
            .spawn().expect("Failed to execute command");
    }
}

fn main() {
    let light = Led::new("tpacpi::lid_logo_dot"); 

    //light.blink(Duration::from_millis(100));
    light.morse("ABC".to_string());
}

