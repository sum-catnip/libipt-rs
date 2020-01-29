use libipt_sys::{
    pt_event,
    pt_event_type_ptev_async_branch as PT_EVENT_TYPE_PTEV_ASYNC_BRANCH,
    pt_event_type_ptev_async_disabled as PT_EVENT_TYPE_PTEV_ASYNC_DISABLED,
    pt_event_type_ptev_async_paging as PT_EVENT_TYPE_PTEV_ASYNC_PAGING,
    pt_event_type_ptev_async_vmcs as PT_EVENT_TYPE_PTEV_ASYNC_VMCS,
    pt_event_type_ptev_cbr as PT_EVENT_TYPE_PTEV_CBR,
    pt_event_type_ptev_disabled as PT_EVENT_TYPE_PTEV_DISABLED,
    pt_event_type_ptev_enabled as PT_EVENT_TYPE_PTEV_ENABLED,
    pt_event_type_ptev_exec_mode as PT_EVENT_TYPE_PTEV_EXEC_MODE,
    pt_event_type_ptev_exstop as PT_EVENT_TYPE_PTEV_EXSTOP,
    pt_event_type_ptev_mnt as PT_EVENT_TYPE_PTEV_MNT,
    pt_event_type_ptev_mwait as PT_EVENT_TYPE_PTEV_MWAIT,
    pt_event_type_ptev_overflow as PT_EVENT_TYPE_PTEV_OVERFLOW,
    pt_event_type_ptev_paging as PT_EVENT_TYPE_PTEV_PAGING,
    pt_event_type_ptev_ptwrite as PT_EVENT_TYPE_PTEV_PTWRITE,
    pt_event_type_ptev_pwre as PT_EVENT_TYPE_PTEV_PWRE,
    pt_event_type_ptev_pwrx as PT_EVENT_TYPE_PTEV_PWRX,
    pt_event_type_ptev_stop as PT_EVENT_TYPE_PTEV_STOP,
    pt_event_type_ptev_tick as PT_EVENT_TYPE_PTEV_TICK,
    pt_event_type_ptev_tsx as PT_EVENT_TYPE_PTEV_TSX,
    pt_event_type_ptev_vmcs as PT_EVENT_TYPE_PTEV_VMCS
};

mod enabled;
pub use enabled::*;
mod disabled;
pub use disabled::*;
mod branch;
pub use branch::*;
mod paging;
pub use paging::*;
mod overflow;
pub use overflow::*;
mod exec_mode;
pub use exec_mode::*;
mod tsx;
pub use tsx::*;
mod vmcs;
pub use vmcs::*;
mod exstop;
pub use exstop::*;
mod mwait;
pub use mwait::*;
mod pwre;
pub use pwre::*;
mod pwrx;
pub use pwrx::*;
mod ptwrite;
pub use ptwrite::*;
mod tick;
pub use tick::*;
mod mnt;
pub use mnt::*;
mod cbr;
pub use cbr::*;

mod qry;
pub use qry::*;

#[cfg(test)]
mod test {
    use super::*;
    use std::mem;
    use libipt_sys::pt_event_type_ptev_stop;

    #[test]
    fn test_create_event() {
        let evt = pt_event {
            type_: pt_event_type_ptev_stop,
            tsc: 1,
            lost_mtc: 2,
            lost_cyc: 3,
            _bitfield_1: pt_event::new_bitfield_1(1, 0, 1),
            variant: unsafe { mem::zeroed() },
            reserved: [0; 2]
        };

        let evt = Event(evt);
        assert!(evt.ip_suppressed());
        assert!(!evt.status_update());
        assert!(evt.has_tsc());

        assert_eq!(evt.tsc(), 1);
        assert_eq!(evt.lost_mtc(), 2);
        assert_eq!(evt.lost_cyc(), 3);

        match evt.payload() {
            Payload::Stop => (),
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub enum Payload {
    Enabled(Enabled),
    Disabled(Disabled),
    AsnycDisabled(AsyncDisabled),
    AsyncBranch(AsyncBranch),
    Paging(Paging),
    AsyncPaging(AsyncPaging),
    Overflow(Overflow),
    ExecMode(ExecMode),
    Tsx(Tsx),
    Vmcs(Vmcs),
    AsyncVmcs(AsyncVmcs),
    Exstop(Exstop),
    Mwait(Mwait),
    Pwre(Pwre),
    Pwrx(Pwrx),
    Ptwrite(Ptwrite),
    Tick(Tick),
    Mnt(Mnt),
    Cbr(Cbr),
    Stop
}

impl From<pt_event> for Payload {
    fn from(evt: pt_event) -> Payload {
        unsafe {
            match evt.type_ {
                PT_EVENT_TYPE_PTEV_ASYNC_BRANCH => Payload::AsyncBranch(AsyncBranch(evt.variant.async_branch)),
                PT_EVENT_TYPE_PTEV_ASYNC_DISABLED => Payload::AsnycDisabled(AsyncDisabled(evt.variant.async_disabled)),
                PT_EVENT_TYPE_PTEV_ASYNC_PAGING => Payload::AsyncPaging(AsyncPaging(evt.variant.async_paging)),
                PT_EVENT_TYPE_PTEV_ASYNC_VMCS => Payload::AsyncVmcs(AsyncVmcs(evt.variant.async_vmcs)),
                PT_EVENT_TYPE_PTEV_CBR => Payload::Cbr(Cbr(evt.variant.cbr)),
                PT_EVENT_TYPE_PTEV_DISABLED => Payload::Disabled(Disabled(evt.variant.disabled)),
                PT_EVENT_TYPE_PTEV_ENABLED => Payload::Enabled(Enabled(evt.variant.enabled)),
                PT_EVENT_TYPE_PTEV_EXEC_MODE => Payload::ExecMode(ExecMode(evt.variant.exec_mode)),
                PT_EVENT_TYPE_PTEV_EXSTOP => Payload::Exstop(Exstop(evt.variant.exstop)),
                PT_EVENT_TYPE_PTEV_MNT => Payload::Mnt(Mnt(evt.variant.mnt)),
                PT_EVENT_TYPE_PTEV_MWAIT => Payload::Mwait(Mwait(evt.variant.mwait)),
                PT_EVENT_TYPE_PTEV_OVERFLOW => Payload::Overflow(Overflow(evt.variant.overflow)),
                PT_EVENT_TYPE_PTEV_PAGING => Payload::Paging(Paging(evt.variant.paging)),
                PT_EVENT_TYPE_PTEV_PTWRITE => Payload::Ptwrite(Ptwrite(evt.variant.ptwrite)),
                PT_EVENT_TYPE_PTEV_PWRE => Payload::Pwre(Pwre(evt.variant.pwre)),
                PT_EVENT_TYPE_PTEV_PWRX => Payload::Pwrx(Pwrx(evt.variant.pwrx)),
                PT_EVENT_TYPE_PTEV_TICK => Payload::Tick(Tick(evt.variant.tick)),
                PT_EVENT_TYPE_PTEV_TSX => Payload::Tsx(Tsx(evt.variant.tsx)),
                PT_EVENT_TYPE_PTEV_VMCS => Payload::Vmcs(Vmcs(evt.variant.vmcs)),
                PT_EVENT_TYPE_PTEV_STOP => Payload::Stop,
                _ => unreachable!()
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Event(pub(crate) pt_event);
impl Event {
    /// A flag indicating that the event IP has been suppressed.
    pub fn ip_suppressed(self) -> bool { self.0.ip_suppressed() > 0 }
    /// A flag indicating that the event is for status update.
    pub fn status_update(self) -> bool { self.0.status_update() > 0 }
    /// A flag indicating that the event has timing information.
    pub fn has_tsc(self) -> bool { self.0.has_tsc() > 0 }
    /// The time stamp count of the event.
    /// This field is only valid if \@has_tsc is set.
    pub fn tsc(self) -> u64 { self.0.tsc }
    /// The number of lost mtc packets.
    ///
    /// This gives an idea about the quality of the \@tsc.
    /// The more packets were dropped, the less precise timing is.
    pub fn lost_mtc(self) -> u32 { self.0.lost_mtc }
    /// The number of lost cyc packets.
    ///
    /// This gives an idea about the quality of the \@tsc.
    /// The more packets were dropped, the less precise timing is.
    pub fn lost_cyc(self) -> u32 { self.0.lost_cyc }
    /// Event specific data.
    pub fn payload(self) -> Payload { self.0.into() }
}