use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_15;

/// A power state was exited
#[derive(Clone, Copy)]
pub struct Pwrx(pub(super) pt_event__bindgen_ty_1__bindgen_ty_15);
impl Pwrx {
    /// The core C-state at the time of the wake.
    pub fn last(self) -> u8 { self.0.last }
    /// The deepest core C-state achieved during sleep.
    pub fn deepest(self) -> u8 { self.0.deepest }
    /// The wake reason:
    ///
    /// - due to external interrupt received.
    pub fn interrupt(self) -> bool { self.0.interrupt() > 0 }
    /// The wake reason:
    ///
    /// - due to store to monitored address.
    pub fn store(self) -> bool { self.0.store() > 0 }
    /// The wake reason:
    ///
    /// - due to h/w autonomous condition such as HDC.
    pub fn autonomous(self) -> bool { self.0.autonomous() > 0 }
}