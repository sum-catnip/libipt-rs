#![warn(clippy::all, clippy::cargo)]

/// Intel Processor Trace (Intel PT) encoder or decoder configuration.
pub mod enc_dec_builder;

/// Unique error enum used across the library.
///
/// Not all errors may occur in every module's function.
/// Every API function specifies the errors it may return (sometimes not exhaustively!).
pub mod error;

/// Intel PT packet encoding and decoding.
pub mod packet;

/// Higher-level events often resulting from the combination of different PT packets.
///
/// It is used for reconstructing execution flow for users who need finer-grain control not available via the instruction flow layer
/// or for users who want to integrate execution flow reconstruction with other functionality more tightly than it would be possible otherwise.
pub mod event;

/// Simple API for iterating over blocks of sequential instructions in execution order.
///
/// The instructions in a block are sequential in the sense that no trace is required for reconstructing the instructions.
/// The IP of the first instruction is given in struct `Block` and the IP of other instructions in the block can be determined by decoding and examining the previous instruction.
pub mod block;

/// Simple API for iterating over instructions in execution order.
pub mod insn;

mod version;
pub use version::Version;

/// Module handling the memory images of the binaries beeing traced, used in high level decoding.
pub mod image;

/// Address Space Identifier
pub mod asid;

/// Info about the decoder status
pub mod status;
