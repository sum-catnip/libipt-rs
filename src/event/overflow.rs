use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_7;

/// Trace overflow
#[derive(Clone, Copy)] 
pub struct Overflow(pub(super) pt_event__bindgen_ty_1__bindgen_ty_7);
impl Overflow {
    /// The address at which tracing resumes after overflow.
    ///
    /// This field is not valid, if ip_suppressed is set.
    /// In this case, the overflow resolved while tracing was disabled.
    pub fn ip(self) -> u64 { self.0.ip }
}