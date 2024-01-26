mod inline_cpp;
mod tf;

use std::env;

pub use inline_cpp::*;
pub use tf::*;

trait CompilationBuilder {
    fn flag(&mut self, s: &str) -> &mut Self;
    fn define(&mut self, var: &str, val: Option<&str>) -> &mut Self;

    /// Build flags for tensorflow micro sources
    fn tensorflow_build_setup(&mut self) -> &mut Self {
        let target = env::var("TARGET").unwrap_or_else(|_| "".to_string());

        let build = self
            .flag("-fno-rtti") // No Runtime type information
            .flag("-fmessage-length=0")
            .flag("-fno-exceptions")
            .flag("-fno-unwind-tables")
            .flag("-ffunction-sections")
            .flag("-fdata-sections")
            .flag("-funsigned-char")
            .flag("-MMD")
            .flag("-std=c++11")
            .flag("-fno-delete-null-pointer-checks")
            .flag("-fomit-frame-pointer")
            .flag("-fpermissive")
            .flag("-fno-use-cxa-atexit")
            // use a full word for enums, this should match clang's behaviour
            .flag("-fno-short-enums")
            .define("TF_LITE_STATIC_MEMORY", None)
            .define("TF_LITE_MCU_DEBUG_LOG", None)
            .define("GEMMLOWP_ALLOW_SLOW_SCALAR_FALLBACK", None);

        // warnings on by default
        let build = if cfg!(feature = "no-c-warnings") {
            build.flag("-w")
        } else {
            build
                .flag("-Wvla")
                .flag("-Wall")
                .flag("-Wextra")
                .flag("-Wno-unused-parameter")
                .flag("-Wno-missing-field-initializers")
                .flag("-Wno-write-strings")
                .flag("-Wno-sign-compare")
                .flag("-Wunused-function")
        };

        if target.starts_with("thumb") {
            // unaligned accesses are usually a poor idea on ARM cortex-m
            build.flag("-mno-unaligned-access")
        } else {
            build
        }
    }
}
impl CompilationBuilder for cpp_build::Config {
    fn flag(&mut self, s: &str) -> &mut Self {
        self.flag(s)
    }
    fn define(&mut self, var: &str, val: Option<&str>) -> &mut Self {
        self.define(var, val)
    }
}
impl CompilationBuilder for cc::Build {
    fn flag(&mut self, s: &str) -> &mut Self {
        self.flag(s)
    }
    fn define(&mut self, var: &str, val: Option<&str>) -> &mut Self {
        self.define(var, val)
    }
}
