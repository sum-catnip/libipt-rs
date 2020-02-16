/// The pt_config structure defines an Intel Processor Trace (Intel PT) encoder or decoder configuration.
///
/// It is required for allocating a trace packet encoder (see pt_alloc_encoder(3)),
/// a trace packet decoder (see pt_pkt_alloc_decoder(3)),
/// a query decoder (see pt_qry_alloc_decoder(3)),
/// or an instruction flow decoder (see pt_insn_alloc_decoder(3)).
pub mod config;
pub use config::*;

/// The library uses a single error enum for all layers.
///
/// Not all errors may occur on every layer.
/// Every API function specifies the errors it may return. (not accurate!)
pub mod error;
pub use error::PtError;
pub use error::PtErrorCode;

/// This layer deals with Intel PT packet encoding and decoding.
///
/// It can further be split into three sub-layers: opcodes, encoding, and decoding.
pub mod packet;

/// The event layer deals with packet combinations that encode higher-level events.
///
/// It is used for reconstructing execution flow for users who need finer-grain control not available via the instruction flow layer
/// or for users who want to integrate execution flow reconstruction with other functionality more tightly than it would be possible otherwise.
pub mod event;

/// The block layer provides a simple API for iterating over blocks of sequential instructions in execution order.
///
/// The instructions in a block are sequential in the sense that no trace is required for reconstructing the instructions.
/// The IP of the first instruction is given in struct `Block` and the IP of other instructions in the block can be determined by decoding and examining the previous instruction.
pub mod block;

/// The instruction flow layer provides a simple API for iterating over instructions in execution order.
pub mod insn;

mod version;
pub use version::Version;
mod image;
pub use image::*;
mod asid;
pub use asid::Asid;
mod flags;
pub use flags::Status;