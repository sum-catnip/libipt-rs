use libipt_sys::{
    pt_event__bindgen_ty_1__bindgen_ty_5,
    pt_event__bindgen_ty_1__bindgen_ty_6
};

/// A synchronous paging event
#[derive(Clone, Copy)]
pub struct Paging(pub(super) pt_event__bindgen_ty_1__bindgen_ty_5);
impl Paging {
    /// The updated CR3 value.
    /// The lower 5 bit have been zeroed out.
    /// The upper bits have been zeroed out depending on the maximum possible address.
    pub fn cr3(self) -> u64 { self.0.cr3 }

    /// A flag indicating whether the cpu is operating in
    /// vmx non-root (guest) mode.
    pub fn non_root(self) -> bool { self.0.non_root() > 0 }
}

/// An asynchronous paging event
#[derive(Clone, Copy)]
pub struct AsyncPaging(pub(super) pt_event__bindgen_ty_1__bindgen_ty_6);
impl AsyncPaging {
    /// The updated CR3 value.
    ///
    /// The lower 5 bit have been zeroed out.
    /// The upper bits have been zeroed out depending on the
    /// maximum possible address.
    pub fn cr3(self) -> u64 { self.0.cr3 }
    /// A flag indicating whether the cpu is operating in
    /// vmx non-root (guest) mode.
    pub fn non_root(self) -> bool { self.0.non_root() > 0 }
    /// The address at which the event is effective
    pub fn ip(self) -> u64 { self.0.ip }
}