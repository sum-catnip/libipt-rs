use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_9;

/// A transactional execution state change
#[derive(Clone, Copy)]
pub struct Tsx(pub(super) pt_event__bindgen_ty_1__bindgen_ty_9);
impl Tsx {
    /// The address at which the event is effective.
    ///
    /// This field is not valid if @ip_suppressed is set.
    pub fn ip(self) -> u64 { self.0.ip }

    /// A flag indicating speculative execution mode
    pub fn speculative(self) -> bool { self.0.speculative() > 0 }

    /// A flag indicating speculative execution aborts
    pub fn aborted(self) -> bool { self.0.aborted() > 0 }
}