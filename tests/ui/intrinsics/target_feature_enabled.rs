//@ only-x86_64
//@ compile-flags: -Ctarget-feature=-avx
//@ build-pass
#![feature(core_intrinsics)]

const GLOBAL_AVX: bool = core::intrinsics::target_feature_enabled("avx");

fn main() {
    const SSE2_ENABLED: bool = core::intrinsics::target_feature_enabled("sse2");

    const {
        assert!(!GLOBAL_AVX);
        assert!(SSE2_ENABLED);
    }

    #[target_feature(enable = "avx2")]
    unsafe fn with_avx<T: Into<()>>(x: T) {
        const HAS_AVX: bool = core::intrinsics::target_feature_enabled("avx");
        const {
            assert!(HAS_AVX);
        }
        x.into();
    }

    unsafe fn without_avx() {
        const HAS_AVX: bool = core::intrinsics::target_feature_enabled("avx");
        const {
            assert!(!HAS_AVX);
        }
    }

    unsafe {
        with_avx(());
        without_avx();
    }
}
