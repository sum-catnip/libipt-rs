# Changelog

_This changelog documents only changes relevant to users, internal changes might be omitted._

## [Unreleased]

### Added

- This [CHANGELOG](./CHANGELOG.md) 🎉
- Explicit [MSRV](Cargo.toml)
- Block/Insn decoder `to_owned_image(self) -> Image`

### Changed

- `ConfigBuilder` and `Config` are replaced by `EncoderDecoderBuilder`
- Decoders/Encoder `::new()` are replaced by `EncoderDecoderBuilder.build()`
- Decoders/Encoder `.get_config()` are replaced by `.used_builder()`
- Block/Insn decoders `.image()` now returns `&mut Image` instead of `Result<Image,...>`
- Block/Insn decoders `.set_image()` now takes `Option<Image>` instead of `Option<&mut Image>`
- `Image.copy()` is now replaced by `Image.extend()`
- `Image.add_cached()` takes a `Rc<SectionCache>` instead of `&mut SectionCache` to ensure that the cache outlive the Image. 
- Many packet/event types methods now get a `&self` instead of consuming `self`

### Removed

- `From<&mut pt_image> for Image`
- Decoders/Encoder callback due to [UB issues](https://github.com/sum-catnip/libipt-rs/issues/9) (PRs are welcome)

### Fixed

- Safety problems: __do not cast raw pointers to references with `ptr.as_mut()` with FFI ptrs.__  
  `as_mut()` [requires](https://doc.rust-lang.org/std/ptr/index.html#pointer-to-reference-conversion) the pointer to be  convertible to a reference.
  This is not true in many cases for FFI pointers: when creating a mutable reference, the referenced memory must not get accessed (read or written) through any other pointer or reference not derived from this reference.
  (Often) we cannot ensure that libipt doesn't access the pointed data internally.

## [0.2.x]

_No changelog provided_
