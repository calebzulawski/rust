#![cfg_attr(target_arch = "x86_64", feature(target_feature_enabled))]

#[cfg(target_arch = "x86_64")]
const SSE2: bool = std::arch::is_x86_feature_enabled!("sse2");

#[cfg(target_arch = "x86_64")]
#[test]
fn global() {
    const {
        assert!(SSE2, "global features should have SSE2");
    }

    let sse2 = std::arch::is_x86_feature_enabled!("sse2");
    assert!(sse2, "global features should have SSE2");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn attr() {
    #[target_feature(enable = "avx")]
    fn avx() {
        const AVX: bool = std::arch::is_x86_feature_enabled!("avx");
        const {
            assert!(AVX, "enabled features should have AVX");
        }

        let avx = std::arch::is_x86_feature_enabled!("avx");
        assert!(avx, "enabled features should have AVX");
    }

    if std::arch::is_x86_feature_detected!("avx") {
        unsafe { avx() };
    }
}
