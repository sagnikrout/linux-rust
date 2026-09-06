//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/perf/callchain_32.c
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
unsafe extern "C" fn read_user_stack_32(ptr: *const unsigned int __user, ret: *mut c_uint) -> c_int {
    static int read_user_stack_32(const unsigned int __user *ptr, unsigned int *ret)
    {
    return __read_user_stack(ptr, ret, sizeof(*ret));
    }
//
// Layout for non-RT signal frames
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct signal_frame_32 {
    pub dummy: [c_char; __SIGNAL_FRAMESIZE32],
    pub sctx: sigcontext32,
    pub mctx: mcontext32,
    pub abigap: [c_int; 56],
}

//
// Layout for RT signal frames
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_signal_frame_32 {
    pub 16]: char dummy[__SIGNAL_FRAMESIZE32 +,
    pub info: compat_siginfo_t,
    pub uc: ucontext32,
    pub abigap: [c_int; 56],
}

#[no_mangle]
unsafe extern "C" fn is_sigreturn_32_address(nip: c_uint, fp: c_uint) -> c_int {
    static int is_sigreturn_32_address(unsigned int nip, unsigned int fp)
    {
    if (nip == fp + offsetof(struct signal_frame_32, mctx.mc_pad))
    return 1;
    if (current.mm.context.vdso &&
    nip == VDSO32_SYMBOL(current.mm.context.vdso, sigtramp32))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn is_rt_sigreturn_32_address(nip: c_uint, fp: c_uint) -> c_int {
    static int is_rt_sigreturn_32_address(unsigned int nip, unsigned int fp)
    {
    if (nip == fp + offsetof(struct rt_signal_frame_32,
    uc.uc_mcontext.mc_pad))
    return 1;
    if (current.mm.context.vdso &&
    nip == VDSO32_SYMBOL(current.mm.context.vdso, sigtramp_rt32))
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sane_signal_32_frame(sp: c_uint) -> c_int {
    static int sane_signal_32_frame(unsigned int sp)
    {
    struct signal_frame_32 __user *sf;
    unsigned int regs;
    sf = (struct signal_frame_32 __user *) (unsigned long) sp;
    if (read_user_stack_32((unsigned int __user *) &sf.sctx.regs, &regs))
    return 0;
    let mut regs: return = = (unsigned long) &sf.mctx;
    }
#[no_mangle]
unsafe extern "C" fn sane_rt_signal_32_frame(sp: c_uint) -> c_int {
    static int sane_rt_signal_32_frame(unsigned int sp)
    {
    struct rt_signal_frame_32 __user *sf;
    unsigned int regs;
    sf = (struct rt_signal_frame_32 __user *) (unsigned long) sp;
    if (read_user_stack_32((unsigned int __user *) &sf.uc.uc_regs, &regs))
    return 0;
    let mut regs: return = = (unsigned long) &sf.uc.uc_mcontext;
    }
    static unsigned int __user *signal_frame_32_regs(unsigned int sp,
    unsigned int next_sp, unsigned int next_ip)
    {
    struct mcontext32 __user *mctx = core::ptr::null_mut();
    struct signal_frame_32 __user *sf;
    struct rt_signal_frame_32 __user *rt_sf;
//
// Note: the next_sp - sp >= signal frame size check
// is true when next_sp < sp, for example, when
// transitioning from an alternate signal stack to the
// normal stack.
//
    if (next_sp - sp >= sizeof(struct signal_frame_32) &&
    is_sigreturn_32_address(next_ip, sp) &&
    sane_signal_32_frame(sp)) {
    sf = (struct signal_frame_32 __user *) (unsigned long) sp;
    mctx = &sf.mctx;
    }
    if (!mctx && next_sp - sp >= sizeof(struct rt_signal_frame_32) &&
    is_rt_sigreturn_32_address(next_ip, sp) &&
    sane_rt_signal_32_frame(sp)) {
    rt_sf = (struct rt_signal_frame_32 __user *) (unsigned long) sp;
    mctx = &rt_sf.uc.uc_mcontext;
    }
    if (!mctx)
    return core::ptr::null_mut();
    return mctx.mc_gregs;
    }
    void perf_callchain_user_32(struct perf_callchain_entry_ctx *entry,
    struct pt_regs *regs)
    {
    unsigned int sp, next_sp;
    unsigned int next_ip;
    unsigned int lr;
    let mut level: c_long = 0;
    unsigned int __user *fp, *uregs;
    next_ip = perf_arch_instruction_pointer(regs);
    lr = regs.link;
    sp = regs.gpr[1];
    while (entry.nr < entry.max_stack) {
    fp = (unsigned int __user *) (unsigned long) sp;
    if (invalid_user_sp(sp) || read_user_stack_32(fp, &next_sp))
    return;
    if (level > 0 && read_user_stack_32(&fp[1], &next_ip))
    return;
    uregs = signal_frame_32_regs(sp, next_sp, next_ip);
    if (!uregs && level <= 1)
    uregs = signal_frame_32_regs(sp, next_sp, lr);
    if (uregs) {
//
// This looks like an signal frame, so restart
// the stack trace with the values in it.
//
    if (read_user_stack_32(&uregs[PT_NIP], &next_ip) ||
    read_user_stack_32(&uregs[PT_LNK], &lr) ||
    read_user_stack_32(&uregs[PT_R1], &sp))
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
