//! Run-time feature detection on ARM Aarch32.

features! {
    @TARGET: arm;
    @CFG: target_arch = "arm";
    @MACRO_NAME: is_arm_feature_detected;
    @MACRO_ATTRS:
    /// Checks if `arm` feature is enabled.
    #[unstable(feature = "stdarch_arm_feature_detection", issue = "111190")]
    @MACRO_NAME_TARGET_FEATURE_ENABLED: is_arm_feature_enabled;
    @MACRO_TARGET_FEATURE_ENABLED_ATTRS:
    /// This macro tests, at compile time, whether an `arm` feature is enabled on arm platforms.
    ///
    /// This macro expands to a boolean constant that reflects the target features of the enclosing
    /// function, considering both global features and those added by `#[target_feature]`. If this
    /// macro is used outside of a function, it only considers the global features.
    @NO_RUNTIME_DETECTION: "v7";
    @NO_RUNTIME_DETECTION: "vfp2";
    @NO_RUNTIME_DETECTION: "vfp3";
    @NO_RUNTIME_DETECTION: "vfp4";
    @FEATURE: #[unstable(feature = "stdarch_arm_feature_detection", issue = "111190")] neon: "neon";
    /// ARM Advanced SIMD (NEON) - Aarch32
    @FEATURE: #[unstable(feature = "stdarch_arm_feature_detection", issue = "111190")] pmull: "pmull";
    without cfg check: true;
    /// Polynomial Multiply
    @FEATURE: #[unstable(feature = "stdarch_arm_feature_detection", issue = "111190")] crc: "crc";
    /// CRC32 (Cyclic Redundancy Check)
    @FEATURE: #[unstable(feature = "stdarch_arm_feature_detection", issue = "111190")] aes: "aes";
    /// FEAT_AES (AES instructions)
    @FEATURE: #[unstable(feature = "stdarch_arm_feature_detection", issue = "111190")] sha2: "sha2";
    /// FEAT_SHA1 & FEAT_SHA256 (SHA1 & SHA2-256 instructions)
    @FEATURE: #[unstable(feature = "stdarch_arm_feature_detection", issue = "111190")] i8mm: "i8mm";
    /// FEAT_I8MM (integer matrix multiplication, plus ASIMD support)
    @FEATURE: #[unstable(feature = "stdarch_arm_feature_detection", issue = "111190")] dotprod: "dotprod";
    /// FEAT_DotProd (Vector Dot-Product - ASIMDDP)
}
