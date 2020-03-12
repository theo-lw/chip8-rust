/// Struct representing the timers of a chip-8 machine
#[derive(Default)]
pub struct Timers {
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Timers {
    pub fn new() -> Timers {
        Default::default()
    }

    /// Decrements each timer if they are positive
    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrement_timers() {
        let mut timers = Timers::new();
        timers.delay_timer = 3;
        timers.sound_timer = 2;
        timers.decrement_timers();
        assert_eq!(2, timers.delay_timer);
        assert_eq!(1, timers.sound_timer);
        timers.decrement_timers();
        assert_eq!(1, timers.delay_timer);
        assert_eq!(0, timers.sound_timer);
        timers.decrement_timers();
        assert_eq!(0, timers.delay_timer);
        assert_eq!(0, timers.sound_timer);
    }
}
