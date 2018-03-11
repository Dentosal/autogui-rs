use std::marker::Sized;
use std::thread::sleep;
use std::time::Duration;

use action::InputAction;

#[cfg(target_os = "macos")]
use crate::platform::macos;

pub(crate) trait Actor where Self: Sized {
    fn event(self, t: InputAction) -> Self {
        #[cfg(target_os = "macos")]
        macos::process_event(t, None);
        self
    }

    /// Sleeps given amount of time
    fn delay(self, d: Duration) -> Self {
        sleep(d);
        self
    }
}
