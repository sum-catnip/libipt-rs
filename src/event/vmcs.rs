use libipt_sys::{
    pt_event__bindgen_ty_1__bindgen_ty_10,
    pt_event__bindgen_ty_1__bindgen_ty_11
};

/// A synchronous vmcs event
#[derive(Clone, Copy)]
pub struct Vmcs(pub(super) pt_event__bindgen_ty_1__bindgen_ty_10);
impl Vmcs {
    /// The VMCS base address.
    /// 
    /// The address is zero-extended with the lower 12 bits all zero
    pub fn base(self) -> u64 { self.0.base }
}

/// An asynchronous vmcs event
#[derive(Clone, Copy)]
pub struct AsyncVmcs(pub(super) pt_event__bindgen_ty_1__bindgen_ty_11);
impl AsyncVmcs {
    /// The VMCS base address.
    ///
    /// The address is zero-extended with the lower 12 bits all zero
    pub fn base(self) -> u64 { self.0.base }

    /// The address at which the event is effective.
    pub fn ip(self) -> u64 { self.0.ip }
}