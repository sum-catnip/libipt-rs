#[macro_use]
mod conversions;

pub mod pad;
pub mod ovf;
pub mod psb;
pub mod psbend;
pub mod stop;
pub mod invalid;

pub mod tnt;
pub mod ip;
pub mod mode;
pub mod pip;
pub mod tsc;
pub mod cbr;
pub mod tma;
pub mod mtc;
pub mod cyc;
pub mod vmcs;
pub mod mnt;
pub mod exstop;
pub mod mwait;
pub mod pwre;
pub mod pwrx;
pub mod ptw;
pub mod unknown;

#[derive(Clone, Copy)]
pub enum Packet {
    Invalid(invalid::Invalid),
    Psbend(psbend::Psbend),
    Stop(stop::Stop),
    Pad(pad::Pad),
    Psb(psb::Psb),
    Ovf(ovf::Ovf),

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