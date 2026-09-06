//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/test_FCMOV.c
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

    long double __attribute__((noinline)) insn(long flags) \
    {						\
    long double out;			\
    asm ("\n"				\
    "	push	%1""\n"			\
    "	popf""\n"			\
    "	fldpi""\n"			\
    "	fld1""\n"			\
    "	" #insn " %%st(1), %%st" "\n"	\
    "	ffree	%%st(1)" "\n"		\
    : "=t" (out)				\
    : "r" (flags)				\
    );					\
    return out;				\
    }
    TEST(fcmovb)
    TEST(fcmove)
    TEST(fcmovbe)
    TEST(fcmovu)
    TEST(fcmovnb)
    TEST(fcmovne)
    TEST(fcmovnbe)
    TEST(fcmovnu)
    enum {
    CF = 1 << 0,
    PF = 1 << 2,
    ZF = 1 << 6,
    };
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
// SIGILL triggers on 32-bit kernels w/o fcomi emulation
// when run with "no387 nofxsr". Other signals are caught
// just in case.
//
    signal(SIGILL, sighandler);
    signal(SIGFPE, sighandler);
    signal(SIGSEGV, sighandler);
    printf("[RUN]\tTesting fcmovCC instructions\n");
// If fcmovCC() returns 1.0, the move wasn't done
    err |= !(fcmovb(0)   == 1.0); err |= !(fcmovnb(0)  != 1.0);
    err |= !(fcmove(0)   == 1.0); err |= !(fcmovne(0)  != 1.0);
    err |= !(fcmovbe(0)  == 1.0); err |= !(fcmovnbe(0) != 1.0);
    err |= !(fcmovu(0)   == 1.0); err |= !(fcmovnu(0)  != 1.0);
    err |= !(fcmovb(CF)  != 1.0); err |= !(fcmovnb(CF)  == 1.0);
    err |= !(fcmove(CF)  == 1.0); err |= !(fcmovne(CF)  != 1.0);
    err |= !(fcmovbe(CF) != 1.0); err |= !(fcmovnbe(CF) == 1.0);
    err |= !(fcmovu(CF)  == 1.0); err |= !(fcmovnu(CF)  != 1.0);
    err |= !(fcmovb(ZF)  == 1.0); err |= !(fcmovnb(ZF)  != 1.0);
    err |= !(fcmove(ZF)  != 1.0); err |= !(fcmovne(ZF)  == 1.0);
    err |= !(fcmovbe(ZF) != 1.0); err |= !(fcmovnbe(ZF) == 1.0);
    err |= !(fcmovu(ZF)  == 1.0); err |= !(fcmovnu(ZF)  != 1.0);
    err |= !(fcmovb(PF)  == 1.0); err |= !(fcmovnb(PF)  != 1.0);
    err |= !(fcmove(PF)  == 1.0); err |= !(fcmovne(PF)  != 1.0);
    err |= !(fcmovbe(PF) == 1.0); err |= !(fcmovnbe(PF) != 1.0);
    err |= !(fcmovu(PF)  != 1.0); err |= !(fcmovnu(PF)  == 1.0);
    if (!err)
    printf("[OK]\tfcmovCC\n");
    else
    printf("[FAIL]\tfcmovCC errors: %d\n", err);
    return err;
    }
