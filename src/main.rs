use num::integer::div_mod_floor;
use std::error::Error;
use std::time::{Duration, Instant};
use std::io::{self, Write};

pub struct Pomo {
    time_left: Option<Duration>,
    start_time: Option<Instant>,
    display_time: u64,
    done: bool,
}

impl Default for Pomo {
    fn default() -> Pomo {
        Pomo::new(25)
    }
}

impl Pomo {

    pub fn new(minutes: u64) -> Pomo {
        let seconds = Duration::new(minutes * 60, 0);
        Pomo {
            time_left: Some(seconds),
            start_time: None,
            display_time: seconds.as_secs(),
            done: false
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn check_finished(&mut self) {
        let elapsed = self.start_time.unwrap().elapsed();
        match elapsed {
            elapsed if  elapsed < self.time_left.unwrap() => {
                self.display_remaining_time(elapsed);
            },
            _ => {
                println!("All done");
            }

            }
        }

    fn display_remaining_time(&mut self, elapsed_time: Duration) {
        let currently_left = {
            self.time_left.unwrap().as_secs() - elapsed_time.as_secs()
        };

        match currently_left {
            currently_left if currently_left != self.display_time => {
                self.display_time = currently_left;
                self.generate_remaining_string();
            },
            _ => {},
        }
    }

    fn generate_remaining_string(&self) {
       let (minutes, seconds) = div_mod_floor(self.display_time, 60);
       let min_string = self.make_two_digit_str(minutes);
       let sec_string = self.make_two_digit_str(seconds);

       print!("\r{}:{} until done.", min_string, sec_string);
       io::stdout().flush().unwrap();

    }

    fn make_two_digit_str(&self, input_val: u64) -> String {
        let number_string = input_val.to_string();
        match number_string.len() {
            1 => {
                  let mut new_num_string: String = "0".to_string();
                  new_num_string.push_str(&number_string);
                  new_num_string
                  },
            2 => number_string,
            _ => "incorrect_value supplied".to_string(),
        }
    }
}

fn main() {

    let mut timer = Pomo::new(1);
    timer.start();

    loop {
        timer.check_finished();

    }
}
