
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

// SPDX-License-Identifier: GPL-2.0

//! Safety related APIs.

/// Checks that a precondition of an unsafe function is followed.
///
/// The check is enabled at runtime if debug assertions (`CONFIG_RUST_DEBUG_ASSERTIONS`)
/// are enabled. Otherwise, this macro is a no-op.
///
/// # Examples
///
/// ```no_run
/// use kernel::unsafe_precondition_assert;
///
/// struct RawBuffer<T: Copy, const N: usize> {
///     data: [T; N],
/// }
///
/// impl<T: Copy, const N: usize> RawBuffer<T, N> {
///     /// # Safety
///     ///
///     /// The caller must ensure that `index` is less than `N`.
///     unsafe fn set_unchecked(&mut self, index: usize, value: T) {
///         unsafe_precondition_assert!(
///             index < N,
///             "RawBuffer::set_unchecked() requires index ({index}) < N ({N})"
///         );
///
///         // SAFETY: By the safety requirements of this function, `index` is valid.
///         unsafe {
///             *self.data.get_unchecked_mut(index) = value;
///         }
///     }
/// }
/// ```
///
/// # Panics
///
/// Panics if the expression is evaluated to [`false`] at runtime.
#[macro_export]
macro_rules! unsafe_precondition_assert {
    ($cond:expr $(,)?) => {
        $crate::unsafe_precondition_assert!(@inner $cond, ::core::stringify!($cond))
    };

    ($cond:expr, $($arg:tt)+) => {
        $crate::unsafe_precondition_assert!(@inner $cond, $crate::prelude::fmt!($($arg)+))
    };

    (@inner $cond:expr, $msg:expr) => {
        ::core::debug_assert!($cond, "unsafe precondition violated: {}", $msg)
    };
}
