use libipt_sys::{
    pt_event__bindgen_ty_1__bindgen_ty_5,
    pt_event__bindgen_ty_1__bindgen_ty_6
};

#[cfg(test)]
mod test {
    use super::*;
    use super::super::Payload;
    use std::mem;
    use libipt_sys::{
        pt_event,
        pt_event_type_ptev_paging,
        pt_event_type_ptev_async_paging
    };

    #[test]
    fn test_paging_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_paging;
        evt.variant.paging = pt_event__bindgen_ty_1__bindgen_ty_5 {
            cr3: 11,
            _bitfield_1: pt_event__bindgen_ty_1__bindgen_ty_5::new_bitfield_1(1),
            __bindgen_padding_0: Default::default()
        };

        let payload: Payload = evt.into();
        match payload {
            Payload::Paging(e) => {
                assert_eq!(e.cr3(), 11);
                assert!(e.non_root());
            },
            _ => unreachable!("oof")
        }
    }

    #[test]
    fn test_async_paging_payload() {
        let mut evt: pt_event = unsafe { mem::zeroed() };
        evt.type_ = pt_event_type_ptev_async_paging;
        evt.variant.async_paging = pt_event__bindgen_ty_1__bindgen_ty_6 {
            cr3: 11,
            ip: 12,
            _bitfield_1: pt_event__bindgen_ty_1__bindgen_ty_6::new_bitfield_1(1)
        };

        let payload: Payload = evt.into();
        match payload {
            Payload::AsyncPaging(e) => {
                assert_eq!(e.cr3(), 11);
                assert_eq!(e.ip(), 12);
                assert!(e.non_root());
            },
            _ => unreachable!("oof")
        }
    }
}

/// A synchronous paging event
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
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