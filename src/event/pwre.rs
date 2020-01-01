use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_14;

/// A power state was entered
#[derive(Clone, Copy)] 
pub struct Pwre(pub(super) pt_event__bindgen_ty_1__bindgen_ty_14);
impl Pwre {
    /// The resolved thread C-state.
    pub fn state(self) -> u8 { self.0.state }
    /// The resolved thread sub C-state
    pub fn sub_state(self) -> u8 { self.0.sub_state }
    /// A flag indicating whether the C-state entry was
    /// initiated by h/w.
    pub fn hw(self) -> bool { self.0.hw() > 0 }
}