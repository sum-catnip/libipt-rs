use libipt_sys::{
    pt_event__bindgen_ty_1__bindgen_ty_2,
    pt_event__bindgen_ty_1__bindgen_ty_3
};

/// Tracing has been disabled
#[derive(Clone, Copy)]
pub struct Disabled(pub(super) pt_event__bindgen_ty_1__bindgen_ty_2);
impl Disabled {
    /// The destination of the first branch inside a
    /// filtered area.
    ///
    /// This field is not valid if \@ip_suppressed is set.
    pub fn ip(self) -> u64 { self.0.ip }
}

/// Tracing has been disabled asynchronously
#[derive(Clone, Copy)]
pub struct AsyncDisabled(pub(super) pt_event__bindgen_ty_1__bindgen_ty_3);
impl AsyncDisabled {
    /// The source address of the asynchronous branch that disabled tracing
    pub fn at(self) -> u64 { self.0.at }
    /// The destination of the first branch inside a filtered area.
    /// This field is not valid if @ip_suppressed is set.
    pub fn ip(self) -> u64 { self.0.ip }

}