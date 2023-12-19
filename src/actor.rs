use std::marker::Sized;
use std::thread::sleep;
use std::time::Duration;

use action::InputAction;

use crate::platform;

pub(crate) trait Actor
where
    Self: Sized,
{
    fn event(self, t: InputAction) -> Self {
        platform::current::process_event(t, None);
        self
    }
}

/// Methods to help working with chained actions
pub trait ChainedAction
where
    Self: Sized,
{
    /// Sleeps given amount of time
    fn delay(self, d: Duration) -> Self {
        sleep(d);
        self
    }
}
