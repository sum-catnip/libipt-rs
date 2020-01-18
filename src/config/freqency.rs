#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_freq_props() {
        let mut freq = Frequency::new(1, 2, 3, 4);
        assert_eq!(freq.mtc(), 1);
        assert_eq!(freq.nom(), 2);
        assert_eq!(freq.ctc(), 3);
        assert_eq!(freq.tsc(), 4);

        freq.set_mtc(5);
        freq.set_nom(6);
        freq.set_ctc(7);
        freq.set_tsc(8);

        assert_eq!(freq.mtc(), 5);
        assert_eq!(freq.nom(), 6);
        assert_eq!(freq.ctc(), 7);
        assert_eq!(freq.tsc(), 8);
    }
}

/// Frequency values used for timing packets
#[derive(Clone, Copy, Default)]
pub struct Frequency {
    /// The Mini Time Counter (MTC) frequency as defined in IA32_RTIT_CTL.MTCFreq.
    ///
    /// This field is ignored by the packet encoder and packet decoder. It is
    /// required for other decoders if Mini Time Counter (MTC) packets are enabled
    /// in the collected trace.
    pub(super) mtc: u8,

    /// The nominal or max non-turbo frequency.
    ///
    /// This field is ignored by the packet encoder and packet decoder. It is
    /// used by other decoders if Cycle Count (CYC) packets are enabled to improve
    /// timing calibration for cycle-accurate tracing.
    ///
    /// If the field is zero, the time tracking algorithm will use Mini Time
    /// Counter (MTC) and Cycle Count (CYC) packets for calibration.
    ///
    /// If the field is non-zero, the time tracking algorithm will additionally be
    /// able to calibrate at Core:Bus Ratio (CBR) packets.
    pub(super) nom: u8,

    /// The value of ebx on a cpuid call for leaf 0x15.
    ///
    /// The value *ebx/eax* gives the ratio of the Core Crystal Clock (CTC) to
    /// Timestamp Counter (TSC) frequency.
    ///
    /// This field is ignored by the packet encoder and packet decoder. It is
    /// required for other decoders if Mini Time Counter (MTC) packets are enabled
    /// in the collected trace.
    pub(super) ctc: u32,

    /// The value of eax on a cpuid call for leaf 0x15.
    ///
    /// The value *ebx/eax* gives the ratio of the Core Crystal Clock (CTC) to
    /// Timestamp Counter (TSC) frequency.
    ///
    /// This field is ignored by the packet encoder and packet decoder. It is
    /// required for other decoders if Mini Time Counter (MTC) packets are enabled
    /// in the collected trace.
    pub(super) tsc: u32
}

impl Frequency {
    /// Initialize frequency values used in timing packets
    /// * `mtc` - The Mini Time Counter (MTC) frequency as defined in IA32_RTIT_CTL.MTCFreq
    /// * `nom` - The nominal or max non-turbo frequency
    /// * `ctc` - The value of ebx on a cpuid call for leaf 0x15
    /// * `tsc` - The value of eax on a cpuid call for leaf 0x15
    #[inline]
    pub fn new(mtc: u8, nom: u8, ctc: u32, tsc: u32) -> Self {
        Frequency {mtc, nom, ctc, tsc}
    }

    /// The Mini Time Counter (MTC) frequency as defined in IA32_RTIT_CTL.MTCFreq.
    ///
    /// This field is ignored by the packet encoder and packet decoder. It is
    /// required for other decoders if Mini Time Counter (MTC) packets are enabled
    /// in the collected trace.
    #[inline]
    pub fn mtc(self) -> u8 { self.mtc }
    /// The nominal or max non-turbo frequency.
    ///
    /// This field is ignored by the packet encoder and packet decoder. It is
    /// used by other decoders if Cycle Count (CYC) packets are enabled to improve
    /// timing calibration for cycle-accurate tracing.
    ///
    /// If the field is zero, the time tracking algorithm will use Mini Time
    /// Counter (MTC) and Cycle Count (CYC) packets for calibration.
    ///
    /// If the field is non-zero, the time tracking algorithm will additionally be
    /// able to calibrate at Core:Bus Ratio (CBR) packets.
    #[inline]
    pub fn nom(self) -> u8 { self.nom }
    /// The value of ebx on a cpuid call for leaf 0x15.
    ///
    /// The value *ebx/eax* gives the ratio of the Core Crystal Clock (CTC) to
    /// Timestamp Counter (TSC) frequency.
    ///
    /// This field is ignored by the packet encoder and packet decoder. It is
    /// required for other decoders if Mini Time Counter (MTC) packets are enabled
    /// in the collected trace.
    #[inline]
    pub fn ctc(self) -> u32 { self.ctc }
    /// The value of eax on a cpuid call for leaf 0x15.
    ///
    /// The value *ebx/eax* gives the ratio of the Core Crystal Clock (CTC) to
    /// Timestamp Counter (TSC) frequency.
    ///
    /// This field is ignored by the packet encoder and packet decoder. It is
    /// required for other decoders if Mini Time Counter (MTC) packets are enabled
    /// in the collected trace.
    #[inline]
    pub fn tsc(self) -> u32 { self.tsc }

    #[inline]
    pub fn set_mtc(&mut self, mtc: u8) { self.mtc = mtc }
    #[inline]
    pub fn set_nom(&mut self, nom: u8) { self.nom = nom }
    #[inline]
    pub fn set_ctc(&mut self, ctc: u32) { self.ctc = ctc }
    #[inline]
    pub fn set_tsc(&mut self, tsc: u32) { self.tsc = tsc }
}