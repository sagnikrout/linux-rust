//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/futex_val3.c
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


// SPDX-License-Identifier: LGPL-2.1

pub const FUTEX_BITSET_MATCH_ANY: c_uint = 0xffffffff;

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_futex_val3(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_futex_val3(char *bf, size_t size, struct syscall_arg *arg)
    {
    const char *prefix = "FUTEX_BITSET_";
    let mut bitset: c_uint = arg.val;
    if (bitset == FUTEX_BITSET_MATCH_ANY)
    return scnprintf(bf, size, "%s%s", arg.show_string_prefix ? prefix : "", "MATCH_ANY");
    return scnprintf(bf, size, "%#xd", bitset);
    }
