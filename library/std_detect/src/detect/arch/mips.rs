//! Run-time feature detection on MIPS.

features! {
    @TARGET: mips;
    @CFG: target_arch = "mips";
    @MACRO_NAME: is_mips_feature_detected;
    @MACRO_ATTRS:
    /// Checks if `mips` feature is enabled.
    #[unstable(feature = "stdarch_mips_feature_detection", issue = "111188")]
    @MACRO_NAME_TARGET_FEATURE_ENABLED: is_mips_feature_enabled;
    @MACRO_TARGET_FEATURE_ENABLED_ATTRS:
    /// This macro tests, at compile time, whether a `mips` feature is enabled on mips platforms.
    ///
    /// This macro expands to a boolean constant that reflects the target features of the enclosing
    /// function, considering both global features and those added by `#[target_feature]`. If this
    /// macro is used outside of a function, it only considers the global features.
    @FEATURE: #[unstable(feature = "stdarch_mips_feature_detection", issue = "111188")] msa: "msa";
    /// MIPS SIMD Architecture (MSA)
}
