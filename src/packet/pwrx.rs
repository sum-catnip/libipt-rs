use libipt_sys::{pt_packet_pwrx, pt_packet_type_ppt_pwrx};

/// A PWRX packet.
/// Packet: pwrx
#[derive(Clone, Copy, Debug)]
pub struct Pwrx(pt_packet_pwrx);
impl Pwrx {
    #[inline]
    #[must_use]
    pub fn new(last: u8, deepest: u8, interrupt: bool, store: bool, autonomous: bool) -> Self {
        Pwrx(pt_packet_pwrx {
            last,
            deepest,
            __bindgen_padding_0: Default::default(),
            _bitfield_1: pt_packet_pwrx::new_bitfield_1(
                interrupt as u32,
                store as u32,
                autonomous as u32,
            ),
            _bitfield_align_1: [],
        })
    }

    /// The core C-state at the time of the wake
    #[inline]
    #[must_use]
    pub fn last(&self) -> u8 {
        self.0.last
    }

    /// The core C-state at the time of the wake
    #[inline]
    pub fn set_last(&mut self, last: u8) {
        self.0.last = last
    }

    /// The deepest core C-state achieved during sleep
    #[inline]
    #[must_use]
    pub fn deepest(&self) -> u8 {
        self.0.deepest
    }

    /// The deepest core C-state achieved during sleep
    #[inline]
    pub fn set_deepest(&mut self, deepest: u8) {
        self.0.deepest = deepest;
    }

    /// The wake reason:
    /// due to external interrupt received.
    #[inline]
    #[must_use]
    pub fn interrupt(&self) -> bool {
        self.0.interrupt() > 0
    }

    /// The wake reason:
    /// due to external interrupt received.
    #[inline]
    pub fn set_interrupt(&mut self, interrupt: bool) {
        self.0.set_interrupt(interrupt as u32);
    }

    /// The wake reason:
    /// due to store to monitored address
    #[inline]
    pub fn store(self) -> bool {
        self.0.store() > 0
    }

    /// The wake reason:
    /// due to store to monitored address
    #[inline]
    pub fn set_store(&mut self, store: bool) {
        self.0.set_store(store as u32);
    }

    /// The wake reason:
    /// due to h/w autonomous condition such as HDC
    #[inline]
    #[must_use]
    pub fn autonomous(&self) -> bool {
        self.0.autonomous() > 0
    }

    /// The wake reason:
    /// due to h/w autonomous condition such as HDC
    #[inline]
    pub fn set_autonomous(&mut self, autonomous: bool) {
        self.0.set_autonomous(autonomous as u32);
    }
}

wrap2raw!(Pwrx, pt_packet_type_ppt_pwrx, pwrx);
raw2wrap!(Pwrx, Pwrx, pt_packet_pwrx);
