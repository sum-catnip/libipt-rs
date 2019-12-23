use std::ptr;
use libipt_sys::{
    pt_config,
    pt_config__bindgen_ty_1,
    pt_packet_unknown,
    pt_cpu,
    pt_errata,
    __BindgenBitfieldUnit
};
use bitflags::bitflags;

mod test {
    use super::*;

    fn assert_errata_equal(flags: Errata) {
        let raw: pt_errata = flags.into();
        assert_eq!(flags.contains(Errata::BDM70)  as u32, raw.bdm70());
        assert_eq!(flags.contains(Errata::BDM64)  as u32, raw.bdm64());
        assert_eq!(flags.contains(Errata::SKD007) as u32, raw.skd007());
        assert_eq!(flags.contains(Errata::SKD022) as u32, raw.skd022());
        assert_eq!(flags.contains(Errata::SKD010) as u32, raw.skd010());
        assert_eq!(flags.contains(Errata::SKL014) as u32, raw.skl014());
        assert_eq!(flags.contains(Errata::APL12)  as u32, raw.apl12());
        assert_eq!(flags.contains(Errata::APL11)  as u32, raw.apl11());
        assert_eq!(flags.contains(Errata::SKL168) as u32, raw.skl168());
    }

    #[test]
    fn errata_cast() {
        assert_errata_equal(Errata::BDM70);
        assert_errata_equal(Errata::BDM70 | Errata::SKL168);
        assert_errata_equal(Errata::empty());
        assert_errata_equal(Errata::all());
    }
}

pub struct Config<'a> {
    _wrapped: pt_config,
    buffer: &'a[u8]
}

/* TODO: expose rustic callback with ok performance..
pub trait DecodeUnknown {
    pub fn decode(&mut self, pck: pt_packet_unknown, )
}*/

bitflags! {
    pub struct Errata: u16 {
        /// BDM70: Intel(R) Processor Trace PSB+ Packets May Contain
        ///         Unexpected Packets.
        ///
        /// Same as: SKD024, SKL021, KBL021.
        ///
        /// Some Intel Processor Trace packets should be issued only between
        /// TIP.PGE and TIP.PGD packets.  Due to this erratum, when a TIP.PGE
        /// packet is generated it may be preceded by a PSB+ that incorrectly
        /// includes FUP and MODE.Exec packets.
        const BDM70  = 0b00000000_00000001;

        /// BDM64: An Incorrect LBR or Intel(R) Processor Trace Packet May Be
        ///        Recorded Following a Transactional Abort.
        ///
        /// Use of Intel(R) Transactional Synchronization Extensions (Intel(R)
        /// TSX) may result in a transactional abort.  If an abort occurs
        /// immediately following a branch instruction, an incorrect branch
        /// target may be logged in an LBR (Last Branch Record) or in an Intel(R)
        /// Processor Trace (Intel(R) PT) packet before the LBR or Intel PT
        /// packet produced by the abort.
        const BDM64  = 0b00000000_00000010;

        /// SKD007: Intel(R) PT Buffer Overflow May Result in Incorrect Packets.
        ///
        /// Same as: SKL049, KBL041.
        ///
        /// Under complex micro-architectural conditions, an Intel PT (Processor
        /// Trace) OVF (Overflow) packet may be issued after the first byte of a
        /// multi-byte CYC (Cycle Count) packet, instead of any remaining bytes
        /// of the CYC.
        const SKD007 = 0b00000000_00000100;

        /// SKD022: VM Entry That Clears TraceEn May Generate a FUP.
        ///
        /// Same as: SKL024, KBL023.
        ///
        /// If VM entry clears Intel(R) PT (Intel Processor Trace)
        /// IA32_RTIT_CTL.TraceEn (MSR 570H, bit 0) while PacketEn is 1 then a
        /// FUP (Flow Update Packet) will precede the TIP.PGD (Target IP Packet,
        /// Packet Generation Disable).  VM entry can clear TraceEn if the
        /// VM-entry MSR-load area includes an entry for the IA32_RTIT_CTL MSR.
        const SKD022 = 0b00000000_00001000;

        /// SKD010: Intel(R) PT FUP May be Dropped After OVF.
        ///
        /// Same as: SKD014, SKL033, KBL030.
        ///
        /// Some Intel PT (Intel Processor Trace) OVF (Overflow) packets may not
        /// be followed by a FUP (Flow Update Packet) or TIP.PGE (Target IP
        /// Packet, Packet Generation Enable).
        const SKD010 = 0b00000000_00010000;

        /// SKL014: Intel(R) PT TIP.PGD May Not Have Target IP Payload.
        ///
        /// Same as: KBL014.
        ///
        /// When Intel PT (Intel Processor Trace) is enabled and a direct
        /// unconditional branch clears IA32_RTIT_STATUS.FilterEn (MSR 571H, bit
        /// 0), due to this erratum, the resulting TIP.PGD (Target IP Packet,
        /// Packet Generation Disable) may not have an IP payload with the target
        /// IP.
        const SKL014 = 0b00000000_00100000;

        /// APL12: Intel(R) PT OVF May Be Followed By An Unexpected FUP Packet.
        ///
        /// Certain Intel PT (Processor Trace) packets including FUPs (Flow
        /// Update Packets), should be issued only between TIP.PGE (Target IP
        /// Packet - Packet Generaton Enable) and TIP.PGD (Target IP Packet -
        /// Packet Generation Disable) packets.  When outside a TIP.PGE/TIP.PGD
        /// pair, as a result of IA32_RTIT_STATUS.FilterEn[0] (MSR 571H) being
        /// cleared, an OVF (Overflow) packet may be unexpectedly followed by a
        /// FUP.
        const APL12  = 0b00000000_01000000;

        /// APL11: Intel(R) PT OVF Pakcet May Be Followed by TIP.PGD Packet
        ///
        /// If Intel PT (Processor Trace) encounters an internal buffer overflow
        /// and generates an OVF (Overflow) packet just as IA32_RTIT_CTL (MSR
        /// 570H) bit 0 (TraceEn) is cleared, or during a far transfer that
        /// causes IA32_RTIT_STATUS.ContextEn[1] (MSR 571H) to be cleared, the
        /// OVF may be followed by a TIP.PGD (Target Instruction Pointer - Packet
        /// Generation Disable) packet.
        const APL11  = 0b00000000_10000000;

        /// SKL168: Intel(R) PT CYC Packets Can be Dropped When Immediately
        ///          Preceding PSB
        ///
        /// Due to a rare microarchitectural condition, generation of an Intel
        /// PT (Processor Trace) PSB (Packet Stream Boundary) packet can cause a
        /// single CYC (Cycle Count) packet, possibly along with an associated
        /// MTC (Mini Time Counter) packet, to be dropped.
        const SKL168 = 0b00000001_00000000;
    }
}

impl From<Errata> for pt_errata {
    fn from(errata: Errata) -> Self {
        let err_bits = errata.bits();
        // i know this looks confusing (and it kinda is)
        // bindgen uses a byte array to store the bitfields
        // we split the u16 (Errata) into such an array
        // this is both faster and more future proof
        // than calling the setter for each bitfield
        // sadly, at the cost of readability
        pt_errata {
            _bitfield_1: __BindgenBitfieldUnit::new([
                (err_bits & 0x00FF) as u8,
                (err_bits >> 8) as u8
            ]), reserved: Default::default()
        }
    }
}


impl<'a> Config<'a> {
    /// the simples constructor
    /// offering none of the intelpt configuration options
    pub fn new(mut buffer: &'a [u8], cpu: pt_cpu, errata: Errata) -> Config {
        // TODO error handling if buffer has no elements
        // would i really want to return Result<Config>?
        // seems a bit weird to have a failing ctor
        // maybe im just an oop slut
        let cfg = pt_config {
            size: buffer.len(), // pushing cuz battery dying :'D
            begin: buffer.as_mut_ptr(),
            end: buffer.as_mut_ptr().offset(buffer.len() as isize),
            decode: pt_config__bindgen_ty_1 {
                callback: None,
                context: ptr::null_mut()
            },
            cpu,
            errata: errata.into()

        };
    }
}