/// Frequency values used for timing packets
#[derive(Clone, Copy, Default)]
pub struct Frequency {
    /// The Mini Time Counter (MTC) frequency as defined in IA32_RTIT_CTL.MTCFreq.
    ///
    /// This field is ignored by the packet encoder and packet decoder. It is
    /// required for other decoders if Mini Time Counter (MTC) packets are enabled
    /// in the collected trace.
    mtc: u8,

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
    nom: u8,

    /// The value of ebx on a cpuid call for leaf 0x15.
    ///
    /// The value *ebx/eax* gives the ratio of the Core Crystal Clock (CTC) to
    /// Timestamp Counter (TSC) frequency.
    ///
    /// This field is ignored by the packet encoder and packet decoder. It is
    /// required for other decoders if Mini Time Counter (MTC) packets are enabled
    /// in the collected trace.
    ctc: u32,

    /// The value of eax on a cpuid call for leaf 0x15.
    ///
    /// The value *ebx/eax* gives the ratio of the Core Crystal Clock (CTC) to
    /// Timestamp Counter (TSC) frequency.
    ///
    /// This field is ignored by the packet encoder and packet decoder. It is
    /// required for other decoders if Mini Time Counter (MTC) packets are enabled
    /// in the collected trace.
    tsc: u32
}

impl Frequency {
    /// Initialize frequency values used in timing packets
    /// * `mtc` - The Mini Time Counter (MTC) frequency as defined in IA32_RTIT_CTL.MTCFreq
    /// * `nom` - The nominal or max non-turbo frequency
    /// * `ctc` - The value of ebx on a cpuid call for leaf 0x15
    /// * `tsc` - The value of eax on a cpuid call for leaf 0x15
    pub fn new(mtc: u8, nom: u8, ctc: u32, tsc: u32) -> Self {
        Frequency {mtc, nom, ctc, tsc}
    }

    /// The Mini Time Counter (MTC) frequency as defined in IA32_RTIT_CTL.MTCFreq
    pub fn mtc(self) -> u8 { self.mtc }
    /// The nominal or max non-turbo frequency
    pub fn nom(self) -> u8 { self.nom }
    /// The value of ebx on a cpuid call for leaf 0x15
    pub fn ctc(self) -> u32 { self.ctc }
    /// The value of eax on a cpuid call for leaf 0x15
    pub fn tsc(self) -> u32 { self.tsc }
}