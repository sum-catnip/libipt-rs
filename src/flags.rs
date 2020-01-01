use libipt_sys::{
    pt_status_flag_pts_eos,
    pt_status_flag_pts_event_pending,
    pt_status_flag_pts_ip_suppressed
};
use bitflags::bitflags;

bitflags! {
    pub struct Status: u32 {
        const EOS = pt_status_flag_pts_eos as u32;
        const EVENT_PENDING = pt_status_flag_pts_event_pending as u32;
        const IP_SUPRESSED  = pt_status_flag_pts_ip_suppressed as u32;
    }
}

impl Status {
    /// is eos bit set?
    pub fn eos(self) -> bool { self.contains(Status::EOS) }
    /// is event_pending bit set?
    pub fn event_pending(self) -> bool { self.contains(Status::EVENT_PENDING) }
    /// is ip_supressed bit set?
    pub fn ip_supressed(self) -> bool { self.contains(Status::IP_SUPRESSED) }
}