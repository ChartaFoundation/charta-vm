/// Timer state management for Charta VM
/// 
/// Timers track elapsed time in scan cycles. Each timer maintains its own state
/// and is updated each cycle.

use std::collections::HashMap;

/// Timer state (τ) - tracks elapsed time for all timers
#[derive(Debug, Clone, Default)]
pub struct TimerState {
    /// Timer instances: timer_id -> TimerInstance
    timers: HashMap<String, TimerInstance>,
}

/// Individual timer instance
#[derive(Debug, Clone)]
pub enum TimerInstance {
    /// TON (Timer On Delay) - delays output activation
    TON {
        elapsed: u64, // Elapsed cycles
        preset: u64,  // Preset cycles
        input: bool,  // Last input state
    },
    /// TOF (Timer Off Delay) - delays output deactivation
    TOF {
        elapsed: u64,
        preset: u64,
        input: bool,
    },
    /// TP (Pulse Timer) - generates fixed-duration pulse
    TP {
        elapsed: u64,
        preset: u64,
        active: bool,
        input: bool,
    },
    /// Watchdog - monitors for timeouts
    Watchdog {
        elapsed: u64,
        timeout: u64,
        last_toggle: u64, // Cycle number of last toggle
        input: bool,
    },
}

impl TimerState {
    /// Create new timer state
    pub fn new() -> Self {
        Self {
            timers: HashMap::new(),
        }
    }

    /// Get timer instance
    pub fn get(&self, timer_id: &str) -> Option<&TimerInstance> {
        self.timers.get(timer_id)
    }

    /// Get mutable timer instance
    pub fn get_mut(&mut self, timer_id: &str) -> Option<&mut TimerInstance> {
        self.timers.get_mut(timer_id)
    }

    /// Create or get TON timer
    pub fn get_or_create_ton(&mut self, timer_id: String, preset: u64) -> &mut TimerInstance {
        self.timers.entry(timer_id).or_insert_with(|| {
            TimerInstance::TON {
                elapsed: 0,
                preset,
                input: false,
            }
        })
    }

    /// Create or get TOF timer
    pub fn get_or_create_tof(&mut self, timer_id: String, preset: u64) -> &mut TimerInstance {
        self.timers.entry(timer_id).or_insert_with(|| {
            TimerInstance::TOF {
                elapsed: 0,
                preset,
                input: false,
            }
        })
    }

    /// Create or get TP timer
    pub fn get_or_create_tp(&mut self, timer_id: String, preset: u64) -> &mut TimerInstance {
        self.timers.entry(timer_id).or_insert_with(|| {
            TimerInstance::TP {
                elapsed: 0,
                preset,
                active: false,
                input: false,
            }
        })
    }

    /// Create or get Watchdog timer
    pub fn get_or_create_watchdog(&mut self, timer_id: String, timeout: u64) -> &mut TimerInstance {
        self.timers.entry(timer_id).or_insert_with(|| {
            TimerInstance::Watchdog {
                elapsed: 0,
                timeout,
                last_toggle: 0,
                input: false,
            }
        })
    }

    /// Update all timers for one cycle
    /// Returns cycle number (incremented each call)
    pub fn update_cycle(&mut self, cycle: u64) {
        // Timers are updated individually when accessed
        // This method can be used for global timer updates if needed
    }
}

impl TimerInstance {
    /// Update TON timer for one cycle
    pub fn update_ton(&mut self, input: bool, cycle: u64) -> (bool, u64) {
        match self {
            TimerInstance::TON { elapsed, preset, input: last_input } => {
                if input && *last_input {
                    // Input still true, increment elapsed
                    *elapsed += 1;
                } else if input && !*last_input {
                    // Rising edge, start timer
                    *elapsed = 1;
                } else if !input {
                    // Input false, reset timer
                    *elapsed = 0;
                }
                
                *last_input = input;
                let done = *elapsed >= *preset;
                (done, *elapsed)
            }
            _ => panic!("Timer type mismatch"),
        }
    }

    /// Update TOF timer for one cycle
    pub fn update_tof(&mut self, input: bool, _cycle: u64) -> (bool, u64) {
        match self {
            TimerInstance::TOF { elapsed, preset, input: last_input } => {
                if !input && !*last_input {
                    // Input still false, increment elapsed
                    *elapsed += 1;
                } else if !input && *last_input {
                    // Falling edge, start timer
                    *elapsed = 1;
                } else if input {
                    // Input true, reset timer and set done
                    *elapsed = 0;
                }
                
                *last_input = input;
                let done = *elapsed < *preset; // Done is true until elapsed >= preset
                (done, *elapsed)
            }
            _ => panic!("Timer type mismatch"),
        }
    }

    /// Update TP timer for one cycle
    pub fn update_tp(&mut self, input: bool, _cycle: u64) -> (bool, u64) {
        match self {
            TimerInstance::TP { elapsed, preset, active, input: last_input } => {
                if input && !*last_input {
                    // Rising edge, start pulse
                    *elapsed = 1;
                    *active = true;
                } else if *active {
                    // Pulse active, increment elapsed
                    *elapsed += 1;
                    if *elapsed >= *preset {
                        *active = false;
                    }
                }
                
                if !input {
                    // Input false, reset
                    *elapsed = 0;
                    *active = false;
                }
                
                *last_input = input;
                (*active, *elapsed)
            }
            _ => panic!("Timer type mismatch"),
        }
    }

    /// Update Watchdog timer for one cycle
    pub fn update_watchdog(&mut self, input: bool, cycle: u64) -> (bool, u64) {
        match self {
            TimerInstance::Watchdog { elapsed, timeout, last_toggle, input: last_input } => {
                if input != *last_input {
                    // Toggle detected, reset
                    *last_toggle = cycle;
                    *elapsed = 0;
                } else {
                    // No toggle, increment elapsed
                    *elapsed = cycle - *last_toggle;
                }
                
                *last_input = input;
                let timed_out = *elapsed >= *timeout;
                (timed_out, *elapsed)
            }
            _ => panic!("Timer type mismatch"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ton_timer() {
        let mut timer = TimerInstance::TON {
            elapsed: 0,
            preset: 5,
            input: false,
        };

        // Input true for 3 cycles
        assert_eq!(timer.update_ton(true, 1), (false, 1));
        assert_eq!(timer.update_ton(true, 2), (false, 2));
        assert_eq!(timer.update_ton(true, 3), (false, 3));
        
        // Input true for 2 more cycles (total 5)
        assert_eq!(timer.update_ton(true, 4), (false, 4));
        assert_eq!(timer.update_ton(true, 5), (true, 5)); // Done!
        
        // Input false, reset
        assert_eq!(timer.update_ton(false, 6), (false, 0));
    }

    #[test]
    fn test_tof_timer() {
        let mut timer = TimerInstance::TOF {
            elapsed: 0,
            preset: 3,
            input: false,
        };

        // Input true, done immediately
        assert_eq!(timer.update_tof(true, 1), (true, 0));
        
        // Input false, start counting
        assert_eq!(timer.update_tof(false, 2), (true, 1));
        assert_eq!(timer.update_tof(false, 3), (true, 2));
        assert_eq!(timer.update_tof(false, 4), (false, 3)); // Done becomes false
    }

    #[test]
    fn test_tp_timer() {
        let mut timer = TimerInstance::TP {
            elapsed: 0,
            preset: 3,
            active: false,
            input: false,
        };

        // Rising edge, start pulse
        assert_eq!(timer.update_tp(true, 1), (true, 1));
        assert_eq!(timer.update_tp(true, 2), (true, 2));
        // At cycle 3, elapsed is 3, which equals preset, so active becomes false
        assert_eq!(timer.update_tp(true, 3), (false, 3)); // Pulse complete
        assert_eq!(timer.update_tp(true, 4), (false, 3)); // Still inactive
    }

    #[test]
    fn test_watchdog_timer() {
        let mut timer = TimerInstance::Watchdog {
            elapsed: 0,
            timeout: 5,
            last_toggle: 0,
            input: false,
        };

        // Toggle input
        assert_eq!(timer.update_watchdog(true, 1), (false, 0));
        
        // No toggle for 4 cycles
        assert_eq!(timer.update_watchdog(true, 2), (false, 1));
        assert_eq!(timer.update_watchdog(true, 3), (false, 2));
        assert_eq!(timer.update_watchdog(true, 4), (false, 3));
        assert_eq!(timer.update_watchdog(true, 5), (false, 4));
        assert_eq!(timer.update_watchdog(true, 6), (true, 5)); // Timeout!
    }
}
