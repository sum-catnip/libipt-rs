use super::*;

#[test]
fn test_config_empty_buffer() {
    let c = Config::new(&mut [0; 0]);
    assert_eq!(c.0.begin, c.0.end);
    assert_eq!(c.0.size, std::mem::size_of::<libipt_sys::pt_config>());
}

/*
fn test_config_all_values() {
    let c = Config::new(&mut [0; 10])
        .cpu(C)
}*/