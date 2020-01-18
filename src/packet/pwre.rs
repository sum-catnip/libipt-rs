use libipt_sys::{
    pt_packet_pwre,
    pt_packet_type_ppt_pwre,
    __BindgenBitfieldUnit
};

/// A PWRE packet.
/// Packet: pwre
#[derive(Clone, Copy, Debug)]
pub struct Pwre (pt_packet_pwre);
impl Pwre {
    #[inline]
    pub fn new(state: u8, substate: u8, hw: bool) -> Self {
        Pwre(pt_packet_pwre{
            state, sub_state: substate,
            _bitfield_1: __BindgenBitfieldUnit::new([hw as u8]),
            __bindgen_padding_0: Default::default()
        })
    }

    /// The resolved thread C-state
    #[inline]
    pub fn state(self) -> u8 { self.0.state }

    /// The resolved thread C-state
    #[inline]
    pub fn set_state(&mut self, state: u8) { self.0.state = state }

    /// The resolved thread sub C-state
    #[inline]
    pub fn substate(self) -> u8 { self.0.sub_state }

    /// The resolved thread sub C-state
    #[inline]
    pub fn set_substate(&mut self, substate: u8) { self.0.sub_state = substate }

    /// A flag indicating whether the C-state entry was initiated by h/w
    #[inline]
    pub fn hw(self) -> bool { self.0.hw() > 0 }

    /// A flag indicating whether the C-state entry was initiated by h/w
    #[inline]
    pub fn set_hw(&mut self, hw: bool) { self.0.set_hw(hw as u32) }
}

wrap2raw!(Pwre, pt_packet_type_ppt_pwre, pwre);
raw2wrap!(Pwre, Pwre, pt_packet_pwre);