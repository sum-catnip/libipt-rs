macro_rules! wrap2raw {
    ($target:ty, $type:expr, $payload:ident) => {
        impl From<$target> for libipt_sys::pt_packet {
            #[inline]
            fn from(origin: $target) -> Self {
                libipt_sys::pt_packet {
                    type_: $type,
                    size: std::mem::size_of::<libipt_sys::pt_packet>() as u8,
                    payload: libipt_sys::pt_packet__bindgen_ty_1 { $payload: origin.0 }
                }
            }
        }
    };
}

macro_rules! raw2wrap {
    ($target:ty, $target_fac:expr, $origin:ty) => {
        impl Into<$target> for $origin {
            #[inline]
            fn into(self) -> $target { $target_fac(self) }
        }
    };
}