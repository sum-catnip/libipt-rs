use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_17;

/// A timing event
#[derive(Clone, Copy)]
pub struct Tick(pub(super) pt_event__bindgen_ty_1__bindgen_ty_17);
impl Tick {
    /// The instruction address near which the tick occured.
    ///
    /// A timestamp can sometimes be attributed directly to
    /// an instruction (e.g. to an indirect branch that
    /// receives CYC + TIP) and sometimes not (e.g. MTC).
    ///
    /// This field is not valid, if \@ip_suppressed is set.
    pub fn ip(self) -> u64 { self.0.ip }
}