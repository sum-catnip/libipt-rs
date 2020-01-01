use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_19;

/// A maintenance event.
#[derive(Clone, Copy)]
pub struct Mnt(pub(super) pt_event__bindgen_ty_1__bindgen_ty_19);
impl Mnt {
    /// The raw payload.
    pub fn payload(self) -> u64 { self.0.payload }
}