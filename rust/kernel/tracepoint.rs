
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

// Copyright (C) 2024 Google LLC.

//! Logic for tracepoints.

/// Declare the Rust entry point for a tracepoint.
///
/// This macro generates an unsafe function that calls into C, and its safety requirements will be
/// whatever the relevant C code requires. To document these safety requirements, you may add
/// doc-comments when invoking the macro.
#[macro_export]
macro_rules! declare_trace {
    ($($(#[$attr:meta])* $pub:vis unsafe fn $name:ident($($argname:ident : $argtyp:ty),* $(,)?);)*) => {$(
        $( #[$attr] )*
        #[inline(always)]
        $pub unsafe fn $name($($argname : $argtyp),*) {
            #[cfg(CONFIG_TRACEPOINTS)]
            {
                // SAFETY: It's always okay to query the static key for a tracepoint.
                let should_trace = unsafe {
                    $crate::macros::paste! {
                        $crate::jump_label::static_branch_unlikely!(
                            $crate::bindings::[< __tracepoint_ $name >],
                            $crate::bindings::tracepoint,
                            key
                        )
                    }
                };

                if should_trace {
                    $crate::macros::paste! {
                        // SAFETY: The caller guarantees that it is okay to call this tracepoint.
                        unsafe { $crate::bindings::[< rust_do_trace_ $name >]($($argname),*) };
                    }
                }
            }

            #[cfg(not(CONFIG_TRACEPOINTS))]
            {
                // If tracepoints are disabled, insert a trivial use of each argument
                // to avoid unused argument warnings.
                $( let _unused = $argname; )*
            }
        }
    )*}
}

pub use declare_trace;
