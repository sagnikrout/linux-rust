
// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------

// SPDX-License-Identifier: Apache-2.0 OR MIT

use alloc::borrow::Cow;
use core::fmt;
use proc_macro2::{Ident, Span};

/// Specialized formatting trait used by `format_ident!`.
///
/// [`Ident`] arguments formatted using this trait will have their `r#` prefix
/// stripped, if present.
///
/// See [`format_ident!`] for more information.
///
/// [`format_ident!`]: crate::format_ident
pub trait IdentFragment {
    /// Format this value as an identifier fragment.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;

    /// Span associated with this `IdentFragment`.
    ///
    /// If non-`None`, may be inherited by formatted identifiers.
    fn span(&self) -> Option<Span> {
        None
    }
}

impl<T: IdentFragment + ?Sized> IdentFragment for &T {
    fn span(&self) -> Option<Span> {
        <T as IdentFragment>::span(*self)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        IdentFragment::fmt(*self, f)
    }
}

impl<T: IdentFragment + ?Sized> IdentFragment for &mut T {
    fn span(&self) -> Option<Span> {
        <T as IdentFragment>::span(*self)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        IdentFragment::fmt(*self, f)
    }
}

impl IdentFragment for Ident {
    fn span(&self) -> Option<Span> {
        Some(self.span())
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = self.to_string();
        if let Some(id) = id.strip_prefix("r#") {
            fmt::Display::fmt(id, f)
        } else {
            fmt::Display::fmt(&id[..], f)
        }
    }
}

impl<T> IdentFragment for Cow<'_, T>
where
    T: IdentFragment + ToOwned + ?Sized,
{
    fn span(&self) -> Option<Span> {
        T::span(self)
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        T::fmt(self, f)
    }
}

// Limited set of types which this is implemented for, as we want to avoid types
// which will often include non-identifier characters in their `Display` impl.
macro_rules! ident_fragment_display {
    ($($T:ty),*) => {
        $(
            impl IdentFragment for $T {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Display::fmt(self, f)
                }
            }
        )*
    };
}

ident_fragment_display!(bool, str, String, char);
ident_fragment_display!(u8, u16, u32, u64, u128, usize);
