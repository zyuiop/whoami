// We don't need unsafe, yay!
#![forbid(unsafe_code)]

use std::ffi::OsString;

use crate::{
    os::{Os, Target},
    Arch, DesktopEnv, LanguagePrefs, Platform, Result,
};

impl Target for Os {
    fn lang_prefs(self) -> Result<LanguagePrefs> {
        super::unix_lang()
    }

    #[inline(always)]
    fn realname(self) -> Result<OsString> {
        Ok("single-process".into())
    }

    #[inline(always)]
    fn username(self) -> Result<OsString> {
        Ok("single-process".into())
    }

    #[inline(always)]
    fn devicename(self) -> Result<OsString> {
        // TODO: we may be able to obtain this information from the environment or from network configuration, if this is useful
        Ok("hermit-device".into())
    }

    #[inline(always)]
    fn hostname(self) -> Result<String> {
        // TODO: we may be able to obtain this information from the environment or from network configuration, if this is useful
        Ok("hermit-device".into())
    }

    #[inline(always)]
    fn distro(self) -> Result<String> {
        Ok("hermit".to_string())
    }

    #[inline(always)]
    fn desktop_env(self) -> Option<DesktopEnv> {
        None
    }

    #[inline(always)]
    fn platform(self) -> Platform {
        Platform::Hermit
    }

    #[inline(always)]
    fn arch(self) -> Result<Arch> {
        Ok(
            Arch::Unknown("unknown architecture".into()),
        )
    }
}
