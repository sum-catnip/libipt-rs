use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_16;

/// A PTWRITE event.
#[derive(Clone, Copy, Debug)]
pub struct Ptwrite(pub(super) pt_event__bindgen_ty_1__bindgen_ty_16);
impl Ptwrite {
    /// The address of the ptwrite instruction.
    ///
    /// This field is not valid, if @ip_suppressed is set.
    /// In this case, the address is obvious from the disassembly.
    #[must_use]
    pub fn ip(self) -> u64 {
        self.0.ip
    }

    /// The size of the below @payload in bytes.
    #[must_use]
    pub fn size(self) -> u8 {
        self.0.size
    }

    /// The ptwrite payload.
    #[must_use]
    pub fn payload(self) -> u64 {
        self.0.payload
    }
}

#[cfg(test)]
mod test {
    use super::super::Payload;
    use super::*;
    use crate::event::Event;
    use libipt_sys::{pt_event, pt_event_type_ptev_ptwrite};
    use std::mem;

    #[test]
    fn test_ptwrite_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_ptwrite;
        evt.variant.ptwrite = pt_event__bindgen_ty_1__bindgen_ty_16 {
            ip: 11,
            size: 22,
            payload: 33,
        };

        let payload: Payload = Event(evt).into();
        match payload {
            Payload::Ptwrite(e) => {
                assert_eq!(e.ip(), 11);
                assert_eq!(e.size(), 22);
                assert_eq!(e.payload(), 33);
            }
            _ => unreachable!("oof"),
        }
    }
}
