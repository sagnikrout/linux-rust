//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/perf/callchain_64.c
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
// Performance counter callchain support - powerpc architecture code
//
// Copyright © 2009 Paul Mackerras, IBM Corporation.
//

#[no_mangle]
unsafe extern "C" fn read_user_stack_64(ptr: *const unsigned long __user, ret: *mut c_ulong) -> c_int {
    static int read_user_stack_64(const unsigned long __user *ptr, unsigned long *ret)
    {
    return __read_user_stack(ptr, ret, sizeof(*ret));
    }
//
// 64-bit user processes use the same stack frame for RT and non-RT signals.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct signal_frame_64 {
    pub dummy: [c_char; __SIGNAL_FRAMESIZE],
    pub uc: ucontext,
    pub unused: [c_ulong; 2],
    pub tramp: [c_uint; 6],
    pub pinfo: *mut siginfo,
    pub puc: *mut c_void,
    pub info: siginfo,
    pub abigap: [c_char; 288],
}

#[no_mangle]
unsafe extern "C" fn is_sigreturn_64_address(nip: c_ulong, fp: c_ulong) -> c_int {
    static int is_sigreturn_64_address(unsigned long nip, unsigned long fp)
    {
    if (nip == fp + offsetof(struct signal_frame_64, tramp))
    return 1;
    if (current.mm.context.vdso &&
    nip == VDSO64_SYMBOL(current.mm.context.vdso, sigtramp_rt64))
    return 1;
    return 0;
    }
//
// Do some sanity checking on the signal frame pointed to by sp.
// We check the pinfo and puc pointers in the frame.
//
#[no_mangle]
unsafe extern "C" fn sane_signal_64_frame(sp: c_ulong) -> c_int {
    static int sane_signal_64_frame(unsigned long sp)
    {
    struct signal_frame_64 __user *sf;
    unsigned long pinfo, puc;
    sf = (struct signal_frame_64 __user *) sp;
    if (read_user_stack_64((unsigned long __user *) &sf.pinfo, &pinfo) ||
    read_user_stack_64((unsigned long __user *) &sf.puc, &puc))
    return 0;
    return pinfo == (unsigned long) &sf.info &&
    puc == (unsigned long) &sf.uc;
    }
    void perf_callchain_user_64(struct perf_callchain_entry_ctx *entry,
    struct pt_regs *regs)
    {
    unsigned long sp, next_sp;
    unsigned long next_ip;
    unsigned long lr;
    let mut level: c_long = 0;
    struct signal_frame_64 __user *sigframe;
    unsigned long __user *fp, *uregs;
    next_ip = perf_arch_instruction_pointer(regs);
    lr = regs.link;
    sp = regs.gpr[1];
    while (entry.nr < entry.max_stack) {
    fp = (unsigned long __user *) sp;
    if (invalid_user_sp(sp) || read_user_stack_64(fp, &next_sp))
    return;
    if (level > 0 && read_user_stack_64(&fp[2], &next_ip))
    return;
//
// Note: the next_sp - sp >= signal frame size check
// is true when next_sp < sp, which can happen when
// transitioning from an alternate signal stack to the
// normal stack.
//
    if (next_sp - sp >= sizeof(struct signal_frame_64) &&
    (is_sigreturn_64_address(next_ip, sp) ||
    (level <= 1 && is_sigreturn_64_address(lr, sp))) &&
    sane_signal_64_frame(sp)) {
//
// This looks like an signal frame
//
    sigframe = (struct signal_frame_64 __user *) sp;
    uregs = sigframe.uc.uc_mcontext.gp_regs;
    if (read_user_stack_64(&uregs[PT_NIP], &next_ip) ||
    read_user_stack_64(&uregs[PT_LNK], &lr) ||
    read_user_stack_64(&uregs[PT_R1], &sp))
    return;
    level = 0;
    perf_callchain_store_context(entry, PERF_CONTEXT_USER);
    perf_callchain_store(entry, next_ip);
    continue;
    }
    if (level == 0)
    next_ip = lr;
    perf_callchain_store(entry, next_ip);
    ++level;
    sp = next_sp;
    }
    }
