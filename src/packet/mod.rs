use libipt_sys::{
    pt_packet,
    pt_packet_type_ppt_cbr as PT_PACKET_TYPE_PPT_CBR,
    pt_packet_type_ppt_cyc as PT_PACKET_TYPE_PPT_CYC,
    pt_packet_type_ppt_exstop as PT_PACKET_TYPE_PPT_EXSTOP,
    pt_packet_type_ppt_fup as PT_PACKET_TYPE_PPT_FUP,
    pt_packet_type_ppt_invalid as PT_PACKET_TYPE_PPT_INVALID,
    pt_packet_type_ppt_mnt as PT_PACKET_TYPE_PPT_MNT,
    pt_packet_type_ppt_mode as PT_PACKET_TYPE_PPT_MODE,
    pt_packet_type_ppt_mtc as PT_PACKET_TYPE_PPT_MTC,
    pt_packet_type_ppt_mwait as PT_PACKET_TYPE_PPT_MWAIT,
    pt_packet_type_ppt_ovf as PT_PACKET_TYPE_PPT_OVF,
    pt_packet_type_ppt_pad as PT_PACKET_TYPE_PPT_PAD,
    pt_packet_type_ppt_pip as PT_PACKET_TYPE_PPT_PIP,
    pt_packet_type_ppt_psb as PT_PACKET_TYPE_PPT_PSB,
    pt_packet_type_ppt_psbend as PT_PACKET_TYPE_PPT_PSBEND,
    pt_packet_type_ppt_ptw as PT_PACKET_TYPE_PPT_PTW,
    pt_packet_type_ppt_pwre as PT_PACKET_TYPE_PPT_PWRE,
    pt_packet_type_ppt_pwrx as PT_PACKET_TYPE_PPT_PWRX,
    pt_packet_type_ppt_stop as PT_PACKET_TYPE_PPT_STOP,
    pt_packet_type_ppt_tip as PT_PACKET_TYPE_PPT_TIP,
    pt_packet_type_ppt_tip_pgd as PT_PACKET_TYPE_PPT_TIP_PGD,
    pt_packet_type_ppt_tip_pge as PT_PACKET_TYPE_PPT_TIP_PGE,
    pt_packet_type_ppt_tma as PT_PACKET_TYPE_PPT_TMA,
    pt_packet_type_ppt_tnt_8 as PT_PACKET_TYPE_PPT_TNT_8,
    pt_packet_type_ppt_tnt_64 as PT_PACKET_TYPE_PPT_TNT_64,
    pt_packet_type_ppt_tsc as PT_PACKET_TYPE_PPT_TSC,
    pt_packet_type_ppt_unknown as PT_PACKET_TYPE_PPT_UNKNOWN,
    pt_packet_type_ppt_vmcs as PT_PACKET_TYPE_PPT_VMCS
};

#[macro_use]
mod conversions;

mod pad;
pub use pad::*;
mod ovf;
pub use ovf::*;
mod psb;
pub use psb::*;
mod psbend;
pub use psbend::*;
mod stop;
pub use stop::*;
mod invalid;
pub use invalid::*;

mod tnt;
pub use tnt::*;
mod ip;
pub use ip::*;
mod mode;
pub use mode::*;
mod pip;
pub use pip::*;
mod tsc;
pub use tsc::*;
mod cbr;
pub use cbr::*;
mod tma;
pub use tma::*;
mod mtc;
pub use mtc::*;
mod cyc;
pub use cyc::*;
mod vmcs;
pub use vmcs::*;
mod mnt;
pub use mnt::*;
mod exstop;
pub use exstop::*;
mod mwait;
pub use mwait::*;
mod pwre;
pub use pwre::*;
mod pwrx;
pub use pwrx::*;
mod ptw;
pub use ptw::*;
mod unknown;
pub use unknown::*;

mod decoder;
pub use decoder::PacketDecoder;

mod encoder;
pub use encoder::Encoder;

#[cfg(test)]
mod test {
    use super::*;
    use libipt_sys::pt_packet_mnt;
    use libipt_sys::pt_packet__bindgen_ty_1;

    #[test]
    fn test_pkt_from() {
        let p1 = pt_packet_mnt { payload: 666 };
        let p2 = pt_packet {
            type_: PT_PACKET_TYPE_PPT_MNT,
            size: std::mem::size_of::<pt_packet_mnt>() as u8,
            payload: pt_packet__bindgen_ty_1 { mnt: p1 }
        };
        let p3: Packet::<()> = p2.into();
        match p3 {
            Packet::Mnt(m) => assert_eq!(m.payload(), p1.payload),
            _ => unreachable!()
        };
    }
}

pub enum Packet<T> {
    Invalid(invalid::Invalid),
    Psbend(psbend::Psbend),
    Stop(stop::Stop),
    Pad(pad::Pad),
    Psb(psb::Psb),
    Ovf(ovf::Ovf),
    Unknown(unknown::Unknown<T>),

    Fup(ip::Fup),
    Tip(ip::Tip),
    TipPge(ip::TipPge),
    TipPgd(ip::TipPgd),
    Tnt8(tnt::Tnt8),
    Tnt64(tnt::Tnt64),
    Mode(mode::Mode),
    Pip(pip::Pip),
    Vmcs(vmcs::Vmcs),
    Cbr(cbr::Cbr),
    Tsc(tsc::Tsc),
    Tma(tma::Tma),
    Mtc(mtc::Mtc),
    Cyc(cyc::Cyc),
    Mnt(mnt::Mnt),
    Exstop(exstop::Exstop),
    Mwait(mwait::Mwait),
    Pwre(pwre::Pwre),
    Pwrx(pwrx::Pwrx),
    Ptw(ptw::Ptw)
}

impl<T> From<pt_packet> for Packet<T> {
    fn from(pkt: pt_packet) -> Self {
        unsafe {
            match pkt.type_ {
                PT_PACKET_TYPE_PPT_CBR => Packet::Cbr(pkt.payload.cbr.into()),
                PT_PACKET_TYPE_PPT_CYC => Packet::Cyc(pkt.payload.cyc.into()),
                PT_PACKET_TYPE_PPT_EXSTOP => Packet::Exstop(pkt.payload.exstop.into()),
                PT_PACKET_TYPE_PPT_FUP => Packet::Fup(pkt.payload.ip.into()),
                PT_PACKET_TYPE_PPT_INVALID => Packet::Invalid(pkt.into()),
                PT_PACKET_TYPE_PPT_MNT => Packet::Mnt(pkt.payload.mnt.into()),
                PT_PACKET_TYPE_PPT_MODE => Packet::Mode(pkt.payload.mode.into()),
                PT_PACKET_TYPE_PPT_MTC => Packet::Mtc(pkt.payload.mtc.into()),
                PT_PACKET_TYPE_PPT_MWAIT => Packet::Mwait(pkt.payload.mwait.into()),
                PT_PACKET_TYPE_PPT_OVF => Packet::Ovf(pkt.into()),
                PT_PACKET_TYPE_PPT_PAD => Packet::Pad(pkt.into()),
                PT_PACKET_TYPE_PPT_PIP => Packet::Pip(pkt.payload.pip.into()),
                PT_PACKET_TYPE_PPT_PSB => Packet::Psb(pkt.into()),
                PT_PACKET_TYPE_PPT_PSBEND => Packet::Psbend(pkt.into()),
                PT_PACKET_TYPE_PPT_PTW => Packet::Ptw(pkt.payload.ptw.into()),
                PT_PACKET_TYPE_PPT_PWRE => Packet::Pwre(pkt.payload.pwre.into()),
                PT_PACKET_TYPE_PPT_PWRX => Packet::Pwrx(pkt.payload.pwrx.into()),
                PT_PACKET_TYPE_PPT_STOP => Packet::Stop(pkt.into()),
                PT_PACKET_TYPE_PPT_TIP => Packet::Tip(pkt.payload.ip.into()),
                PT_PACKET_TYPE_PPT_TIP_PGD => Packet::TipPgd(pkt.payload.ip.into()),
                PT_PACKET_TYPE_PPT_TIP_PGE => Packet::TipPge(pkt.payload.ip.into()),
                PT_PACKET_TYPE_PPT_TMA => Packet::Tma(pkt.payload.tma.into()),
                PT_PACKET_TYPE_PPT_TNT_8 => Packet::Tnt8(pkt.payload.tnt.into()),
                PT_PACKET_TYPE_PPT_TNT_64 => Packet::Tnt64(pkt.payload.tnt.into()),
                PT_PACKET_TYPE_PPT_TSC => Packet::Tsc(pkt.payload.tsc.into()),
                PT_PACKET_TYPE_PPT_VMCS => Packet::Vmcs(pkt.payload.vmcs.into()),
                PT_PACKET_TYPE_PPT_UNKNOWN => Packet::Unknown(unknown::Unknown::<T>::from(pkt.payload.unknown)),
                _ => unreachable!("invalid packet type")
            }
        }
    }
}