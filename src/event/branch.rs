use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_4;

/// An asynchronous branch, e.g. interrupt
#[derive(Clone, Copy)]
pub struct AsyncBranch(pub(super) pt_event__bindgen_ty_1__bindgen_ty_4);
impl AsyncBranch {
    /// The branch source address
    pub fn from(self) -> u64 { self.0.from }
    /// The branch destination address.
    /// This field is not valid if @ip_suppressed is set.
    pub fn to(self) -> u64 { self.0.to }
}