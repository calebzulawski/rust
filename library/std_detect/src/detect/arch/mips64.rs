//! Run-time feature detection on MIPS64.

features! {
    @TARGET: mips64;
    @CFG: target_arch = "mips64";
    @MACRO_NAME: is_mips64_feature_detected;
    @MACRO_ATTRS:
    /// Checks if `mips64` feature is enabled.
    #[unstable(feature = "stdarch_mips_feature_detection", issue = "111188")]
    @MACRO_NAME_TARGET_FEATURE_ENABLED: is_mips64_feature_enabled;
    @MACRO_TARGET_FEATURE_ENABLED_ATTRS:
    /// This macro tests, at compile time, whether a `mips64` feature is enabled on mips64 platforms.
    ///
    /// This macro expands to a boolean constant that reflects the target features of the enclosing
    /// function, considering both global features and those added by `#[target_feature]`. If this
    /// macro is used outside of a function, it only considers the global features.
    @FEATURE: #[unstable(feature = "stdarch_mips_feature_detection", issue = "111188")] msa: "msa";
    /// MIPS SIMD Architecture (MSA)
}
