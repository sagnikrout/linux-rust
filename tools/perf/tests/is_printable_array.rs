//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/is_printable_array.c
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
unsafe extern "C" fn test__is_printable_array(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__is_printable_array(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    char buf1[] = { 'k', 'r', 4, 'v', 'a', 0 };
    char buf2[] = { 'k', 'r', 'a', 'v', 4, 0 };
    struct {
    char		*buf;
    unsigned int	 len;
    int		 ret;
    } t[] = {
    { (char *) "krava",	sizeof("krava"),	1 },
    { (char *) "krava",	sizeof("krava") - 1,	0 },
    { (char *) "",		sizeof(""),		1 },
    { (char *) "",		0,			0 },
    { core::ptr::null_mut(),			0,			0 },
    { buf1,			sizeof(buf1),		0 },
    { buf2,			sizeof(buf2),		0 },
    };
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(t); i++) {
    int ret;
    ret = is_printable_array((char *) t[i].buf, t[i].len);
    if (ret != t[i].ret) {
    pr_err("failed: test %u\n", i);
    return TEST_FAIL;
    }
    }
    return TEST_OK;
    }
    DEFINE_SUITE("is_printable_array", is_printable_array);
