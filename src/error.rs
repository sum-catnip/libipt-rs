// Certain casts are required only on Linux. Inform Clippy to ignore them.
#![allow(clippy::unnecessary_cast)]

use libipt_sys::{pt_error_code, pt_errstr};
use libipt_sys::{
    pt_error_code_pte_bad_config, pt_error_code_pte_bad_context, pt_error_code_pte_bad_cpu,
    pt_error_code_pte_bad_file, pt_error_code_pte_bad_image, pt_error_code_pte_bad_insn,
    pt_error_code_pte_bad_lock, pt_error_code_pte_bad_opc, pt_error_code_pte_bad_packet,
    pt_error_code_pte_bad_query, pt_error_code_pte_bad_retcomp,
    pt_error_code_pte_bad_status_update, pt_error_code_pte_eos, pt_error_code_pte_event_ignored,
    pt_error_code_pte_internal, pt_error_code_pte_invalid, pt_error_code_pte_ip_suppressed,
    pt_error_code_pte_no_cbr, pt_error_code_pte_no_enable, pt_error_code_pte_no_time,
    pt_error_code_pte_noip, pt_error_code_pte_nomap, pt_error_code_pte_nomem,
    pt_error_code_pte_nosync, pt_error_code_pte_not_supported, pt_error_code_pte_ok,
    pt_error_code_pte_overflow, pt_error_code_pte_retstack_empty,
};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};
use std::ptr::NonNull;

#[derive(Clone, Copy, Debug, TryFromPrimitive, PartialEq)]
#[repr(i32)]
pub enum PtErrorCode {
    /// No error. Everything is OK
    Ok = pt_error_code_pte_ok as i32,
    /// Internal decoder error
    Internal = pt_error_code_pte_internal as i32,
    /// Invalid argument
    Invalid = pt_error_code_pte_invalid as i32,
    /// Decoder out of sync
    Nosync = pt_error_code_pte_nosync as i32,
    /// Unknown opcode
    BadOpc = pt_error_code_pte_bad_opc as i32,
    /// Unknown payload
    BadPacket = pt_error_code_pte_bad_packet as i32,
    /// Unexpected packet context
    BadContext = pt_error_code_pte_bad_context as i32,
    /// Decoder reached end of trace stream
    Eos = pt_error_code_pte_eos as i32,
    /// No packet matching the query to be found
    BadQuery = pt_error_code_pte_bad_query as i32,
    /// Decoder out of memory
    Nomem = pt_error_code_pte_nomem as i32,
    /// Bad configuration
    BadConfig = pt_error_code_pte_bad_config as i32,
    /// There is no IP
    Noip = pt_error_code_pte_noip as i32,
    /// The IP has been suppressed
    IpSuppressed = pt_error_code_pte_ip_suppressed as i32,
    /// There is no memory mapped at the requested address
    Nomap = pt_error_code_pte_nomap as i32,
    /// An instruction could not be decoded
    BadInsn = pt_error_code_pte_bad_insn as i32,
    /// No wall-clock time is available
    NoTime = pt_error_code_pte_no_time as i32,
    /// No core:bus ratio available
    NoCbr = pt_error_code_pte_no_cbr as i32,
    /// Bad traced image
    BadImage = pt_error_code_pte_bad_image as i32,
    /// A locking error
    BadLock = pt_error_code_pte_bad_lock as i32,
    /// The requested feature is not supported
    NotSupported = pt_error_code_pte_not_supported as i32,
    /// The return address stack is empty
    RetstackEmpty = pt_error_code_pte_retstack_empty as i32,
    /// A compressed return is not indicated correctly by a taken branch
    BadRetcomp = pt_error_code_pte_bad_retcomp as i32,
    /// The current decoder state does not match the state in the trace
    BadStatusUpdate = pt_error_code_pte_bad_status_update as i32,
    /// The trace did not contain an expected enabled event
    NoEnable = pt_error_code_pte_no_enable as i32,
    /// An event was ignored
    EventIgnored = pt_error_code_pte_event_ignored as i32,
    /// Something overflowed
    Overflow = pt_error_code_pte_overflow as i32,
    /// A file handling error
    BadFile = pt_error_code_pte_bad_file as i32,
    /// Unknown cpu
    BadCpu = pt_error_code_pte_bad_cpu as i32,

    /// No Error Information available
    NoInfo = -1,
}

#[derive(Debug, Clone, Copy)]
pub struct PtError {
    code: PtErrorCode,
    msg: &'static str,
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
        // panicking here is fine since this should only be called
        // for return values of libipt functions
        // so invalid returns = bug in libipt or the bindings
        PtError::new(PtErrorCode::try_from(-code).unwrap(), unsafe {
            CStr::from_ptr(pt_errstr(-code as pt_error_code))
                .to_str()
                .unwrap()
        })
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
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Checks a pointer returned by one of the libipt functions. And converts it in a `Result`.
/// NULL values will be translated into `Err(pt_err)` value.
/// other values are mapped into Ok(NonNull(ptr))
#[inline]
pub(crate) fn libipt_ptr_check_null<T>(
    ptr: *mut T,
    pt_err: PtError,
) -> Result<NonNull<T>, PtError> {
    NonNull::new(ptr).ok_or(pt_err)
}

// todo: nuke deref_ptresult_mut in the entire crate
/// Dereferences a pointer returned by one of the libipt functions.
/// Checks the pointer for NULL.
/// Negative values will be translated into the appropriate error value.
#[inline]
pub(crate) fn deref_ptresult_mut<T>(res: *mut T) -> Result<&'static mut T, PtError> {
    match res as isize {
        // null reference, no error info
        0 => Err(PtError::new(PtErrorCode::NoInfo, "No further information")),
        x if x < 0 => Err(PtError::from_code(x as i32)),
        // fixme: `as_mut()` requires the pointer to be  convertible to a reference. This is not
        // true in many cases: When creating a mutable reference, then while this reference exists,
        // the memory it points to must not get accessed (read or written) through any other pointer
        // or reference not derived from this reference.
        // (Often) we cannot ensure that libipt doesnt use those pointer inside.
        _ => Ok(unsafe { res.as_mut().unwrap() }),
    }
}

// Translates a pt error code into a result enum.
// Discards the error code
#[inline]
pub(crate) fn ensure_ptok(code: i32) -> Result<(), PtError> {
    extract_pterr(code).map(|_| ())
}

// Turns a negative code into a PtErr.
// Returns the code as an unsigned int
#[inline]
pub(crate) fn extract_pterr(code: i32) -> Result<u32, PtError> {
    match code {
        x if x >= 0 => Ok(code as u32),
        _ => Err(PtError::from_code(code)),
    }
}
