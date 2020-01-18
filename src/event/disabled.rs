use libipt_sys::{
    pt_event__bindgen_ty_1__bindgen_ty_2,
    pt_event__bindgen_ty_1__bindgen_ty_3
};

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Payload;
    use std::mem;
    use libipt_sys::{
        pt_event,
        pt_event_type_ptev_disabled,
        pt_event_type_ptev_async_disabled
    };

    #[test]
    fn test_disabled_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_disabled;
        evt.variant.disabled = pt_event__bindgen_ty_1__bindgen_ty_2 {
            ip: 11,
        };

        let payload: Payload = evt.into();
        match payload {
            Payload::Disabled(e) => {
                assert_eq!(e.ip(), 11);
            },
            _ => unreachable!("oof")
        }
    }

    #[test]
    fn test_async_disabled_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_async_disabled;
        evt.variant.async_disabled = pt_event__bindgen_ty_1__bindgen_ty_3 {
            at: 1,
            ip: 11,
        };

        let payload: Payload = evt.into();
        match payload {
            Payload::AsnycDisabled(e) => {
                assert_eq!(e.ip(), 11);
                assert_eq!(e.at(), 1);
            },
            _ => unreachable!("oof")
        }
    }
}

/// Tracing has been disabled
#[derive(Clone, Copy, Debug)]
pub struct Disabled(pub(super) pt_event__bindgen_ty_1__bindgen_ty_2);
impl Disabled {
    /// The destination of the first branch inside a
    /// filtered area.
    ///
    /// This field is not valid if \@ip_suppressed is set.
    pub fn ip(self) -> u64 { self.0.ip }
}

/// Tracing has been disabled asynchronously
#[derive(Clone, Copy, Debug)]
pub struct AsyncDisabled(pub(super) pt_event__bindgen_ty_1__bindgen_ty_3);
impl AsyncDisabled {
    /// The source address of the asynchronous branch that disabled tracing
    pub fn at(self) -> u64 { self.0.at }
    /// The destination of the first branch inside a filtered area.
    /// This field is not valid if @ip_suppressed is set.
    pub fn ip(self) -> u64 { self.0.ip }

}