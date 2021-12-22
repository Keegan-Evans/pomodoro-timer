use std::{fmt::Display, time::{Duration, Instant}};

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

//impl Display for Pomo {
//    fn display_remaining(&self) -> String {
//        format!("{}", self.time_left.unwrap().as_secs())
//}

impl Pomo {

    pub fn new(minutes: u64) -> Pomo {
        let seconds = Duration::new(minutes * 60, 0);
        let pomo = Pomo {
                           time_left: Some(seconds),
                           start_time: None,
                           display_time: seconds.as_secs(),
                           done: false };
        return pomo;
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn check_finished(&mut self) {
        let elapsed = self.start_time.unwrap().elapsed();
        match elapsed {
            elapsed if  elapsed < self.time_left.unwrap() => {
                self.display_remaining_time(self.time_left.unwrap().as_secs() - elapsed.as_secs());
            },
            _ => {
                println!("All done");
            }

            }
        }

    fn display_remaining_time(&mut self, currently_left: u64) {
        match currently_left {
            currently_left if currently_left != self.display_time => {
                self.display_time = currently_left;
                print!("{}", self.display_time);
            },
            _ => {},
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
