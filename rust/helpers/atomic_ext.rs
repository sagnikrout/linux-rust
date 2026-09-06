//! Automatically rewritten from C to Rust
//! Source: rust/helpers/atomic_ext.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

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

    __rust_helper type rust_helper_atomic_##tname##_read(type *ptr)			\
    {										\
    return READ_ONCE(*ptr);							\
    }

    __rust_helper void rust_helper_atomic_##tname##_set(type *ptr, type val)	\
    {										\
    WRITE_ONCE(*ptr, val);							\
    }

    __rust_helper type rust_helper_atomic_##tname##_read_acquire(type *ptr)		\
    {										\
    return smp_load_acquire(ptr);						\
    }

    __rust_helper void rust_helper_atomic_##tname##_set_release(type *ptr, type val)\
    {										\
    smp_store_release(ptr, val);						\
    }

    GEN_READ_HELPER(tname, type)						\
    GEN_SET_HELPER(tname, type)						\
    GEN_READ_ACQUIRE_HELPER(tname, type)					\
    GEN_SET_RELEASE_HELPER(tname, type)					\
    GEN_READ_SET_HELPERS(i8, s8)
    GEN_READ_SET_HELPERS(i16, s16)
    GEN_READ_SET_HELPERS(ptr, const void *)
//
// xchg helpers depend on ARCH_SUPPORTS_ATOMIC_RMW and on the
// architecture provding xchg() support for i8 and i16.
//
// The architectures that currently support Rust (x86_64, armv7,
// arm64, riscv, and loongarch) satisfy these requirements.
//

    __rust_helper type								\
    rust_helper_atomic_##tname##_xchg##suffix(type *ptr, type new)			\
    {										\
    return xchg##suffix(ptr, new);					\
    }

    GEN_XCHG_HELPER(tname, type, )						\
    GEN_XCHG_HELPER(tname, type, _acquire)					\
    GEN_XCHG_HELPER(tname, type, _release)					\
    GEN_XCHG_HELPER(tname, type, _relaxed)					\
    GEN_XCHG_HELPERS(i8, s8)
    GEN_XCHG_HELPERS(i16, s16)
    GEN_XCHG_HELPERS(ptr, const void *)
//
// try_cmpxchg helpers depend on ARCH_SUPPORTS_ATOMIC_RMW and on the
// architecture provding try_cmpxchg() support for i8 and i16.
//
// The architectures that currently support Rust (x86_64, armv7,
// arm64, riscv, and loongarch) satisfy these requirements.
//

    __rust_helper bool								\
    rust_helper_atomic_##tname##_try_cmpxchg##suffix(type *ptr, type *old, type new)\
    {										\
    return try_cmpxchg##suffix(ptr, old, new);				\
    }

    GEN_TRY_CMPXCHG_HELPER(tname, type, )					\
    GEN_TRY_CMPXCHG_HELPER(tname, type, _acquire)				\
    GEN_TRY_CMPXCHG_HELPER(tname, type, _release)				\
    GEN_TRY_CMPXCHG_HELPER(tname, type, _relaxed)				\
    GEN_TRY_CMPXCHG_HELPERS(i8, s8)
    GEN_TRY_CMPXCHG_HELPERS(i16, s16)
    GEN_TRY_CMPXCHG_HELPERS(ptr, const void *)
