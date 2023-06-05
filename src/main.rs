use std::process::Command;
use std::thread;
use std::time::Duration;
use std::env;

enum Symbol {
    Dash,
    Dot,
}
struct Led {
    device: String,
    tu: Duration,
}

impl Led {
    pub fn new(device: &str) -> Self {
        Self {
            device: device.to_string(),
            tu: Duration::from_millis(256),
        }
    }
    
    pub fn blink(&self) {
        loop{
            self.sig(Symbol::Dot);
        }
    }

    pub fn morse(&self, mstring: &str) {
        let mstring = mstring.to_string().to_uppercase();
        self.brightness(0);
        thread::sleep(Duration::from_millis(1000));
        loop{
            for letter in mstring.chars() {
                
                if letter == ' ' {
                    thread::sleep(self.tu*7);
                    continue;
                }

                let morsed_letter = self.morsify(letter);
                
                // write one letter
                for signal in morsed_letter {
                    match signal {
                        2 => continue,
                        1 => self.sig(Symbol::Dash),
                        0 => self.sig(Symbol::Dot),
                        _ => println!("morse val error"),
                    }
                }
                thread::sleep(self.tu*3); // letter spacing

            }
            thread::sleep(self.tu*9);
        }
    }

    fn morsify(&self, letter: char) -> [u8; 5]{
        match letter {
            'A' => [0,1, 2,2,2],
            'B' => [1,0,0,0, 2],
            'C' => [1,0,1,0, 2],
            'D' => [1,0,0, 2,2],
            'E' => [0 ,2,2,2,2],
            'F' => [0,0,1,0, 2],
            'G' => [1,1,0, 2,2],
            'H' => [0,0,0,0, 2],
            'I' => [0,0, 2,2,2],
            'J' => [0,1,1,1, 2],
            'K' => [1,0,1, 2,2],
            'L' => [0,1,0,0, 2],
            'M' => [1,1, 2,2,2],
            'N' => [1,0, 2,2,2],
            'O' => [1,1,1, 2,2],
            'P' => [0,1,1,0, 2],
            'Q' => [1,1,0,1, 2],
            'R' => [0,1,0, 2,2],
            'S' => [0,0,0, 2,2],
            'T' => [1, 2,2,2,2],
            'U' => [0,0,1, 2,2],
            'V' => [0,0,0,1, 2],
            'W' => [0,1,1, 2,2],
            'X' => [1,0,0,1, 2],
            'Y' => [1,0,1,1, 2],
            'Z' => [1,1,0,0, 2],
            '1' => [0,1,1,1,1 ],
            '2' => [0,0,1,1,1 ],
            '3' => [0,0,0,1,1 ],
            '4' => [0,0,0,0,1 ],
            '5' => [0,0,0,0,0 ],
            '6' => [1,0,0,0,0 ],
            '7' => [1,1,0,0,0 ],
            '8' => [1,1,1,0,0 ],
            '9' => [1,1,1,1,0 ],
            '0' => [1,1,1,1,1 ],
            ' ' => [ 2,2,2,2,2],
            _   => [ 2,2,2,2,2],
        }
    }
    
    fn sig(&self, s: Symbol){
        let tu_mult:u32 = match s {
            Symbol::Dot => 1,
            Symbol::Dash => 3,
        };

        self.brightness(255);
        thread::sleep(self.tu*tu_mult);
        self.brightness(0);
        thread::sleep(self.tu);
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
    
    let args: Vec<String> = env::args().collect();

    //light.blink(Duration::from_millis(100));

    if args.len() > 1 {
        light.morse(&args[1]);
    } else {
        light.morse("I USE ARCH BTW");
    }

}

