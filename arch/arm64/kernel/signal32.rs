//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/signal32.c
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
// Based on arch/arm/kernel/signal.c
//
// Copyright (C) 1995-2009 Russell King
// Copyright (C) 2012 ARM Ltd.
// Modified by Will Deacon <will.deacon@arm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_vfp_sigframe {
    pub magic: compat_ulong_t,
    pub size: compat_ulong_t,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_user_vfp {
    pub fpregs: [compat_u64; 32],
    pub fpscr: compat_ulong_t,
    pub ufp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_user_vfp_exc {
    pub fpexc: compat_ulong_t,
    pub fpinst: compat_ulong_t,
    pub fpinst2: compat_ulong_t,
    pub ufp_exc: },
    pub __attribute__((__aligned__(8))): },
pub const VFP_MAGIC: c_uint = 0x56465001;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_aux_sigframe {
    pub vfp: compat_vfp_sigframe,
// Something that isn't a valid magic number for any coprocessor.
    pub end_magic: c_ulong,
    pub __attribute__((__aligned__(8))): },
#[no_mangle]
pub unsafe extern "C" fn put_sigset_t(uset: *mut compat_sigset_t __user, set: *mut sigset_t) -> c_int {
    static inline int put_sigset_t(compat_sigset_t __user *uset, sigset_t *set)
    {
    pub cset: compat_sigset_t,
    pub 0xffffffffull: cset.sig[0] = set->sig[0] &,
    pub 32: cset.sig[1] = set->sig[0] >>,
    pub sizeof(*uset)): *mut return copy_to_user(uset, &cset,,
    }
    static inline int get_sigset_t(sigset_t *set,
    const compat_sigset_t __user *uset)
    {
    pub s32: compat_sigset_t,
    if (copy_from_user(&s32, uset, sizeof(*uset)))
    pub -EFAULT: return,
    pub 32): set->sig[0] = s32.sig[0] | (((long)s32.sig[1]) <<,
    pub 0: return,
    }
//
// VFP save/restore code.
//
// We have to be careful with endianness, since the fpsimd context-switch
// code operates on 128-bit (Q) register values whereas the compat ABI
// uses an array of 64-bit (D) registers. Consequently, we need to swap
// the two halves of each Q register when running on a big-endian CPU.
//
    union __fpsimd_vreg {
    pub raw: __uint128_t,
    struct {

    pub hi: u64,
    pub lo: u64,

    pub lo: u64,
    pub hi: u64,

}

    };
#[no_mangle]
unsafe extern "C" fn compat_preserve_vfp_context(frame: *mut compat_vfp_sigframe __user) -> c_int {
    static int compat_preserve_vfp_context(struct compat_vfp_sigframe __user *frame)
    {
    struct user_fpsimd_state const *fpsimd =
    &current.thread.uw.fpsimd_state;
    let mut magic: compat_ulong_t = VFP_MAGIC;
    let mut size: compat_ulong_t = VFP_STORAGE_SIZE;
    compat_ulong_t fpscr, fpexc;
    int i, err = 0;
//
// Save the hardware registers to the fpsimd_state structure.
// Note that this also saves V16-31, which aren't visible
// in AArch32.
//
    fpsimd_save_and_flush_current_state();
// Place structure header on the stack
    __put_user_error(magic, &frame.magic, err);
    __put_user_error(size, &frame.size, err);
//
// Now copy the FP registers. Since the registers are packed,
// we can copy the prefix we want (V0-V15) as it is.
//
    for (i = 0; i < ARRAY_SIZE(frame.ufp.fpregs); i += 2) {
    union __fpsimd_vreg vreg = {
    .raw = fpsimd.vregs[i >> 1],
    };
    __put_user_error(vreg.lo, &frame.ufp.fpregs[i], err);
    __put_user_error(vreg.hi, &frame.ufp.fpregs[i + 1], err);
    }
// Create an AArch32 fpscr from the fpsr and the fpcr.
    fpscr = (fpsimd.fpsr & VFP_FPSCR_STAT_MASK) |
    (fpsimd.fpcr & VFP_FPSCR_CTRL_MASK);
    __put_user_error(fpscr, &frame.ufp.fpscr, err);
//
// The exception register aren't available so we fake up a
// basic FPEXC and zero everything else.
//
    fpexc = (1 << 30);
    __put_user_error(fpexc, &frame.ufp_exc.fpexc, err);
    __put_user_error(0, &frame.ufp_exc.fpinst, err);
    __put_user_error(0, &frame.ufp_exc.fpinst2, err);
    return err ? -EFAULT : 0;
    }
#[no_mangle]
unsafe extern "C" fn compat_restore_vfp_context(frame: *mut compat_vfp_sigframe __user) -> c_int {
    static int compat_restore_vfp_context(struct compat_vfp_sigframe __user *frame)
    {
    struct user_fpsimd_state fpsimd;
    let mut magic: compat_ulong_t = VFP_MAGIC;
    let mut size: compat_ulong_t = VFP_STORAGE_SIZE;
    compat_ulong_t fpscr;
    int i, err = 0;
    __get_user_error(magic, &frame.magic, err);
    __get_user_error(size, &frame.size, err);
    if (err)
    return -EFAULT;
    if (magic != VFP_MAGIC || size != VFP_STORAGE_SIZE)
    return -EINVAL;
// Copy the FP registers into the start of the fpsimd_state.
    for (i = 0; i < ARRAY_SIZE(frame.ufp.fpregs); i += 2) {
    union __fpsimd_vreg vreg;
    __get_user_error(vreg.lo, &frame.ufp.fpregs[i], err);
    __get_user_error(vreg.hi, &frame.ufp.fpregs[i + 1], err);
    fpsimd.vregs[i >> 1] = vreg.raw;
    }
// Extract the fpsr and the fpcr from the fpscr
    __get_user_error(fpscr, &frame.ufp.fpscr, err);
    fpsimd.fpsr = fpscr & VFP_FPSCR_STAT_MASK;
    fpsimd.fpcr = fpscr & VFP_FPSCR_CTRL_MASK;
    if (err)
    return -EFAULT;
//
// We don't need to touch the exception register, so
// reload the hardware state.
//
    fpsimd_save_and_flush_current_state();
    current.thread.uw.fpsimd_state = fpsimd;
    return 0;
    }
    static int compat_restore_sigframe(struct pt_regs *regs,
    struct compat_sigframe __user *sf)
    {
    int err;
    sigset_t set;
    struct compat_aux_sigframe __user *aux;
    unsigned long psr;
    err = get_sigset_t(&set, &sf.uc.uc_sigmask);
    if (err == 0)
    set_current_blocked(&set);
    __get_user_error(regs.regs[0], &sf.uc.uc_mcontext.arm_r0, err);
    __get_user_error(regs.regs[1], &sf.uc.uc_mcontext.arm_r1, err);
    __get_user_error(regs.regs[2], &sf.uc.uc_mcontext.arm_r2, err);
    __get_user_error(regs.regs[3], &sf.uc.uc_mcontext.arm_r3, err);
    __get_user_error(regs.regs[4], &sf.uc.uc_mcontext.arm_r4, err);
    __get_user_error(regs.regs[5], &sf.uc.uc_mcontext.arm_r5, err);
    __get_user_error(regs.regs[6], &sf.uc.uc_mcontext.arm_r6, err);
    __get_user_error(regs.regs[7], &sf.uc.uc_mcontext.arm_r7, err);
    __get_user_error(regs.regs[8], &sf.uc.uc_mcontext.arm_r8, err);
    __get_user_error(regs.regs[9], &sf.uc.uc_mcontext.arm_r9, err);
    __get_user_error(regs.regs[10], &sf.uc.uc_mcontext.arm_r10, err);
    __get_user_error(regs.regs[11], &sf.uc.uc_mcontext.arm_fp, err);
    __get_user_error(regs.regs[12], &sf.uc.uc_mcontext.arm_ip, err);
    __get_user_error(regs.compat_sp, &sf.uc.uc_mcontext.arm_sp, err);
    __get_user_error(regs.compat_lr, &sf.uc.uc_mcontext.arm_lr, err);
    __get_user_error(regs.pc, &sf.uc.uc_mcontext.arm_pc, err);
    __get_user_error(psr, &sf.uc.uc_mcontext.arm_cpsr, err);
    regs.pstate = compat_psr_to_pstate(psr);
//
// Avoid compat_sys_sigreturn() restarting.
//
    forget_syscall(regs);
    err |= !valid_user_regs(&regs.user_regs, current);
    aux = (struct compat_aux_sigframe __user *) sf.uc.uc_regspace;
    if (err == 0 && system_supports_fpsimd())
    err |= compat_restore_vfp_context(&aux.vfp);
    return err;
    }
    COMPAT_SYSCALL_DEFINE0(sigreturn)
    {
    struct pt_regs *regs = current_pt_regs();
    struct compat_sigframe __user *frame;
// Always make any pending restarted system calls return -EINTR
    current.restart_block.fn = do_no_restart_syscall;
//
// Since we stacked the signal on a 64-bit boundary,
// then 'sp' should be word aligned here.  If it's
// not, then the user is trying to mess with us.
//
    if (regs.compat_sp & 7)
    goto badframe;
    frame = (struct compat_sigframe __user *)regs.compat_sp;
    if (!access_ok(frame, sizeof (*frame)))
    goto badframe;
    if (compat_restore_sigframe(regs, frame))
    goto badframe;
    return regs.regs[0];
    badframe:
    arm64_notify_segfault(regs.compat_sp);
    return 0;
    }
    COMPAT_SYSCALL_DEFINE0(rt_sigreturn)
    {
    struct pt_regs *regs = current_pt_regs();
    struct compat_rt_sigframe __user *frame;
// Always make any pending restarted system calls return -EINTR
    current.restart_block.fn = do_no_restart_syscall;
//
// Since we stacked the signal on a 64-bit boundary,
// then 'sp' should be word aligned here.  If it's
// not, then the user is trying to mess with us.
//
    if (regs.compat_sp & 7)
    goto badframe;
    frame = (struct compat_rt_sigframe __user *)regs.compat_sp;
    if (!access_ok(frame, sizeof (*frame)))
    goto badframe;
    if (compat_restore_sigframe(regs, &frame.sig))
    goto badframe;
    if (compat_restore_altstack(&frame.sig.uc.uc_stack))
    goto badframe;
    return regs.regs[0];
    badframe:
    arm64_notify_segfault(regs.compat_sp);
    return 0;
    }
    static void __user *compat_get_sigframe(struct ksignal *ksig,
    struct pt_regs *regs,
    int framesize)
    {
    let mut sp: compat_ulong_t = sigsp(regs.compat_sp, ksig);
    void __user *frame;
//
// ATPCS B01 mandates 8-byte alignment
//
    frame = compat_ptr((compat_uptr_t)((sp - framesize) & ~7));
//
// Check that we can actually write to the signal frame.
//
    if (!access_ok(frame, framesize))
    frame = core::ptr::null_mut();
    return frame;
    }
    static void compat_setup_return(struct pt_regs *regs, struct k_sigaction *ka,
    compat_ulong_t __user *rc, void __user *frame,
    int usig)
    {
    let mut handler: compat_ulong_t = ptr_to_compat(ka.sa.sa_handler);
    compat_ulong_t retcode;
    let mut spsr: compat_ulong_t = regs.pstate & ~(PSR_f | PSR_AA32_E_BIT);
    int thumb;
// Check if the handler is written for ARM or Thumb
    thumb = handler & 1;
    if (thumb)
    spsr |= PSR_AA32_T_BIT;
    else
    spsr &= ~PSR_AA32_T_BIT;
// The IT state must be cleared for both ARM and Thumb-2
    spsr &= ~PSR_AA32_IT_MASK;
// Restore the original endianness
    spsr |= PSR_AA32_ENDSTATE;
    if (ka.sa.sa_flags & SA_RESTORER) {
    retcode = ptr_to_compat(ka.sa.sa_restorer);
    } else {
// Set up sigreturn pointer
    let mut idx: c_uint = thumb << 1;
    if (ka.sa.sa_flags & SA_SIGINFO)
    idx += 3;
    retcode = (unsigned long)current.mm.context.sigpage +
    (idx << 2) + thumb;
    }
    regs.regs[0]	= usig;
    regs.compat_sp	= ptr_to_compat(frame);
    regs.compat_lr	= retcode;
    regs.pc	= handler;
    regs.pstate	= spsr;
    }
    static int compat_setup_sigframe(struct compat_sigframe __user *sf,
    struct pt_regs *regs, sigset_t *set)
    {
    struct compat_aux_sigframe __user *aux;
    let mut psr: c_ulong = pstate_to_compat_psr(regs.pstate);
    let mut err: c_int = 0;
    __put_user_error(regs.regs[0], &sf.uc.uc_mcontext.arm_r0, err);
    __put_user_error(regs.regs[1], &sf.uc.uc_mcontext.arm_r1, err);
    __put_user_error(regs.regs[2], &sf.uc.uc_mcontext.arm_r2, err);
    __put_user_error(regs.regs[3], &sf.uc.uc_mcontext.arm_r3, err);
    __put_user_error(regs.regs[4], &sf.uc.uc_mcontext.arm_r4, err);
    __put_user_error(regs.regs[5], &sf.uc.uc_mcontext.arm_r5, err);
    __put_user_error(regs.regs[6], &sf.uc.uc_mcontext.arm_r6, err);
    __put_user_error(regs.regs[7], &sf.uc.uc_mcontext.arm_r7, err);
    __put_user_error(regs.regs[8], &sf.uc.uc_mcontext.arm_r8, err);
    __put_user_error(regs.regs[9], &sf.uc.uc_mcontext.arm_r9, err);
    __put_user_error(regs.regs[10], &sf.uc.uc_mcontext.arm_r10, err);
    __put_user_error(regs.regs[11], &sf.uc.uc_mcontext.arm_fp, err);
    __put_user_error(regs.regs[12], &sf.uc.uc_mcontext.arm_ip, err);
    __put_user_error(regs.compat_sp, &sf.uc.uc_mcontext.arm_sp, err);
    __put_user_error(regs.compat_lr, &sf.uc.uc_mcontext.arm_lr, err);
    __put_user_error(regs.pc, &sf.uc.uc_mcontext.arm_pc, err);
    __put_user_error(psr, &sf.uc.uc_mcontext.arm_cpsr, err);
    __put_user_error((compat_ulong_t)0, &sf.uc.uc_mcontext.trap_no, err);
// set the compat FSR WnR
    __put_user_error(!!(current.thread.fault_code & ESR_ELx_WNR) <<
    FSR_WRITE_SHIFT, &sf.uc.uc_mcontext.error_code, err);
    __put_user_error(current.thread.fault_address, &sf.uc.uc_mcontext.fault_address, err);
    __put_user_error(set.sig[0], &sf.uc.uc_mcontext.oldmask, err);
    err |= put_sigset_t(&sf.uc.uc_sigmask, set);
    aux = (struct compat_aux_sigframe __user *) sf.uc.uc_regspace;
    if (err == 0 && system_supports_fpsimd())
    err |= compat_preserve_vfp_context(&aux.vfp);
    __put_user_error(0, &aux.end_magic, err);
    return err;
    }
//
// 32-bit signal handling routines called from signal.c
//
    int compat_setup_rt_frame(int usig, struct ksignal *ksig,
    sigset_t *set, struct pt_regs *regs)
    {
    struct compat_rt_sigframe __user *frame;
    let mut err: c_int = 0;
    frame = compat_get_sigframe(ksig, regs, sizeof(*frame));
    if (!frame)
    return 1;
    err |= copy_siginfo_to_user32(&frame.info, &ksig.info);
    __put_user_error(0, &frame.sig.uc.uc_flags, err);
    __put_user_error(0, &frame.sig.uc.uc_link, err);
    err |= __compat_save_altstack(&frame.sig.uc.uc_stack, regs.compat_sp);
    err |= compat_setup_sigframe(&frame.sig, regs, set);
    if (err == 0) {
    compat_setup_return(regs, &ksig.ka, frame.sig.retcode, frame, usig);
    regs.regs[1] = (compat_ulong_t)(unsigned long)&frame.info;
    regs.regs[2] = (compat_ulong_t)(unsigned long)&frame.sig.uc;
    }
    return err;
    }
    int compat_setup_frame(int usig, struct ksignal *ksig, sigset_t *set,
    struct pt_regs *regs)
    {
    struct compat_sigframe __user *frame;
    let mut err: c_int = 0;
    frame = compat_get_sigframe(ksig, regs, sizeof(*frame));
    if (!frame)
    return 1;
    __put_user_error(0x5ac3c35a, &frame.uc.uc_flags, err);
    err |= compat_setup_sigframe(frame, regs, set);
    if (err == 0)
    compat_setup_return(regs, &ksig.ka, frame.retcode, frame, usig);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn compat_setup_restart_syscall(regs: *mut pt_regs) {
    void compat_setup_restart_syscall(struct pt_regs *regs)
    {
    regs.regs[7] = __NR_compat32_restart_syscall;
    }
//
// Compile-time assertions for siginfo_t offsets. Check NSIG* as well, as
// changes likely come with new fields that should be added below.
//
    static_assert(NSIGILL	== 11);
    static_assert(NSIGFPE	== 15);
    static_assert(NSIGSEGV	== 10);
    static_assert(NSIGBUS	== 5);
    static_assert(NSIGTRAP	== 6);
    static_assert(NSIGCHLD	== 6);
    static_assert(NSIGSYS	== 2);
    static_assert(sizeof(compat_siginfo_t) == 128);
    static_assert(__alignof__(compat_siginfo_t) == 4);
    static_assert(offsetof(compat_siginfo_t, si_signo)	== 0x00);
    static_assert(offsetof(compat_siginfo_t, si_errno)	== 0x04);
    static_assert(offsetof(compat_siginfo_t, si_code)	== 0x08);
    static_assert(offsetof(compat_siginfo_t, si_pid)	== 0x0c);
    static_assert(offsetof(compat_siginfo_t, si_uid)	== 0x10);
    static_assert(offsetof(compat_siginfo_t, si_tid)	== 0x0c);
    static_assert(offsetof(compat_siginfo_t, si_overrun)	== 0x10);
    static_assert(offsetof(compat_siginfo_t, si_status)	== 0x14);
    static_assert(offsetof(compat_siginfo_t, si_utime)	== 0x18);
    static_assert(offsetof(compat_siginfo_t, si_stime)	== 0x1c);
    static_assert(offsetof(compat_siginfo_t, si_value)	== 0x14);
    static_assert(offsetof(compat_siginfo_t, si_int)	== 0x14);
    static_assert(offsetof(compat_siginfo_t, si_ptr)	== 0x14);
    static_assert(offsetof(compat_siginfo_t, si_addr)	== 0x0c);
    static_assert(offsetof(compat_siginfo_t, si_addr_lsb)	== 0x10);
    static_assert(offsetof(compat_siginfo_t, si_lower)	== 0x14);
    static_assert(offsetof(compat_siginfo_t, si_upper)	== 0x18);
    static_assert(offsetof(compat_siginfo_t, si_pkey)	== 0x14);
    static_assert(offsetof(compat_siginfo_t, si_perf_data)	== 0x10);
    static_assert(offsetof(compat_siginfo_t, si_perf_type)	== 0x14);
    static_assert(offsetof(compat_siginfo_t, si_perf_flags)	== 0x18);
    static_assert(offsetof(compat_siginfo_t, si_band)	== 0x0c);
    static_assert(offsetof(compat_siginfo_t, si_fd)		== 0x10);
    static_assert(offsetof(compat_siginfo_t, si_call_addr)	== 0x0c);
    static_assert(offsetof(compat_siginfo_t, si_syscall)	== 0x10);
    static_assert(offsetof(compat_siginfo_t, si_arch)	== 0x14);
