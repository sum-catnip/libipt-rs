# Libipt

> The Intel Processor Trace (Intel PT) Decoder Library is Intel's reference
implementation for decoding Intel PT.  It can be used as a standalone library or
it can be partially or fully integrated into your tool.

This Repository contains high level rust bindings for the complete functionality provided by [the original libipt library](https://github.com/intel/libipt).

Huge thanks to the rust discord community for being awesome and helping me out with some stuffs :D.

# State

## Testing

All of the functionality is implemented and should be working
but the test coverage is not complete.
If there is interest in this library i might add some more testing.
Contributions are also appreciated.

## Documentation

I did my best to provide useful documentation for most of the library.
If you see any missing or weird documentation feel free to open an issue or pull request.

docs.rs is sadly unable to build the project because of a header file which needs to be copied out of the build dir.
Ill need to get the sorted out somehow.

# Unit Tests
- block:   ✔️
- config:  ✔️
- event:   ✔️
- image:   ✔️
- insn:    ✔️
- packet:  ✔️
- asid:    ✔️️
- encoder: ✔️
- query:   ✔️
- version: ✔️

# Integration Tests
- Encoding:        ❌
- Block Decoding:  ❌
- Insn Decoding:   ❌
- Packet Decoding: ❌
- Query Decoding:  ❌
