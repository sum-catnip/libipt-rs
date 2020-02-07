use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};
use std::error::Error;

use libipt_sys::pt_errstr;
use libipt_sys::{
    pt_error_code_pte_ok,
    pt_error_code_pte_internal,
    pt_error_code_pte_invalid,
    pt_error_code_pte_nosync,
    pt_error_code_pte_bad_opc,
    pt_error_code_pte_bad_packet,
    pt_error_code_pte_bad_context,
    pt_error_code_pte_eos,
    pt_error_code_pte_bad_query,
    pt_error_code_pte_nomem,
    pt_error_code_pte_bad_config,
    pt_error_code_pte_noip,
    pt_error_code_pte_ip_suppressed,
    pt_error_code_pte_nomap,
    pt_error_code_pte_bad_insn,
    pt_error_code_pte_no_time,
    pt_error_code_pte_no_cbr,
    pt_error_code_pte_bad_image,
    pt_error_code_pte_bad_lock,
    pt_error_code_pte_not_supported,
    pt_error_code_pte_retstack_empty,
    pt_error_code_pte_bad_retcomp,
    pt_error_code_pte_bad_status_update,
    pt_error_code_pte_no_enable,
    pt_error_code_pte_event_ignored,
    pt_error_code_pte_overflow,
    pt_error_code_pte_bad_file,
    pt_error_code_pte_bad_cpu
};

#[derive(Clone, Copy, Debug, TryFromPrimitive, PartialEq)]
#[repr(i32)]
pub enum PtErrorCode {
    /// No error. Everything is OK
    Ok = pt_error_code_pte_ok,
    /// Internal decoder error
    Internal = pt_error_code_pte_internal,
    /// Invalid argument
    Invalid = pt_error_code_pte_invalid,
    /// Decoder out of sync
    Nosync = pt_error_code_pte_nosync,
    /// Unknown opcode
    BadOpc = pt_error_code_pte_bad_opc,
    /// Unknown payload
    BadPacket = pt_error_code_pte_bad_packet,
    /// Unexpected packet context
    BadContext = pt_error_code_pte_bad_context,
    /// Decoder reached end of trace stream
    Eos = pt_error_code_pte_eos,
    /// No packet matching the query to be found
    BadQuery = pt_error_code_pte_bad_query,
    /// Decoder out of memory
    Nomem = pt_error_code_pte_nomem,
    /// Bad configuration
    BadConfig = pt_error_code_pte_bad_config,
    /// There is no IP
    Noip = pt_error_code_pte_noip,
    /// The IP has been suppressed
    IpSuppressed = pt_error_code_pte_ip_suppressed,
    /// There is no memory mapped at the requested address
    Nomap = pt_error_code_pte_nomap,
    /// An instruction could not be decoded
    BadInsn = pt_error_code_pte_bad_insn,
    /// No wall-clock time is available
    NoTime = pt_error_code_pte_no_time,
    /// No core:bus ratio available
    NoCbr = pt_error_code_pte_no_cbr,
    /// Bad traced image
    BadImage = pt_error_code_pte_bad_image,
    /// A locking error
    BadLock = pt_error_code_pte_bad_lock,
    /// The requested feature is not supported
    NotSupported = pt_error_code_pte_not_supported,
    /// The return address stack is empty
    RetstackEmpty = pt_error_code_pte_retstack_empty,
    /// A compressed return is not indicated correctly by a taken branch
    BadRetcomp = pt_error_code_pte_bad_retcomp,
    /// The current decoder state does not match the state in the trace
    BadStatusUpdate = pt_error_code_pte_bad_status_update,
    /// The trace did not contain an expected enabled event
    NoEnable = pt_error_code_pte_no_enable,
    /// An event was ignored
    EventIgnored = pt_error_code_pte_event_ignored,
    /// Something overflowed
    Overflow = pt_error_code_pte_overflow,
    /// A file handling error
    BadFile = pt_error_code_pte_bad_file,
    /// Unknown cpu
    BadCpu = pt_error_code_pte_bad_cpu,

    /// No Error Information available
    NoInfo = -1
}

#[derive(Debug, Clone, Copy)]
pub struct PtError {
     code: PtErrorCode,
     msg:  &'static str
}

impl PtError {
    #[inline]
    pub(crate) fn new(code: PtErrorCode, msg: &'static str) -> Self {
        PtError { code, msg }
    }

    /// Creates a PTError instance based on the error code
    /// The code should be provided in the way its returned from the pt function
    /// pt functions always return negative error codes
    /// *error codes not included in the pt_error enum will panic!*
    #[inline]
    pub(crate) fn from_code(code: i32) -> Self {
        // panicing here is fine since this should only be called
        // for return values of libipt functions
        // so invalid returns = bug in libipt or the bindings
        PtError::new(
            PtErrorCode::try_from(-code).unwrap(),
            unsafe { CStr::from_ptr(pt_errstr(-code)).to_str().unwrap() }
        )
    }

    /// get the pt error code
    #[inline]
    pub fn code(self) -> PtErrorCode {
        self.code
    }

    /// get a human readable error message
    #[inline]
    pub fn msg(self) -> &'static str {
        self.msg
    }
}

impl Display for PtError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "error from libipt: {}", self.msg)
    }
}

impl Error for PtError {
    // sadly we have no idea what the source is
    fn source(&self) -> Option<&(dyn Error + 'static)> { None }
}

/// Dereferences a pointer returned by one of the libipt functions.
/// Checks the pointer for NULL.
/// Negative values will be translated into the appropriate error value.
#[inline]
pub(crate) fn deref_ptresult<T>(res: *const T) -> Result<&'static T, PtError> {
    match res as isize {
        // null reference, no error info
        0 => Err(PtError::new(PtErrorCode::NoInfo, "No further information")),
        x if x < 0 => Err(PtError::from_code(x as i32)),
        _ => Ok(unsafe { res.as_ref().unwrap() })
    }
}

/// Dereferences a pointer returned by one of the libipt functions.
/// Checks the pointer for NULL.
/// Negative values will be translated into the appropriate error value.
#[inline]
pub(crate) fn deref_ptresult_mut<T>(res: *mut T) -> Result<&'static mut T, PtError> {
    match res as isize {
        // null reference, no error info
        0 => Err(PtError::new(PtErrorCode::NoInfo, "No further information")),
        x if x < 0 => Err(PtError::from_code(x as i32)),
        _ => Ok(unsafe { res.as_mut().unwrap() })
    }
}

// Translates a pt error code into a result enum.
// Discards the error code
#[inline]
pub(crate) fn ensure_ptok(code: i32) -> Result<(), PtError> {
    extract_pterr(code).map(|_|())
}

// Turns a negative code into a PtErr.
// Returns the code as an unsigned int
#[inline]
pub(crate) fn extract_pterr(code: i32) -> Result<u32, PtError> {
    match code {
        x if x >= 0 => Ok(code as u32),
        _ => Err(PtError::from_code(code))
    }
}
