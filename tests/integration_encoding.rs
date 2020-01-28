use libipt::{ ConfigBuilder, Cpu, Encoder };

#[test]
fn test_encoder_encode() {
    let mut inp = [0; 10];
    let mut cfg = ConfigBuilder::new(&mut inp).finish();
    let enc = Encoder::new(&mut cfg);

}