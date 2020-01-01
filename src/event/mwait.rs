use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_13;

/// An MWAIT operation completed
#[derive(Clone, Copy)]
pub struct Mwait(pub(super) pt_event__bindgen_ty_1__bindgen_ty_13);
impl Mwait {
    /// The address of the instruction causing the mwait.
    ///
    /// This field is not valid, if @ip_suppressed is set.
    pub fn ip(self) -> u64 { self.0.ip }

    /// The mwait hints (eax).
    ///
    /// Reserved bits are undefined.
    pub fn hints(self) -> u32 { self.0.hints }

    /// The mwait extensions (ecx).
    ///
    /// Reserved bits are undefined.
    pub fn ext(self) -> u32 { self.0.ext }
}