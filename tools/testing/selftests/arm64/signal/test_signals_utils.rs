//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/arm64/signal/test_signals_utils.h
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
// Copyright (C) 2019 ARM Limited

extern "C" {
    pub fn test_init(td: *mut tdescr) -> c_int;
}
extern "C" {
    pub fn test_setup(td: *mut tdescr) -> c_int;
}
extern "C" {
    pub fn test_cleanup(td: *mut tdescr);
}
extern "C" {
    pub fn test_run(td: *mut tdescr) -> c_int;
}
extern "C" {
    pub fn test_result(td: *mut tdescr);
}

pub const __NR_prctl: c_int = 167;

//
// The prctl takes 1 argument but we need to ensure that the other
// values passed in registers to the syscall are zero since the kernel
// validates them.
//

extern "C" {
    pub fn volatile(%0: "mrs, (val): S3_3_C2_C5_1" : "=r") -> asm;
}

extern "C" {
    pub fn volatile(%0: "mrs, "=r"(val): " SYS_POR_EL0 "\n" :) -> asm;
}
extern "C" {
    pub fn volatile(": "msr " SYS_POR_EL0, "r"(val): %0\n" ::) -> asm;
}
//
// Obtaining a valid and full-blown ucontext_t from userspace is tricky:
// libc getcontext does() not save all the regs and messes with some of
// them (pstate value in particular is not reliable).
//
// Here we use a service signal to grab the ucontext_t from inside a
// dedicated signal handler, since there, it is populated by Kernel
// itself in setup_sigframe(). The grabbed context is then stored and
// made available in td->live_uc.
//
// As service-signal is used a SIGTRAP induced by a 'brk' instruction,
// because here we have to avoid syscalls to trigger the signal since
// they would cause any SVE sigframe content (if any) to be removed.
//
// Anyway this function really serves a dual purpose:
//
// 1. grab a valid sigcontext into td->live_uc for result analysis: in
// such case it returns 1.
//
// 2. detect if, somehow, a previously grabbed live_uc context has been
// used actively with a sigreturn: in such a case the execution would have
// magically resumed in the middle of this function itself (seen_already==1):
// in such a case return 0, since in fact we have not just simply grabbed
// the context.
//
// This latter case is useful to detect when a fake_sigreturn test-case has
// unexpectedly survived without hitting a SEGV.
//
// Note that the case of runtime dynamically sized sigframes (like in SVE
// context) is still NOT addressed: sigframe size is supposed to be fixed
// at sizeof(ucontext_t).
//
// it's a genuine invocation..reinit
//
// This is a memset() but we don't want the compiler to
// optimise it into either instructions or a library call
// which might be incompatible with streaming mode.
//
// Grab ucontext_t triggering a SIGTRAP.
//
// Note that:
// - live_uc_valid is declared volatile sig_atomic_t in
// struct tdescr since it will be changed inside the
// sig_copyctx handler
// - the additional 'memory' clobber is there to avoid possible
// compiler's assumption on live_uc_valid and the content
// pointed by dest_uc, which are all changed inside the signal
// handler
// - BRK causes a debug exception which is handled by the Kernel
// and finally causes the SIGTRAP signal to be delivered to this
// test thread. Since such delivery happens on the ret_to_user()
// /do_notify_resume() debug exception return-path, we are sure
// that the registered SIGTRAP handler has been run to completion
// before the execution path is restored here: as a consequence
// we can be sure that the volatile sig_atomic_t live_uc_valid
// carries a meaningful result. Being in a single thread context
// we'll also be sure that any access to memory modified by the
// handler (namely ucontext_t) will be visible once returned.
// - note that since we are using a breakpoint instruction here
// to cause a SIGTRAP, the ucontext_t grabbed from the signal
// handler would naturally contain a PC pointing exactly to this
// BRK line, which means that, on return from the signal handler,
// or if we place the ucontext_t on the stack to fake a sigreturn,
// we'll end up in an infinite loop of BRK-SIGTRAP-handler.
// For this reason we take care to artificially move forward the
// PC to the next instruction while inside the signal handler.
//
// If we were grabbing a streaming mode context then we may
// have entered streaming mode behind the system's back and
// libc or compiler generated code might decide to do
// something invalid in streaming mode, or potentially even
// the state of ZA.  Issue a SMSTOP to exit both now we have
// grabbed the state.
//
extern "C" {
    pub fn volatile(S0_3_C4_C6_3: "msr, _arg: xzr") -> asm;
}
//
// If we get here with seen_already==1 it implies the td->live_uc
// context has been used to get back here....this probably means
// a test has failed to cause a SEGV...anyway live_uc does not
// point to a just acquired copy of ucontext_t...so return 0
//
extern "C" {
    pub fn fake_sigreturn(sigframe: *mut c_void, sz: usize, misalign_bytes: c_int) -> c_int;
}
