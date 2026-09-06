//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/s390/util/machine.c
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

#[no_mangle]
pub unsafe extern "C" fn arch__fix_module_text_start(start: *mut u64, size: *mut u64, name: *const c_char) -> c_int {
    int arch__fix_module_text_start(u64 *start, u64 *size, const char *name)
    {
    let mut m_start: u64 = *start;
    char path[PATH_MAX];
    snprintf(path, PATH_MAX, "module/%.*s/sections/.text",
    (int)strlen(name) - 2, name + 1);
    if (sysfs__read_ull(path, (unsigned long long *)start) < 0) {
    pr_debug2("Using module %s start:%#lx\n", path, m_start);
// start = m_start;
    } else {
// Successful read of the modules segment text start address.
// Calculate difference between module start address
// in memory and module text segment start address.
// For example module load address is 0x3ff8011b000
// (from /proc/modules) and module text segment start
// address is 0x3ff8011b870 (from file above).
//
// Adjust the module size and subtract the GOT table
// size located at the beginning of the module.
//
// size -= (*start - m_start);
    }
    return 0;
    }
