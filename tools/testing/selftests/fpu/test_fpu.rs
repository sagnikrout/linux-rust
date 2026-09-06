//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/fpu/test_fpu.c
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


// SPDX-License-Identifier: GPL-2.0+
// This testcase operates with the test_fpu kernel driver.
// It modifies the FPU control register in user mode and calls the kernel
// module to perform floating point operations in the kernel. The control
// register value should be independent between kernel and user mode.
//
// Macro flag: #define _GNU_SOURCE

    const char *test_fpu_path = "/sys/kernel/debug/selftest_helpers/test_fpu";
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    char dummy[1];
    let mut fd: c_int = open(test_fpu_path, O_RDONLY);
    if (fd < 0) {
    printf("[SKIP]\tcan't access %s: %s\n",
    test_fpu_path, strerror(errno));
    return 0;
    }
    if (read(fd, dummy, 1) < 0) {
    printf("[FAIL]\taccess with default rounding mode failed\n");
    return 1;
    }
    fesetround(FE_DOWNWARD);
    if (read(fd, dummy, 1) < 0) {
    printf("[FAIL]\taccess with downward rounding mode failed\n");
    return 2;
    }
    if (fegetround() != FE_DOWNWARD) {
    printf("[FAIL]\tusermode rounding mode clobbered\n");
    return 3;
    }
// Note: the tests up to this point are quite safe and will only return
// an error. But the exception mask setting can cause misbehaving kernel
// to crash.
//
    feclearexcept(FE_ALL_EXCEPT);
    feenableexcept(FE_ALL_EXCEPT);
    if (read(fd, dummy, 1) < 0) {
    printf("[FAIL]\taccess with fpu exceptions unmasked failed\n");
    return 4;
    }
    if (fegetexcept() != FE_ALL_EXCEPT) {
    printf("[FAIL]\tusermode fpu exception mask clobbered\n");
    return 5;
    }
    printf("[OK]\ttest_fpu\n");
    return 0;
    }
