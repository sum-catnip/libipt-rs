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

pub mod enabled;
pub use enabled::*;
pub mod disabled;
pub use disabled::*;
pub mod branch;
pub use branch::*;
pub mod paging;
pub use paging::*;
pub mod overflow;
pub use overflow::*;
pub mod exec_mode;
pub use exec_mode::*;
pub mod tsx;
pub use tsx::*;
pub mod vmcs;
pub use vmcs::*;
pub mod exstop;
pub use exstop::*;
pub mod mwait;
pub use mwait::*;
pub mod pwre;
pub use pwre::*;
pub mod pwrx;
pub use pwrx::*;
pub mod ptwrite;
pub use ptwrite::*;
pub mod tick;
pub use tick::*;
pub mod mnt;
pub use mnt::*;
pub mod cbr;
pub use cbr::*;

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