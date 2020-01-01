use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_12;

/// Execution has stopped
#[derive(Clone, Copy)]
pub struct Exstop(pub(super) pt_event__bindgen_ty_1__bindgen_ty_12);
impl Exstop {
    /// The address at which execution has stopped. This is the last instruction that did not complete.
    ///
    /// This field is not valid, if @ip_suppressed is set.
    pub fn ip(self) -> u64 { self.0.ip }
}