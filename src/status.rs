// Certain casts are required only on Windows. Inform Clippy to ignore them.
#![allow(clippy::unnecessary_cast)]

use bitflags::bitflags;
use libipt_sys::{
    pt_status_flag_pts_eos, pt_status_flag_pts_event_pending, pt_status_flag_pts_ip_suppressed,
};

bitflags! {
    /// Status flags for various IntelPT actions
    #[derive(Debug, Clone, Copy)]
    pub struct Status: u32 {
        /// There is an event pending.
        const EVENT_PENDING = pt_status_flag_pts_event_pending as u32;
        /// The address has been suppressed.
        const IP_SUPRESSED  = pt_status_flag_pts_ip_suppressed as u32;
        /// There is no more trace data available.
        const EOS = pt_status_flag_pts_eos as u32;
    }
}

impl Status {
    /// There is an event pending.
    #[must_use]
    pub const fn event_pending(&self) -> bool {
        self.contains(Self::EVENT_PENDING)
    }

    /// The address has been suppressed.
    #[must_use]
    pub const fn ip_supressed(&self) -> bool {
        self.contains(Self::IP_SUPRESSED)
    }

    /// There is no more trace data available.
    #[must_use]
    pub const fn eos(&self) -> bool {
        self.contains(Self::EOS)
    }
}
