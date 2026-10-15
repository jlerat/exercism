use std::fmt;

pub struct Clock {
    hour: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        
        let hour = match hours {
            0..23 => hours,
            _ => {
                println!("Cannot set {hours}");
                0
            }
        };
        let minutes = match minutes {
            0..59 => minutes,
            _ => {
                println!("Cannot set {minutes}");
                0
            }
        };
        Clock{
            hour: hour,
            minutes: minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let min = (self.minutes + minutes) % 60;
        let hr = (self.hour + (self.minutes + minutes) / 60) / 24;    
        Clock{
            hour: hr,
            minutes: min
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Clock time is {}:{}", self.hour, self.minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock() {
        let clk = Clock::new(20, 55);
        assert!(clk.hour == 20);
        assert!(clk.minutes == 55);
        println!("{}", clk);
        
        let clk = Clock::new(30, 55);
        assert!(clk.hour == 0);
        assert!(clk.minutes == 55);

        let clk = Clock::new(20, 65);
        assert!(clk.hour == 20);
        assert!(clk.minutes == 0);

        let clk = Clock::new(20, 55);
        let clk = clk.add_minutes(5);
        println!("After added 5 min: {}", clk);
        //assert!(clk.hour == 21);
        //assert!(clk.minutes == 0);
    }
}
