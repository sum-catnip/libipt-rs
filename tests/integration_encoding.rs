use libipt::{ ConfigBuilder, Cpu };
use libipt::packet::*;

#[test]
fn test_encoder_all_packets() {
    let mut inp = [0; 132];
    let mut cfg = ConfigBuilder::new(&mut inp)
        .unwrap()
        .cpu(Cpu::intel(1, 2, 3))
        .finish();

    let mut enc = Encoder::new(&mut cfg).unwrap();

    let mut size: u32 = 0;

    size += enc.next(Pad::new()).unwrap();
    size += enc.next(Psb::new()).unwrap();
    size += enc.next(Psbend::new()).unwrap();
    size += enc.next(Ovf::new()).unwrap();

    size += enc.next(Fup::new(123, Compression::Sext48)).unwrap();
    size += enc.next(Tip::new(321, Compression::Full)).unwrap();
    size += enc.next(TipPge::new(666, Compression::Suppressed)).unwrap();
    size += enc.next(TipPgd::new(888, Compression::Update16)).unwrap();
    size += enc.next(Tnt8::new(3, 4)).unwrap();
    size += enc.next(Tnt64::new(4, 13)).unwrap();
    size += enc.next(Mode::new(Payload::Exec(Exec::CSL | Exec::CSD))).unwrap();
    size += enc.next(Pip::new(1337, false)).unwrap();
    size += enc.next(Tsc::new(69)).unwrap();
    size += enc.next(Cbr::new(5)).unwrap();
    size += enc.next(Tma::new(420, 421)).unwrap();
    size += enc.next(Mtc::new(0)).unwrap();
    size += enc.next(Cyc::new(0xCA7)).unwrap();
    size += enc.next(Stop::new()).unwrap();    
    size += enc.next(Vmcs::new(111)).unwrap();
    size += enc.next(Mnt::new(222)).unwrap();
    size += enc.next(Exstop::new(true)).unwrap();
    size += enc.next(Mwait::new(333, 444)).unwrap();
    size += enc.next(Pwre::new(101, 10, false)).unwrap();
    size += enc.next(Pwrx::new(1, 2, false, true, false)).unwrap();
    size += enc.next(Ptw::new(5, 0, false)).unwrap();

    assert_eq!(size, 132);
    assert!(enc.next(Pad::new()).is_err());
}