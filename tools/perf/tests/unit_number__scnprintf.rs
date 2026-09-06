//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/unit_number__scnprintf.c
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
unsafe extern "C" fn test__unit_number__scnprint(__maybe_unused: *mut *mut test_suite t, __maybe_unused: int subtest) -> c_int {
    static int test__unit_number__scnprint(struct test_suite *t __maybe_unused, int subtest __maybe_unused)
    {
    struct {
    u64		 n;
    const char	*str;
    } test[] = {
    { 1,			"1B"	},
    { 10*1024,		"10K"	},
    { 20*1024*1024,		"20M"	},
    { 30*1024*1024*1024ULL,	"30G"	},
    { 0,			"0B"	},
    { 0,			core::ptr::null_mut()	},
    };
    let mut i: unsigned = 0;
    while (test[i].str) {
    char buf[100];
    unit_number__scnprintf(buf, sizeof(buf), test[i].n);
    pr_debug("n %" PRIu64 ", str '%s', buf '%s'\n",
    test[i].n, test[i].str, buf);
    if (strcmp(test[i].str, buf))
    return TEST_FAIL;
    i++;
    }
    return TEST_OK;
    }
    DEFINE_SUITE("unit_number__scnprintf", unit_number__scnprint);
