use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_18;

/// A core:bus ratio event
#[derive(Clone, Copy)]
pub struct Cbr(pub(super) pt_event__bindgen_ty_1__bindgen_ty_18);
impl Cbr {
    /// The core:bus ratio.
    pub fn ratio(self) -> u16 { self.0.ratio }
}