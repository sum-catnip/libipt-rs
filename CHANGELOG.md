# Changelog

_This changelog documents only changes relevant to users, internal changes might be omitted._

## [0.3.0] 2025/01

### Added

- This [CHANGELOG](./CHANGELOG.md) 🎉
- Explicit [MSRV](Cargo.toml)

### Changed

- `ConfigBuilder` and `Config` have been replaced by `EncoderDecoderBuilder`
- Decoders/Encoder `::new()` have been replaced by `EncoderDecoderBuilder.build()`
- Decoders/Encoder `.get_config()` have been replaced by `.used_builder()`
- Block/Insn decoders `.image()` now returns `&mut Image` instead of `Result<Image,...>`
- `Image.copy()` has been replaced by `Image.extend()`
- `Image.add_cached()` now takes a `Rc<SectionCache>` instead of `&mut SectionCache` to ensure that the cache outlives the `Image`. 
- Many packet/event types methods now take a `&self` instead of consuming `self`
- Some simple methods are now `const`

### Removed

- `From<&mut pt_image> for Image`
- Decoders/Encoder callback due to [UB issues](https://github.com/sum-catnip/libipt-rs/issues/9) (PRs are welcome)

### Fixed

- Some safety issues: __do not cast raw pointers to references with `ptr.as_mut()` with FFI pointers.__  
  `as_mut()` [requires](https://doc.rust-lang.org/std/ptr/index.html#pointer-to-reference-conversion) the pointer to be  convertible to a reference.
  This is not true in many cases for FFI pointers: when creating a mutable reference, the referenced memory must not get accessed (read or written) through any other pointer or reference not derived from this reference.
  In many cases, we cannot ensure that `libipt` does not access the pointed data internally.

## [0.2.x]

_No changelog provided_
