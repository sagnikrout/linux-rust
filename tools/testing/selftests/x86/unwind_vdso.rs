//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/unwind_vdso.c
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
// unwind_vdso.c - tests unwind info for AT_SYSINFO in the vDSO
// Copyright (c) 2014-2015 Andrew Lutomirski
//
// This tests __kernel_vsyscall's unwind info.
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main()
    {
// We need getauxval().
    printf("[SKIP]\tGLIBC before 2.16 cannot compile this test\n");
    return 0;
    }

    static volatile sig_atomic_t nerrs;
    static unsigned long sysinfo;
    let mut got_sysinfo: static bool = false;
    static unsigned long return_address;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_state {
    pub /: *mut *mut unsigned long ip; / trap source,
    pub /: *mut *mut int depth; / -1 until we hit the trap source,
}

#[no_mangle]
pub unsafe extern "C" fn trace_fn(ctx: *mut *mut _Unwind_Context, opaque: *mut c_void) -> _Unwind_Reason_Code {
    _Unwind_Reason_Code trace_fn(struct _Unwind_Context * ctx, void *opaque)
    {
    struct unwind_state *state = opaque;
    let mut ip: c_ulong = _Unwind_GetIP(ctx);
    if (state.depth == -1) {
    if (ip == state.ip)
    state.depth = 0;
    else
    return _URC_NO_REASON;	/* Not there yet */
    }
    printf("\t  0x%lx\n", ip);
    if (ip == return_address) {
// Here we are.
    let mut eax: c_ulong = _Unwind_GetGR(ctx, 0);
    let mut ecx: c_ulong = _Unwind_GetGR(ctx, 1);
    let mut edx: c_ulong = _Unwind_GetGR(ctx, 2);
    let mut ebx: c_ulong = _Unwind_GetGR(ctx, 3);
    let mut ebp: c_ulong = _Unwind_GetGR(ctx, 5);
    let mut esi: c_ulong = _Unwind_GetGR(ctx, 6);
    let mut edi: c_ulong = _Unwind_GetGR(ctx, 7);
    bool ok = (eax == SYS_getpid || eax == getpid()) &&
    ebx == 1 && ecx == 2 && edx == 3 &&
    esi == 4 && edi == 5 && ebp == 6;
    if (!ok)
    nerrs++;
    printf("[%s]\t  NR = %ld, args = %ld, %ld, %ld, %ld, %ld, %ld\n",
    (ok ? "OK" : "FAIL"),
    eax, ebx, ecx, edx, esi, edi, ebp);
    return _URC_NORMAL_STOP;
    } else {
    state.depth++;
    return _URC_NO_REASON;
    }
    }
#[no_mangle]
unsafe extern "C" fn sigtrap(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigtrap(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t *)ctx_void;
    struct unwind_state state;
    let mut ip: c_ulong = ctx.uc_mcontext.gregs[REG_EIP];
    if (!got_sysinfo && ip == sysinfo) {
    got_sysinfo = true;
// Find the return address.
    return_address = *(unsigned long *)(unsigned long)ctx.uc_mcontext.gregs[REG_ESP];
    printf("\tIn vsyscall at 0x%lx, returning to 0x%lx\n",
    ip, return_address);
    }
    if (!got_sysinfo)
    return;		/* Not there yet */
    if (ip == return_address) {
    ctx.uc_mcontext.gregs[REG_EFL] &= ~X86_EFLAGS_TF;
    printf("\tVsyscall is done\n");
    return;
    }
    printf("\tSIGTRAP at 0x%lx\n", ip);
    state.ip = ip;
    state.depth = -1;
    _Unwind_Backtrace(trace_fn, &state);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main()
    {
    sysinfo = getauxval(AT_SYSINFO);
    printf("\tAT_SYSINFO is 0x%lx\n", sysinfo);
    Dl_info info;
    if (!dladdr((void *)sysinfo, &info)) {
    printf("[WARN]\tdladdr failed on AT_SYSINFO\n");
    } else {
    printf("[OK]\tAT_SYSINFO maps to %s, loaded at 0x%p\n",
    info.dli_fname, info.dli_fbase);
    }
    sethandler(SIGTRAP, sigtrap, 0);
    syscall(SYS_getpid);  /* Force symbol binding without TF set. */
    printf("[RUN]\tSet TF and check a fast syscall\n");
    set_eflags(get_eflags() | X86_EFLAGS_TF);
    syscall(SYS_getpid, 1, 2, 3, 4, 5, 6);
    if (!got_sysinfo) {
    set_eflags(get_eflags() & ~X86_EFLAGS_TF);
//
// The most likely cause of this is that you're on Debian or
// a Debian-based distro, you're missing libc6-i686, and you're
// affected by libc/19006 (https://sourceware.org/PR19006).
//
    printf("[WARN]\tsyscall(2) didn't enter AT_SYSINFO\n");
    }
    if (get_eflags() & X86_EFLAGS_TF) {
    printf("[FAIL]\tTF is still set\n");
    nerrs++;
    }
    if (nerrs) {
    printf("[FAIL]\tThere were errors\n");
    return 1;
    } else {
    printf("[OK]\tAll is well\n");
    return 0;
    }
    }
