use crate::key::Key;
use crate::portable::PortableHash;
use crate::traits::HighwayHash;
use std::default::Default;

#[cfg(all(target_arch = "x86_64", feature = "use_std"))]
use crate::avx::AvxHash;
#[cfg(all(target_arch = "x86_64", feature = "use_std"))]
use crate::sse::SseHash;

#[derive(Debug, Clone)]
enum HighwayChoices {
    Portable(PortableHash),
    #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
    Sse(SseHash),
    #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
    Avx(AvxHash),
}

/// Main HighwayHash implementation that delegates to one of the other implementations depending on
/// the target compiled for and a runtime CPU check.
///
/// In order of preference:
///
///  - AvxHash
///  - SseHash
///  - PortableHash
#[derive(Debug, Clone)]
pub struct HighwayBuilder(HighwayChoices);

impl HighwayHash for HighwayBuilder {
    fn hash64(self, data: &[u8]) -> u64 {
        match self.0 {
            HighwayChoices::Portable(x) => x.hash64(data),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Avx(x) => x.hash64(data),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Sse(x) => x.hash64(data),
        }
    }

    fn hash128(self, data: &[u8]) -> [u64; 2] {
        match self.0 {
            HighwayChoices::Portable(x) => x.hash128(data),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Avx(x) => x.hash128(data),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Sse(x) => x.hash128(data),
        }
    }

    fn hash256(self, data: &[u8]) -> [u64; 4] {
        match self.0 {
            HighwayChoices::Portable(x) => x.hash256(data),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Avx(x) => x.hash256(data),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Sse(x) => x.hash256(data),
        }
    }

    fn append(&mut self, data: &[u8]) {
        match &mut self.0 {
            HighwayChoices::Portable(x) => x.append(data),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Avx(x) => x.append(data),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Sse(x) => x.append(data),
        }
    }

    fn finalize64(self) -> u64 {
        match self.0 {
            HighwayChoices::Portable(x) => x.finalize64(),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Avx(x) => x.finalize64(),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Sse(x) => x.finalize64(),
        }
    }

    fn finalize128(self) -> [u64; 2] {
        match self.0 {
            HighwayChoices::Portable(x) => x.finalize128(),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Avx(x) => x.finalize128(),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Sse(x) => x.finalize128(),
        }
    }

    fn finalize256(self) -> [u64; 4] {
        match self.0 {
            HighwayChoices::Portable(x) => x.finalize256(),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Avx(x) => x.finalize256(),
            #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
            HighwayChoices::Sse(x) => x.finalize256(),
        }
    }
}

impl HighwayBuilder {
    /// Creates a new hasher based on compilation and runtime capabilities
    pub fn new(key: Key) -> Self {
        #[cfg(all(target_arch = "x86_64", feature = "use_std"))]
        {
            if let Some(h) = AvxHash::new(key) {
                return HighwayBuilder(HighwayChoices::Avx(h));
            }

            if let Some(h) = SseHash::new(key) {
                return HighwayBuilder(HighwayChoices::Sse(h));
            }
        }

        HighwayBuilder(HighwayChoices::Portable(PortableHash::new(key)))
    }
}

impl Default for HighwayBuilder {
    fn default() -> Self {
        HighwayBuilder::new(Key::default())
    }
}

impl_write!(HighwayBuilder);
impl_hasher!(HighwayBuilder);
