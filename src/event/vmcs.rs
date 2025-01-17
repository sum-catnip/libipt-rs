use libipt_sys::{pt_event__bindgen_ty_1__bindgen_ty_10, pt_event__bindgen_ty_1__bindgen_ty_11};

/// A synchronous vmcs event
#[derive(Clone, Copy, Debug)]
pub struct Vmcs(pub(super) pt_event__bindgen_ty_1__bindgen_ty_10);
impl Vmcs {
    /// The VMCS base address.
    ///
    /// The address is zero-extended with the lower 12 bits all zero
    #[must_use]
    pub fn base(self) -> u64 {
        self.0.base
    }
}

/// An asynchronous vmcs event
#[derive(Clone, Copy, Debug)]
pub struct AsyncVmcs(pub(super) pt_event__bindgen_ty_1__bindgen_ty_11);
impl AsyncVmcs {
    /// The VMCS base address.
    ///
    /// The address is zero-extended with the lower 12 bits all zero
    #[must_use]
    pub fn base(self) -> u64 {
        self.0.base
    }

    /// The address at which the event is effective.
    #[must_use]
    pub fn ip(self) -> u64 {
        self.0.ip
    }
}

#[cfg(test)]
mod test {
    use super::super::Payload;
    use super::*;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_async_vmcs, pt_event_type_ptev_vmcs};
    use std::mem;

    #[test]
    fn test_vmcs_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_vmcs;
        evt.variant.vmcs = pt_event__bindgen_ty_1__bindgen_ty_10 { base: 11 };

        let payload: Payload = Event(evt).into();
        match payload {
            Payload::Vmcs(e) => {
                assert_eq!(e.base(), 11);
            }
            _ => unreachable!("oof"),
        }
    }

    #[test]
    fn test_async_vmcs_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_async_vmcs;
        evt.variant.async_vmcs = pt_event__bindgen_ty_1__bindgen_ty_11 { base: 11, ip: 12 };

        let payload: Payload = Event(evt).into();
        match payload {
            Payload::AsyncVmcs(e) => {
                assert_eq!(e.base(), 11);
                assert_eq!(e.ip(), 12);
            }
            _ => unreachable!("oof"),
        }
    }
}
