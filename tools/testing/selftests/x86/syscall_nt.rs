//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/syscall_nt.c
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
// syscall_nt.c - checks syscalls with NT set
// Copyright (c) 2014-2015 Andrew Lutomirski
//
// Some obscure user-space code requires the ability to make system calls
// with FLAGS.NT set.  Make sure it works.
//

    static unsigned int nerrs;
#[no_mangle]
unsafe extern "C" fn sigtrap(sig: c_int, si: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigtrap(int sig, siginfo_t *si, void *ctx_void)
    {
    }
#[no_mangle]
unsafe extern "C" fn do_it(extraflags: c_ulong) {
    static void do_it(unsigned long extraflags)
    {
    unsigned long flags;
    set_eflags(get_eflags() | extraflags);
    syscall(SYS_getpid);
    flags = get_eflags();
    set_eflags(X86_EFLAGS_IF | X86_EFLAGS_FIXED);
    if ((flags & extraflags) == extraflags) {
    printf("[OK]\tThe syscall worked and flags are still set\n");
    } else {
    printf("[FAIL]\tThe syscall worked but flags were cleared (flags = 0x%lx but expected 0x%lx set)\n",
    flags, extraflags);
    nerrs++;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    printf("[RUN]\tSet NT and issue a syscall\n");
    do_it(X86_EFLAGS_NT);
    printf("[RUN]\tSet AC and issue a syscall\n");
    do_it(X86_EFLAGS_AC);
    printf("[RUN]\tSet NT|AC and issue a syscall\n");
    do_it(X86_EFLAGS_NT | X86_EFLAGS_AC);
//
// Now try it again with TF set -- TF forces returns via IRET in all
// cases except non-ptregs-using 64-bit full fast path syscalls.
//
    sethandler(SIGTRAP, sigtrap, 0);
    printf("[RUN]\tSet TF and issue a syscall\n");
    do_it(X86_EFLAGS_TF);
    printf("[RUN]\tSet NT|TF and issue a syscall\n");
    do_it(X86_EFLAGS_NT | X86_EFLAGS_TF);
    printf("[RUN]\tSet AC|TF and issue a syscall\n");
    do_it(X86_EFLAGS_AC | X86_EFLAGS_TF);
    printf("[RUN]\tSet NT|AC|TF and issue a syscall\n");
    do_it(X86_EFLAGS_NT | X86_EFLAGS_AC | X86_EFLAGS_TF);
//
// Now try DF.  This is evil and it's plausible that we will crash
// glibc, but glibc would have to do something rather surprising
// for this to happen.
//
    printf("[RUN]\tSet DF and issue a syscall\n");
    do_it(X86_EFLAGS_DF);
    printf("[RUN]\tSet TF|DF and issue a syscall\n");
    do_it(X86_EFLAGS_TF | X86_EFLAGS_DF);
    let mut nerrs: return = = 0 ? 0 : 1;
    }
