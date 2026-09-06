//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/math/fpu_denormal.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright IBM Corp. 2020
//
// This test attempts to cause a FP denormal exception on POWER8 CPUs. Unfortunately
// if the denormal handler is not configured or working properly, this can cause a bad
// crash in kernel mode when the kernel tries to save FP registers when the process
// exits.
//

#[no_mangle]
unsafe extern "C" fn test_denormal_fpu() -> c_int {
    static int test_denormal_fpu(void)
    {
    unsigned int m32;
    unsigned long m64;
    volatile float f;
    volatile double d;
// try to induce lfs <denormal> ; stfd
    m32 = 0x00715fcf; /* random denormal */
    memcpy((float *)&f, &m32, sizeof(f));
    d = f;
    memcpy(&m64, (double *)&d, sizeof(d));
    FAIL_IF((long)(m64 != 0x380c57f3c0000000)); /* renormalised value */
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(test_denormal_fpu, "fpu_denormal");
    }
