use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};
use std::error::Error;

use libipt_sys::pt_errstr;

#[derive(Clone, Copy, Debug, TryFromPrimitive)]
#[repr(i32)]
pub enum PtErrorCode {
    /// No error. Everything is OK
    Ok,
    /// Internal decoder error
    Internal,
    /// Invalid argument
    Invalid,
    /// Decoder out of sync
    Nosync,
    /// Unknown opcode
    BadOpc,
    /// Unknown payload
    BadPacket,
    /// Unexpected packet context
    BadContext,
    /// Decoder reached end of trace stream
    Eos,
    /// No packet matching the query to be found
    BadQuery,
    /// Decoder out of memory
    Nomem,
    /// Bad configuration
    BadConfig,
    /// There is no IP
    Noip,
    /// The IP has been suppressed
    IpSuppressed,
    /// There is no memory mapped at the requested address
    Nomap,
    /// An instruction could not be decoded
    BadInsn,
    /// No wall-clock time is available
    NoTime,
    /// No core:bus ratio available
    NoCbr,
    /// Bad traced image
    BadImage,
    /// A locking error
    BadLock,
    /// The requested feature is not supported
    NotSupported,
    /// The return address stack is empty
    RetstackEmpty,
    /// A compressed return is not indicated correctly by a taken branch
    BadRetcomp,
    /// The current decoder state does not match the state in the trace
    BadStatusUpdate,
    /// The trace did not contain an expected enabled event
    NoEnable,
    /// An event was ignored
    EventIgnored,
    /// Something overflowed
    Overflow,
    /// A file handling error
    BadFile,
    /// Unknown cpu
    BadCpu,


    /// No Error Information available
    NoInfo = -1
}

#[derive(Debug, Clone, Copy)]
pub struct PtError {
     code: PtErrorCode,
     msg:  &'static str
}

impl PtError {
    pub fn new(code: PtErrorCode, msg: &'static str) -> Self {
        PtError { code, msg }
    }

    /// Creates a PTError instance based on the error code
    /// The code should be provided in the way its returned from the pt function
    /// pt functions always return negative error codes
    /// *error codes not included in the pt_error enum will panic!*
    pub fn from_code(code: i32) -> Self {
        // panicing here is fine since this should only be called
        // for return values of libipt functions
        // so invalid returns = bug in libipt or the bindings
        PtError::new(
            PtErrorCode::try_from(-code).unwrap(),
            unsafe { CStr::from_ptr(pt_errstr(-code)).to_str().unwrap() }
        )
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
pub(crate) fn deref_ptresult_mut<T>(res: *mut T) -> Result<&'static mut T, PtError> {
    match res as isize {
        // null reference, no error info
        0 => Err(PtError::new(PtErrorCode::NoInfo, "No further information")),
        x if x < 0 => Err(PtError::from_code(res as i32)),
        _ => Ok(unsafe { res.as_mut().unwrap() })
    }
}

/// Dereferences a pointer returned by one of the libipt functions.
/// Checks the pointer for NULL.
/// Negative values will be translated into the appropriate error value.
pub(crate) fn deref_ptresult<T>(res: *const T) -> Result<&'static T, PtError> {
    match res as isize {
        // null reference, no error info
        0 => Err(PtError::new(PtErrorCode::NoInfo, "No further information")),
        x if x < 0 => Err(PtError::from_code(x as i32)),
        _ => Ok(unsafe { res.as_ref().unwrap() })
    }
}

// Translates a pt error code into a result enum
pub(crate) fn ensure_ptok(code: i32) -> Result<i32, PtError> {
    match code {
        0 => Ok(code),
        _ => Err(PtError::from_code(code))
    }
}