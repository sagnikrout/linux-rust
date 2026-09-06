//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/test_FISTTP.c
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

pub const _GNU_SOURCE: c_int = 1;

pub const __USE_GNU: c_int = 1;

    let mut res64: c_ulonglong = -1;
    let mut res32: c_uint = -1;
    let mut res16: c_ushort = -1;
#[no_mangle]
pub unsafe extern "C" fn test() -> c_int {
    int test(void)
    {
    int ex;
    feclearexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    asm volatile ("\n"
    "	fld1""\n"
    "	fisttps	res16""\n"
    "	fld1""\n"
    "	fisttpl	res32""\n"
    "	fld1""\n"
    "	fisttpll res64""\n"
    : : : "memory"
    );
    if (res16 != 1 || res32 != 1 || res64 != 1) {
    printf("[BAD]\tfisttp 1\n");
    return 1;
    }
    ex = fetestexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    if (ex != 0) {
    printf("[BAD]\tfisttp 1: wrong exception state\n");
    return 1;
    }
    feclearexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    asm volatile ("\n"
    "	fldpi""\n"
    "	fisttps	res16""\n"
    "	fldpi""\n"
    "	fisttpl	res32""\n"
    "	fldpi""\n"
    "	fisttpll res64""\n"
    : : : "memory"
    );
    if (res16 != 3 || res32 != 3 || res64 != 3) {
    printf("[BAD]\tfisttp pi\n");
    return 1;
    }
    ex = fetestexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    if (ex != FE_INEXACT) {
    printf("[BAD]\tfisttp pi: wrong exception state\n");
    return 1;
    }
    feclearexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    asm volatile ("\n"
    "	fldpi""\n"
    "	fchs""\n"
    "	fisttps	res16""\n"
    "	fldpi""\n"
    "	fchs""\n"
    "	fisttpl	res32""\n"
    "	fldpi""\n"
    "	fchs""\n"
    "	fisttpll res64""\n"
    : : : "memory"
    );
    if (res16 != 0xfffd || res32 != 0xfffffffd || res64 != 0xfffffffffffffffdULL) {
    printf("[BAD]\tfisttp -pi\n");
    return 1;
    }
    ex = fetestexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    if (ex != FE_INEXACT) {
    printf("[BAD]\tfisttp -pi: wrong exception state\n");
    return 1;
    }
    feclearexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    asm volatile ("\n"
    "	fldln2""\n"
    "	fisttps	res16""\n"
    "	fldln2""\n"
    "	fisttpl	res32""\n"
    "	fldln2""\n"
    "	fisttpll res64""\n"
    : : : "memory"
    );
// Test truncation to zero (round-to-nearest would give 1 here)
    if (res16 != 0 || res32 != 0 || res64 != 0) {
    printf("[BAD]\tfisttp ln2\n");
    return 1;
    }
    ex = fetestexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    if (ex != FE_INEXACT) {
    printf("[BAD]\tfisttp ln2: wrong exception state\n");
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sighandler(sig: c_int) {
    void sighandler(int sig)
    {
    printf("[FAIL]\tGot signal %d, exiting\n", sig);
    exit(1);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char, envp: *mut c_char) -> c_int {
    int main(int argc, char **argv, char **envp)
    {
    let mut err: c_int = 0;
// SIGILL triggers on 32-bit kernels w/o fisttp emulation
// when run with "no387 nofxsr". Other signals are caught
// just in case.
//
    signal(SIGILL, sighandler);
    signal(SIGFPE, sighandler);
    signal(SIGSEGV, sighandler);
    printf("[RUN]\tTesting fisttp instructions\n");
    err |= test();
    if (!err)
    printf("[OK]\tfisttp\n");
    else
    printf("[FAIL]\tfisttp errors: %d\n", err);
    return err;
    }
