use libipt_sys::pt_event__bindgen_ty_1__bindgen_ty_4;

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Payload;
    use std::mem;
    use libipt_sys::{ pt_event, pt_event_type_ptev_async_branch };

    #[test]
    fn test_branch_async_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_async_branch;
        evt.variant.async_branch = pt_event__bindgen_ty_1__bindgen_ty_4 {
           from: 1,
           to: 2
        };

        let payload: Payload = evt.into();
        match payload {
            Payload::AsyncBranch(e) => {
                assert_eq!(e.from(), 1);
                assert_eq!(e.to(), 2);
            },
            _ => unreachable!("oof")
        }
    }
}

/// An asynchronous branch, e.g. interrupt
#[derive(Clone, Copy, Debug)]
pub struct AsyncBranch(pub(super) pt_event__bindgen_ty_1__bindgen_ty_4);
impl AsyncBranch {
    /// The branch source address
    pub fn from(self) -> u64 { self.0.from }
    /// The branch destination address.
    /// This field is not valid if @ip_suppressed is set.
    pub fn to(self) -> u64 { self.0.to }
}