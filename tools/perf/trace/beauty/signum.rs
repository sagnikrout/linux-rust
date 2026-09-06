//! Automatically rewritten from C to Rust
//! Source: tools/perf/trace/beauty/signum.c
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


// SPDX-License-Identifier: LGPL-2.1

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_signum(bf: *mut c_char, size: usize, arg: *mut syscall_arg) -> usize {
    size_t syscall_arg__scnprintf_signum(char *bf, size_t size, struct syscall_arg *arg)
    {
    let mut show_prefix: bool = arg.show_string_prefix;
    const char *prefix = "SIG";
    let mut sig: c_int = arg.val;
    switch (sig) {

    P_SIGNUM(HUP);
    P_SIGNUM(INT);
    P_SIGNUM(QUIT);
    P_SIGNUM(ILL);
    P_SIGNUM(TRAP);
    P_SIGNUM(ABRT);
    P_SIGNUM(BUS);
    P_SIGNUM(FPE);
    P_SIGNUM(KILL);
    P_SIGNUM(USR1);
    P_SIGNUM(SEGV);
    P_SIGNUM(USR2);
    P_SIGNUM(PIPE);
    P_SIGNUM(ALRM);
    P_SIGNUM(TERM);
    P_SIGNUM(CHLD);
    P_SIGNUM(CONT);
    P_SIGNUM(STOP);
    P_SIGNUM(TSTP);
    P_SIGNUM(TTIN);
    P_SIGNUM(TTOU);
    P_SIGNUM(URG);
    P_SIGNUM(XCPU);
    P_SIGNUM(XFSZ);
    P_SIGNUM(VTALRM);
    P_SIGNUM(PROF);
    P_SIGNUM(WINCH);
    P_SIGNUM(IO);
    P_SIGNUM(PWR);
    P_SIGNUM(SYS);

    P_SIGNUM(EMT);

    P_SIGNUM(STKFLT);

    P_SIGNUM(SWI);

    default: break;
    }
    return scnprintf(bf, size, "%#x", sig);
    }
