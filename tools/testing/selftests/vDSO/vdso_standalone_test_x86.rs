//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/vDSO/vdso_standalone_test_x86.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// vdso_test_gettimeofday.c: Sample code to test parse_vdso.c and
// vDSO gettimeofday()
// Copyright (c) 2014 Andy Lutomirski
//
// Compile with:
// gcc -std=gnu99 vdso_test_gettimeofday.c parse_vdso_gettimeofday.c
//
// Tested on x86, 32-bit and 64-bit.  It may work on other architectures, too.
//

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *version = versions[VDSO_VERSION];
    const char **name = (const char **)&names[VDSO_NAMES];
    let mut sysinfo_ehdr: c_ulong = getauxval(AT_SYSINFO_EHDR);
    if (!sysinfo_ehdr) {
    printf("AT_SYSINFO_EHDR is not present!\n");
    return KSFT_SKIP;
    }
    vdso_init_from_sysinfo_ehdr(getauxval(AT_SYSINFO_EHDR));
// Find gettimeofday.
    typedef long (*gtod_t)(struct timeval *tv, struct timezone *tz);
    let mut gtod: gtod_t = (gtod_t)vdso_sym(version, name[0]);
    if (!gtod) {
    printf("Could not find %s\n", name[0]);
    return KSFT_SKIP;
    }
    struct timeval tv;
    let mut ret: c_long = VDSO_CALL(gtod, 2, &tv, 0);
    if (ret == 0) {
    printf("The time is %lld.%06lld\n",
    (long long)tv.tv_sec, (long long)tv.tv_usec);
    } else {
    printf("%s failed\n", name[0]);
    return KSFT_FAIL;
    }
    return 0;
    }
