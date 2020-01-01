use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_1;

/// Tracing has been enabled
#[derive(Clone, Copy)]
pub struct Enabled(pub(super) pt_event__bindgen_ty_1__bindgen_ty_1);
impl Enabled {
    /// The address at which tracing resumes
    pub fn ip(self) -> u64 { self.0.ip }

    /// A flag indicating that tracing resumes from the IP
    /// at which tracing had been disabled before.
    pub fn resumed(self) -> bool { self.0.resumed() > 0 }
}