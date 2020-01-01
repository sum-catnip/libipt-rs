use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_16;

/// A PTWRITE event.
pub struct Ptwrite(pub(super) pt_event__bindgen_ty_1__bindgen_ty_16);
impl Ptwrite {
    /// The address of the ptwrite instruction.
    ///
    /// This field is not valid, if \@ip_suppressed is set.
    /// In this case, the address is obvious from the disassembly.
    pub fn ip(self) -> u64 { self.0.ip }
    /// The size of the below \@payload in bytes.
    pub fn size(self) -> u8{ self.0.size }
    /// The ptwrite payload.
    pub fn payload(self) -> u64 { self.0.payload }
}