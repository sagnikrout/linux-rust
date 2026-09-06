//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cpu_mask.c
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

    let mut duration: static int = 0;
#[no_mangle]
unsafe extern "C" fn validate_mask(case_nr: c_int, exp: *const c_char, mask: *mut bool, n: c_int) {
    static void validate_mask(int case_nr, const char *exp, bool *mask, int n)
    {
    int i;
    for (i = 0; exp[i]; i++) {
    if (exp[i] == '1') {
    if (CHECK(i + 1 > n, "mask_short",
    "case #%d: mask too short, got n=%d, need at least %d\n",
    case_nr, n, i + 1))
    return;
    CHECK(!mask[i], "cpu_not_set",
    "case #%d: mask differs, expected cpu#%d SET\n",
    case_nr, i);
    } else {
    CHECK(i < n && mask[i], "cpu_set",
    "case #%d: mask differs, expected cpu#%d UNSET\n",
    case_nr, i);
    }
    }
    CHECK(i < n, "mask_long",
    "case #%d: mask too long, got n=%d, expected at most %d\n",
    case_nr, n, i);
    }
    static struct {
    const char *cpu_mask;
    const char *expect;
    bool fails;
    } test_cases[] = {
    { "0\n", "1", false },
    { "0,2\n", "101", false },
    { "0-2\n", "111", false },
    { "0-2,3-4\n", "11111", false },
    { "0", "1", false },
    { "0-2", "111", false },
    { "0,2", "101", false },
    { "0,1-3", "1111", false },
    { "0,1,2,3", "1111", false },
    { "0,2-3,5", "101101", false },
    { "3-3", "0001", false },
    { "2-4,6,9-10", "00111010011", false },
// failure cases
    { "", "", true },
    { "0-", "", true },
    { "0 ", "", true },
    { "0_1", "", true },
    { "1-0", "", true },
    { "-1", "", true },
    };
#[no_mangle]
pub unsafe extern "C" fn test_cpu_mask() {
    void test_cpu_mask()
    {
    int i, err, n;
    bool *mask;
    for (i = 0; i < ARRAY_SIZE(test_cases); i++) {
    mask = core::ptr::null_mut();
    err = parse_cpu_mask_str(test_cases[i].cpu_mask, &mask, &n);
    if (test_cases[i].fails) {
    CHECK(!err, "should_fail",
    "case #%d: parsing should fail!\n", i + 1);
    } else {
    if (CHECK(err, "parse_err",
    "case #%d: cpu mask parsing failed: %d\n",
    i + 1, err))
    continue;
    validate_mask(i + 1, test_cases[i].expect, mask, n);
    }
    free(mask);
    }
    }
