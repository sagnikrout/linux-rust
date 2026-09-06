//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/arm64/tests/cpuid-match.c
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

    int test__cpuid_match(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
// midr with no leading zeros matches
    if (strcmp_cpuid_str("0x410fd0c0", "0x00000000410fd0c0"))
    return -1;
// Upper case matches
    if (strcmp_cpuid_str("0x410fd0c0", "0x00000000410FD0C0"))
    return -1;
// r0p0 = r0p0 matches
    if (strcmp_cpuid_str("0x00000000410fd480", "0x00000000410fd480"))
    return -1;
// r0p1 > r0p0 matches
    if (strcmp_cpuid_str("0x00000000410fd480", "0x00000000410fd481"))
    return -1;
// r1p0 > r0p0 matches
    if (strcmp_cpuid_str("0x00000000410fd480", "0x00000000411fd480"))
    return -1;
// r0p0 < r0p1 doesn't match
    if (!strcmp_cpuid_str("0x00000000410fd481", "0x00000000410fd480"))
    return -1;
// r0p0 < r1p0 doesn't match
    if (!strcmp_cpuid_str("0x00000000411fd480", "0x00000000410fd480"))
    return -1;
// Different CPU doesn't match
    if (!strcmp_cpuid_str("0x00000000410fd4c0", "0x00000000430f0af0"))
    return -1;
    return 0;
    }
