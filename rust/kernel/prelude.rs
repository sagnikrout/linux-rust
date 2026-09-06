
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

//! The `kernel` prelude.
//!
//! These are the most common items used by Rust code in the kernel,
//! intended to be imported by all Rust code, for convenience.
//!
//! # Examples
//!
//! ```
//! use kernel::prelude::*;
//! ```

#[doc(no_inline)]
pub use core::{
    mem::{
        align_of,
        align_of_val,
        size_of,
        size_of_val, //
    },
    pin::Pin, //
};

#[doc(no_inline)]
pub use ::ffi::{
    c_char,
    c_int,
    c_long,
    c_longlong,
    c_schar,
    c_short,
    c_uchar,
    c_uint,
    c_ulong,
    c_ulonglong,
    c_ushort,
    c_void,
    CStr, //
};

#[doc(no_inline)]
pub use macros::{
    export,
    fmt,
    kunit_tests,
    module,
    vtable, //
};

#[doc(no_inline)]
pub use pin_init::{
    init,
    pin_data,
    pin_init,
    pinned_drop,
    InPlaceWrite,
    Init,
    PinInit,
    Zeroable, //
};

#[doc(no_inline)]
pub use zerocopy::{
    FromBytes,
    IntoBytes, //
};

#[doc(no_inline)]
pub use zerocopy_derive::{
    FromBytes,
    IntoBytes, //
};

#[doc(no_inline)]
pub use super::{
    alloc::{
        flags::*,
        Box,
        KBox,
        KVBox,
        KVVec,
        KVec,
        VBox,
        VVec,
        Vec, //
    },
    build_assert::{
        build_assert,
        build_error,
        const_assert,
        static_assert, //
    },
    current,
    dev_alert,
    dev_crit,
    dev_dbg,
    dev_emerg,
    dev_err,
    dev_info,
    dev_notice,
    dev_warn,
    error::{
        code::*,
        Error,
        Result, //
    },
    init::InPlaceInit,
    pr_alert,
    pr_crit,
    pr_debug,
    pr_emerg,
    pr_err,
    pr_info,
    pr_notice,
    pr_warn,
    str::CStrExt as _,
    try_init,
    try_pin_init,
    uaccess::UserPtr,
    ThisModule, //
};

// `super::std_vendor` is hidden, which makes the macro inline for some reason.
#[doc(no_inline)]
pub use super::dbg;
