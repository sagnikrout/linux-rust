//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/signal_32.c
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
// Signal handling for 32bit PPC and 32bit tasks on 64bit PPC
//
// PowerPC version
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
// Copyright (C) 2001 IBM
// Copyright (C) 1997,1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
// Copyright (C) 1997 David S. Miller (davem@caip.rutgers.edu)
//
// Derived from "arch/i386/kernel/signal.c"
// Copyright (C) 1991, 1992 Linus Torvalds
// 1997-11-28  Modified for POSIX.1b signals by Richard Henderson
//

//
// Userspace code may pass a ucontext which doesn't include VSX added
// at the end.  We need to check for this case.
//

    (sizeof(struct ucontext) - sizeof(elf_vsrreghalf_t32))
//
// Returning 0 means we return to userspace via
// ret_from_except and thus restore all user
// registers from *regs.  This is what we need
// to do when a signal has been delivered.
//

//
// Functions for flipping sigsets (thanks to brain dead generic
// implementation that makes things simple for little endian only)
//

    static __always_inline int
    __unsafe_save_general_regs(struct pt_regs *regs, struct mcontext __user *frame)
    {
    elf_greg_t64 *gregs = (elf_greg_t64 *)regs;
    int val, i;
    for (i = 0; i <= PT_RESULT; i ++) {
// Force usr to alway see softe as 1 (interrupts enabled)
    if (i == PT_SOFTE)
    val = 1;
    else
    val = gregs[i];
    unsafe_put_user(val, &frame.mc_gregs[i], failed);
    }
    return 0;
    failed:
    return 1;
    }
    static __always_inline int
    __unsafe_restore_general_regs(struct pt_regs *regs, struct mcontext __user *sr)
    {
    elf_greg_t64 *gregs = (elf_greg_t64 *)regs;
    int i;
    for (i = 0; i <= PT_RESULT; i++) {
    if ((i == PT_MSR) || (i == PT_SOFTE))
    continue;
    unsafe_get_user(gregs[i], &sr.mc_gregs[i], failed);
    }
    return 0;
    failed:
    return 1;
    }

    sigset_t __user *__us = uset	;				\
    const sigset_t *__s = set;					\
    \
    unsafe_copy_to_user(__us, __s, sizeof(*__us), label);		\
    } while (0)

    static __always_inline int
    __unsafe_save_general_regs(struct pt_regs *regs, struct mcontext __user *frame)
    {
    unsafe_copy_to_user(&frame.mc_gregs, regs, GP_REGS_SIZE, failed);
    return 0;
    failed:
    return 1;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn __unsafe_restore_general_regs(regs: *mut pt_regs, sr: *mut mcontext __user) -> c_int {
    int __unsafe_restore_general_regs(struct pt_regs *regs, struct mcontext __user *sr)
    {
// copy up to but not including MSR
    unsafe_copy_from_user(regs, &sr.mc_gregs, PT_MSR * sizeof(elf_greg_t), failed);
// copy from orig_r3 (the word after the MSR) up to the end
    unsafe_copy_from_user(&regs.orig_gpr3, &sr.mc_gregs[PT_ORIG_R3],
    GP_REGS_SIZE - PT_ORIG_R3 * sizeof(elf_greg_t), failed);
    return 0;
    failed:
    return 1;
    }

    if (__unsafe_save_general_regs(regs, frame))		\
    goto label;					\
    } while (0)

    if (__unsafe_restore_general_regs(regs, frame))		\
    goto label;					\
    } while (0)
//
// When we have signals to deliver, we set up on the
// user stack, going down from the original stack pointer:
// an ABI gap of 56 words
// an mcontext struct
// a sigcontext struct
// a gap of __SIGNAL_FRAMESIZE bytes
//
// Each of these things must be a multiple of 16 bytes in size. The following
// structure represent all of this except the __SIGNAL_FRAMESIZE gap
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigframe {
    pub /: *mut *mut sigcontext sctx; / the sigcontext,
    pub /: *mut *mut mcontext mctx; / all the register values,

    pub sctx_transact: sigcontext,
    pub mctx_transact: mcontext,

//
// Programs using the rs6000/xcoff abi can save up to 19 gp
// regs and 18 fp regs below sp before decrementing it.
//
    pub abigap: [c_int; 56],
}

//
// When we have rt signals to deliver, we set up on the
// user stack, going down from the original stack pointer:
// one rt_sigframe struct (siginfo + ucontext + ABI gap)
// a gap of __SIGNAL_FRAMESIZE+16 bytes
// (the +16 is to get the siginfo and ucontext in the same
// positions as in older kernels).
//
// Each of these things must be a multiple of 16 bytes in size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_sigframe {

    pub info: compat_siginfo_t,

    pub info: siginfo,

    pub uc: ucontext,

    pub uc_transact: ucontext,

//
// Programs using the rs6000/xcoff abi can save up to 19 gp
// regs and 18 fp regs below sp before decrementing it.
//
    pub abigap: [c_int; 56],
}

#[no_mangle]
pub unsafe extern "C" fn get_min_sigframe_size_32() -> c_ulong {
    unsigned long get_min_sigframe_size_32(void)
    {
    return max(sizeof(struct rt_sigframe) + __SIGNAL_FRAMESIZE + 16,
    sizeof(struct sigframe) + __SIGNAL_FRAMESIZE);
    }
//
// Save the current user registers on the user stack.
// We only save the altivec/spe registers if the process has used
// altivec/spe instructions at some point.
//
#[no_mangle]
unsafe extern "C" fn prepare_save_user_regs(ctx_has_vsx_region: c_int) {
    static void prepare_save_user_regs(int ctx_has_vsx_region)
    {
// Make sure floating point registers are stored in regs
    flush_fp_to_thread(current);

    if (current.thread.used_vr)
    flush_altivec_to_thread(current);
    if (cpu_has_feature(CPU_FTR_ALTIVEC))
    current.thread.vrsave = mfspr(SPRN_VRSAVE);

    if (current.thread.used_vsr && ctx_has_vsx_region)
    flush_vsx_to_thread(current);

    if (current.thread.used_spe)
    flush_spe_to_thread(current);

    }
    static __always_inline int
    __unsafe_save_user_regs(struct pt_regs *regs, struct mcontext __user *frame,
    struct mcontext __user *tm_frame, int ctx_has_vsx_region)
    {
    let mut msr: c_ulong = regs.msr;
// save general registers
    unsafe_save_general_regs(regs, frame, failed);

// save altivec registers
    if (current.thread.used_vr) {
    unsafe_copy_to_user(&frame.mc_vregs, &current.thread.vr_state,
    ELF_NVRREG * sizeof(vector128), failed);
// set MSR_VEC in the saved MSR value to indicate that
    frame.mc_vregs contains valid data */
    msr |= MSR_VEC;
    }
// else assert((regs->msr & MSR_VEC) == 0)
// We always copy to/from vrsave, it's 0 if we don't have or don't
// use altivec. Since VSCR only contains 32 bits saved in the least
// significant bits of a vector, we "cheat" and stuff VRSAVE in the
// most significant bits of that same vector. --BenH
// Note that the current VRSAVE value is in the SPR at this point.
//
    unsafe_put_user(current.thread.vrsave, (u32 __user *)&frame.mc_vregs[32],
    failed);

    unsafe_copy_fpr_to_user(&frame.mc_fregs, current, failed);
//
// Clear the MSR VSX bit to indicate there is no valid state attached
// to this context, except in the specific case below where we set it.
//
    msr &= ~MSR_VSX;

//
// Copy VSR 0-31 upper half from thread_struct to local
// buffer, then write that to userspace.  Also set MSR_VSX in
// the saved MSR value to indicate that frame->mc_vregs
// contains valid data
//
    if (current.thread.used_vsr && ctx_has_vsx_region) {
    unsafe_copy_vsx_to_user(&frame.mc_vsregs, current, failed);
    msr |= MSR_VSX;
    }

// save spe registers
    if (current.thread.used_spe) {
    unsafe_copy_to_user(&frame.mc_vregs, current.thread.evr,
    ELF_NEVRREG * sizeof(u32), failed);
// set MSR_SPE in the saved MSR value to indicate that
    frame.mc_vregs contains valid data */
    msr |= MSR_SPE;
    }
// else assert((regs->msr & MSR_SPE) == 0)
// We always copy to/from spefscr
    unsafe_put_user(current.thread.spefscr,
    (u32 __user *)&frame.mc_vregs + ELF_NEVRREG, failed);

    unsafe_put_user(msr, &frame.mc_gregs[PT_MSR], failed);
// We need to write 0 the MSR top 32 bits in the tm frame so that we
// can check it on the restore to see if TM is active
//
    if (tm_frame)
    unsafe_put_user(0, &tm_frame.mc_gregs[PT_MSR], failed);
    return 0;
    failed:
    return 1;
    }

    if (__unsafe_save_user_regs(regs, frame, tm_frame, has_vsx))	\
    goto label;						\
    } while (0)

//
// Save the current user registers on the user stack.
// We only save the altivec/spe registers if the process has used
// altivec/spe instructions at some point.
// We also save the transactional registers to a second ucontext in the
// frame.
//
// See __unsafe_save_user_regs() and signal_64.c:setup_tm_sigcontexts().
//
#[no_mangle]
unsafe extern "C" fn prepare_save_tm_user_regs() {
    static void prepare_save_tm_user_regs(void)
    {
    WARN_ON(tm_suspend_disabled);
    if (cpu_has_feature(CPU_FTR_ALTIVEC))
    current.thread.ckvrsave = mfspr(SPRN_VRSAVE);
    }
    static __always_inline int
    save_tm_user_regs_unsafe(struct pt_regs *regs, struct mcontext __user *frame,
    struct mcontext __user *tm_frame, unsigned long msr)
    {
// Save both sets of general registers
    unsafe_save_general_regs(&current.thread.ckpt_regs, frame, failed);
    unsafe_save_general_regs(regs, tm_frame, failed);
// Stash the top half of the 64bit MSR into the 32bit MSR word
// of the transactional mcontext.  This way we have a backward-compatible
// MSR in the 'normal' (checkpointed) mcontext and additionally one can
// also look at what type of transaction (T or S) was active at the
// time of the signal.
//
    unsafe_put_user((msr >> 32), &tm_frame.mc_gregs[PT_MSR], failed);
// save altivec registers
    if (current.thread.used_vr) {
    unsafe_copy_to_user(&frame.mc_vregs, &current.thread.ckvr_state,
    ELF_NVRREG * sizeof(vector128), failed);
    if (msr & MSR_VEC)
    unsafe_copy_to_user(&tm_frame.mc_vregs,
    &current.thread.vr_state,
    ELF_NVRREG * sizeof(vector128), failed);
    else
    unsafe_copy_to_user(&tm_frame.mc_vregs,
    &current.thread.ckvr_state,
    ELF_NVRREG * sizeof(vector128), failed);
// set MSR_VEC in the saved MSR value to indicate that
// frame->mc_vregs contains valid data
//
    msr |= MSR_VEC;
    }
// We always copy to/from vrsave, it's 0 if we don't have or don't
// use altivec. Since VSCR only contains 32 bits saved in the least
// significant bits of a vector, we "cheat" and stuff VRSAVE in the
// most significant bits of that same vector. --BenH
//
    unsafe_put_user(current.thread.ckvrsave,
    (u32 __user *)&frame.mc_vregs[32], failed);
    if (msr & MSR_VEC)
    unsafe_put_user(current.thread.vrsave,
    (u32 __user *)&tm_frame.mc_vregs[32], failed);
    else
    unsafe_put_user(current.thread.ckvrsave,
    (u32 __user *)&tm_frame.mc_vregs[32], failed);
    unsafe_copy_ckfpr_to_user(&frame.mc_fregs, current, failed);
    if (msr & MSR_FP)
    unsafe_copy_fpr_to_user(&tm_frame.mc_fregs, current, failed);
    else
    unsafe_copy_ckfpr_to_user(&tm_frame.mc_fregs, current, failed);
//
// Copy VSR 0-31 upper half from thread_struct to local
// buffer, then write that to userspace.  Also set MSR_VSX in
// the saved MSR value to indicate that frame->mc_vregs
// contains valid data
//
    if (current.thread.used_vsr) {
    unsafe_copy_ckvsx_to_user(&frame.mc_vsregs, current, failed);
    if (msr & MSR_VSX)
    unsafe_copy_vsx_to_user(&tm_frame.mc_vsregs, current, failed);
    else
    unsafe_copy_ckvsx_to_user(&tm_frame.mc_vsregs, current, failed);
    msr |= MSR_VSX;
    }
    unsafe_put_user(msr, &frame.mc_gregs[PT_MSR], failed);
    return 0;
    failed:
    return 1;
    }

    static void prepare_save_tm_user_regs(void) { }
    static __always_inline int
    save_tm_user_regs_unsafe(struct pt_regs *regs, struct mcontext __user *frame,
    struct mcontext __user *tm_frame, unsigned long msr)
    {
    return 0;
    }

    if (save_tm_user_regs_unsafe(regs, frame, tm_frame, msr))	\
    goto label;						\
    } while (0)
//
// Restore the current user register values from the user stack,
// (except for MSR).
//
    static long restore_user_regs(struct pt_regs *regs,
    struct mcontext __user *sr, int sig)
    {
    let mut save_r2: c_uint = 0;
    unsigned long msr;

    int i;

    if (!user_read_access_begin(sr, sizeof(*sr)))
    return 1;
//
// restore general registers but not including MSR or SOFTE. Also
// take care of keeping r2 (TLS) intact if not a signal
//
    if (!sig)
    save_r2 = (unsigned int)regs.gpr[2];
    unsafe_restore_general_regs(regs, sr, failed);
    set_trap_norestart(regs);
    unsafe_get_user(msr, &sr.mc_gregs[PT_MSR], failed);
    if (!sig)
    regs.gpr[2] = (unsigned long) save_r2;
// if doing signal return, restore the previous little-endian mode
    if (sig)
    regs_set_return_msr(regs, (regs.msr & ~MSR_LE) | (msr & MSR_LE));

//
// Force the process to reload the altivec registers from
// current->thread when it next does altivec instructions
//
    regs_set_return_msr(regs, regs.msr & ~MSR_VEC);
    if (msr & MSR_VEC) {
// restore altivec registers from the stack
    unsafe_copy_from_user(&current.thread.vr_state, &sr.mc_vregs,
    sizeof(sr.mc_vregs), failed);
    current.thread.used_vr = true;
    } else if (current.thread.used_vr)
    memset(&current.thread.vr_state, 0,
    ELF_NVRREG * sizeof(vector128));
// Always get VRSAVE back
    unsafe_get_user(current.thread.vrsave, (u32 __user *)&sr.mc_vregs[32], failed);
    if (cpu_has_feature(CPU_FTR_ALTIVEC))
    mtspr(SPRN_VRSAVE, current.thread.vrsave);

    unsafe_copy_fpr_from_user(current, &sr.mc_fregs, failed);

//
// Force the process to reload the VSX registers from
// current->thread when it next does VSX instruction.
//
    regs_set_return_msr(regs, regs.msr & ~MSR_VSX);
    if (msr & MSR_VSX) {
//
// Restore altivec registers from the stack to a local
// buffer, then write this out to the thread_struct
//
    unsafe_copy_vsx_from_user(current, &sr.mc_vsregs, failed);
    current.thread.used_vsr = true;
    } else if (current.thread.used_vsr)
    for (i = 0; i < 32 ; i++)
    current.thread.fp_state.fpr[i][TS_VSRLOWOFFSET] = 0;

//
// force the process to reload the FP registers from
// current->thread when it next does FP instructions
//
    regs_set_return_msr(regs, regs.msr & ~(MSR_FP | MSR_FE0 | MSR_FE1));

//
// Force the process to reload the spe registers from
// current->thread when it next does spe instructions.
// Since this is user ABI, we must enforce the sizing.
//
    BUILD_BUG_ON(sizeof(current.thread.spe) != ELF_NEVRREG * sizeof(u32));
    regs_set_return_msr(regs, regs.msr & ~MSR_SPE);
    if (msr & MSR_SPE) {
// restore spe registers from the stack
    unsafe_copy_from_user(&current.thread.spe, &sr.mc_vregs,
    sizeof(current.thread.spe), failed);
    current.thread.used_spe = true;
    } else if (current.thread.used_spe)
    memset(&current.thread.spe, 0, sizeof(current.thread.spe));
// Always get SPEFSCR back
    unsafe_get_user(current.thread.spefscr, (u32 __user *)&sr.mc_vregs + ELF_NEVRREG, failed);

    user_read_access_end();
    return 0;
    failed:
    user_read_access_end();
    return 1;
    }

//
// Restore the current user register values from the user stack, except for
// MSR, and recheckpoint the original checkpointed register state for processes
// in transactions.
//
    static long restore_tm_user_regs(struct pt_regs *regs,
    struct mcontext __user *sr,
    struct mcontext __user *tm_sr)
    {
    unsigned long msr, msr_hi;
    int i;
    if (tm_suspend_disabled)
    return 1;
//
// restore general registers but not including MSR or SOFTE. Also
// take care of keeping r2 (TLS) intact if not a signal.
// See comment in signal_64.c:restore_tm_sigcontexts();
// TFHAR is restored from the checkpointed NIP; TEXASR and TFIAR
// were set by the signal delivery.
//
    if (!user_read_access_begin(sr, sizeof(*sr)))
    return 1;
    unsafe_restore_general_regs(&current.thread.ckpt_regs, sr, failed);
    unsafe_get_user(current.thread.tm_tfhar, &sr.mc_gregs[PT_NIP], failed);
    unsafe_get_user(msr, &sr.mc_gregs[PT_MSR], failed);
// Restore the previous little-endian mode
    regs_set_return_msr(regs, (regs.msr & ~MSR_LE) | (msr & MSR_LE));
    regs_set_return_msr(regs, regs.msr & ~MSR_VEC);
    if (msr & MSR_VEC) {
// restore altivec registers from the stack
    unsafe_copy_from_user(&current.thread.ckvr_state, &sr.mc_vregs,
    sizeof(sr.mc_vregs), failed);
    current.thread.used_vr = true;
    } else if (current.thread.used_vr) {
    memset(&current.thread.vr_state, 0,
    ELF_NVRREG * sizeof(vector128));
    memset(&current.thread.ckvr_state, 0,
    ELF_NVRREG * sizeof(vector128));
    }
// Always get VRSAVE back
    unsafe_get_user(current.thread.ckvrsave,
    (u32 __user *)&sr.mc_vregs[32], failed);
    if (cpu_has_feature(CPU_FTR_ALTIVEC))
    mtspr(SPRN_VRSAVE, current.thread.ckvrsave);
    regs_set_return_msr(regs, regs.msr & ~(MSR_FP | MSR_FE0 | MSR_FE1));
    unsafe_copy_fpr_from_user(current, &sr.mc_fregs, failed);
    regs_set_return_msr(regs, regs.msr & ~MSR_VSX);
    if (msr & MSR_VSX) {
//
// Restore altivec registers from the stack to a local
// buffer, then write this out to the thread_struct
//
    unsafe_copy_ckvsx_from_user(current, &sr.mc_vsregs, failed);
    current.thread.used_vsr = true;
    } else if (current.thread.used_vsr)
    for (i = 0; i < 32 ; i++) {
    current.thread.fp_state.fpr[i][TS_VSRLOWOFFSET] = 0;
    current.thread.ckfp_state.fpr[i][TS_VSRLOWOFFSET] = 0;
    }
    user_read_access_end();
    if (!user_read_access_begin(tm_sr, sizeof(*tm_sr)))
    return 1;
    unsafe_restore_general_regs(regs, tm_sr, failed);
// restore altivec registers from the stack
    if (msr & MSR_VEC)
    unsafe_copy_from_user(&current.thread.vr_state, &tm_sr.mc_vregs,
    sizeof(sr.mc_vregs), failed);
// Always get VRSAVE back
    unsafe_get_user(current.thread.vrsave,
    (u32 __user *)&tm_sr.mc_vregs[32], failed);
    unsafe_copy_ckfpr_from_user(current, &tm_sr.mc_fregs, failed);
    if (msr & MSR_VSX) {
//
// Restore altivec registers from the stack to a local
// buffer, then write this out to the thread_struct
//
    unsafe_copy_vsx_from_user(current, &tm_sr.mc_vsregs, failed);
    current.thread.used_vsr = true;
    }
// Get the top half of the MSR from the user context
    unsafe_get_user(msr_hi, &tm_sr.mc_gregs[PT_MSR], failed);
    msr_hi <<= 32;
    user_read_access_end();
// If TM bits are set to the reserved value, it's an invalid context
    if (MSR_TM_RESV(msr_hi))
    return 1;
//
// Disabling preemption, since it is unsafe to be preempted
// with MSR[TS] set without recheckpointing.
//
    preempt_disable();
//
// CAUTION:
// After regs->MSR[TS] being updated, make sure that get_user(),
// put_user() or similar functions are *not* called. These
// functions can generate page faults which will cause the process
// to be de-scheduled with MSR[TS] set but without calling
// tm_recheckpoint(). This can cause a bug.
//
// Pull in the MSR TM bits from the user context
//
    regs_set_return_msr(regs, (regs.msr & ~MSR_TS_MASK) | (msr_hi & MSR_TS_MASK));
// Now, recheckpoint.  This loads up all of the checkpointed (older)
// registers, including FP and V[S]Rs.  After recheckpointing, the
// transactional versions should be loaded.
//
    tm_enable();
// Make sure the transaction is marked as failed
    current.thread.tm_texasr |= TEXASR_FS;
// This loads the checkpointed FP/VEC state, if used
    tm_recheckpoint(&current.thread);
// This loads the speculative FP/VEC state, if used
    msr_check_and_set(msr & (MSR_FP | MSR_VEC));
    if (msr & MSR_FP) {
    load_fp_state(&current.thread.fp_state);
    regs_set_return_msr(regs, regs.msr | (MSR_FP | current.thread.fpexc_mode));
    }
    if (msr & MSR_VEC) {
    load_vr_state(&current.thread.vr_state);
    regs_set_return_msr(regs, regs.msr | MSR_VEC);
    }
    preempt_enable();
    return 0;
    failed:
    user_read_access_end();
    return 1;
    }

    static long restore_tm_user_regs(struct pt_regs *regs, struct mcontext __user *sr,
    struct mcontext __user *tm_sr)
    {
    return 0;
    }

//
// Set up a signal frame for a "real-time" signal handler
// (one which gets siginfo).
//
    int handle_rt_signal32(struct ksignal *ksig, sigset_t *oldset,
    struct task_struct *tsk)
    {
    struct rt_sigframe __user *frame;
    struct mcontext __user *mctx;
    struct mcontext __user *tm_mctx = core::ptr::null_mut();
    let mut newsp: c_ulong = 0;
    unsigned long tramp;
    struct pt_regs *regs = tsk.thread.regs;
// Save the thread's msr before get_tm_stackpointer() changes it
    let mut msr: c_ulong = regs.msr;
// Set up Signal Frame
    frame = get_sigframe(ksig, tsk, sizeof(*frame), 1);
    mctx = &frame.uc.uc_mcontext;

    tm_mctx = &frame.uc_transact.uc_mcontext;

    if (MSR_TM_ACTIVE(msr))
    prepare_save_tm_user_regs();
    else
    prepare_save_user_regs(1);
    if (!user_access_begin(frame, sizeof(*frame)))
    goto badframe;
// Put the siginfo & fill in most of the ucontext
    unsafe_put_user(0, &frame.uc.uc_flags, failed);

    unsafe_compat_save_altstack(&frame.uc.uc_stack, regs.gpr[1], failed);

    unsafe_save_altstack(&frame.uc.uc_stack, regs.gpr[1], failed);

    unsafe_put_user(to_user_ptr(&frame.uc.uc_mcontext), &frame.uc.uc_regs, failed);
    if (MSR_TM_ACTIVE(msr)) {

    unsafe_put_user((unsigned long)&frame.uc_transact,
    &frame.uc.uc_link, failed);
    unsafe_put_user((unsigned long)tm_mctx,
    &frame.uc_transact.uc_regs, failed);

    unsafe_save_tm_user_regs(regs, mctx, tm_mctx, msr, failed);
    } else {
    unsafe_put_user(0, &frame.uc.uc_link, failed);
    unsafe_save_user_regs(regs, mctx, tm_mctx, 1, failed);
    }
// Save user registers on the stack
    if (tsk.mm.context.vdso) {
    tramp = VDSO32_SYMBOL(tsk.mm.context.vdso, sigtramp_rt32);
    } else {
    tramp = (unsigned long)mctx.mc_pad;
    unsafe_put_user(PPC_RAW_LI(_R0, __NR_rt_sigreturn), &mctx.mc_pad[0], failed);
    unsafe_put_user(PPC_RAW_SC(), &mctx.mc_pad[1], failed);
    asm("dcbst %y0; sync; icbi %y0; sync" :: "Z" (mctx.mc_pad[0]));
    }
    unsafe_put_sigset_t(&frame.uc.uc_sigmask, oldset, failed);
    user_access_end();
    if (copy_siginfo_to_user(&frame.info, &ksig.info))
    goto badframe;
    regs.link = tramp;

    tsk.thread.fp_state.fpscr = 0;	/* turn off all fp exceptions */

// create a stack frame for the caller of the handler
    newsp = ((unsigned long)frame) - (__SIGNAL_FRAMESIZE + 16);
    if (put_user(regs.gpr[1], (u32 __user *)newsp))
    goto badframe;
// Fill registers for signal handler
    regs.gpr[1] = newsp;
    regs.gpr[3] = ksig.sig;
    regs.gpr[4] = (unsigned long)&frame.info;
    regs.gpr[5] = (unsigned long)&frame.uc;
    regs.gpr[6] = (unsigned long)frame;
    regs_set_return_ip(regs, (unsigned long) ksig.ka.sa.sa_handler);
// enter the signal handler in native-endian mode
    regs_set_return_msr(regs, (regs.msr & ~MSR_LE) | (MSR_KERNEL & MSR_LE));
    return 0;
    failed:
    user_access_end();
    badframe:
    signal_fault(tsk, regs, "handle_rt_signal32", frame);
    return 1;
    }
//
// OK, we're invoking a handler
//
    int handle_signal32(struct ksignal *ksig, sigset_t *oldset,
    struct task_struct *tsk)
    {
    struct sigcontext __user *sc;
    struct sigframe __user *frame;
    struct mcontext __user *mctx;
    struct mcontext __user *tm_mctx = core::ptr::null_mut();
    let mut newsp: c_ulong = 0;
    unsigned long tramp;
    struct pt_regs *regs = tsk.thread.regs;
// Save the thread's msr before get_tm_stackpointer() changes it
    let mut msr: c_ulong = regs.msr;
// Set up Signal Frame
    frame = get_sigframe(ksig, tsk, sizeof(*frame), 1);
    mctx = &frame.mctx;

    tm_mctx = &frame.mctx_transact;

    if (MSR_TM_ACTIVE(msr))
    prepare_save_tm_user_regs();
    else
    prepare_save_user_regs(1);
    if (!user_access_begin(frame, sizeof(*frame)))
    goto badframe;
    sc = (struct sigcontext __user *) &frame.sctx;

    unsafe_put_user(to_user_ptr(ksig.ka.sa.sa_handler), &sc.handler, failed);
    unsafe_put_user(oldset.sig[0], &sc.oldmask, failed);

    unsafe_put_user((oldset.sig[0] >> 32), &sc._unused[3], failed);

    unsafe_put_user(oldset.sig[1], &sc._unused[3], failed);

    unsafe_put_user(to_user_ptr(mctx), &sc.regs, failed);
    unsafe_put_user(ksig.sig, &sc.signal, failed);
    if (MSR_TM_ACTIVE(msr))
    unsafe_save_tm_user_regs(regs, mctx, tm_mctx, msr, failed);
    else
    unsafe_save_user_regs(regs, mctx, tm_mctx, 1, failed);
    if (tsk.mm.context.vdso) {
    tramp = VDSO32_SYMBOL(tsk.mm.context.vdso, sigtramp32);
    } else {
    tramp = (unsigned long)mctx.mc_pad;
    unsafe_put_user(PPC_RAW_LI(_R0, __NR_sigreturn), &mctx.mc_pad[0], failed);
    unsafe_put_user(PPC_RAW_SC(), &mctx.mc_pad[1], failed);
    asm("dcbst %y0; sync; icbi %y0; sync" :: "Z" (mctx.mc_pad[0]));
    }
    user_access_end();
    regs.link = tramp;

    tsk.thread.fp_state.fpscr = 0;	/* turn off all fp exceptions */

// create a stack frame for the caller of the handler
    newsp = ((unsigned long)frame) - __SIGNAL_FRAMESIZE;
    if (put_user(regs.gpr[1], (u32 __user *)newsp))
    goto badframe;
    regs.gpr[1] = newsp;
    regs.gpr[3] = ksig.sig;
    regs.gpr[4] = (unsigned long) sc;
    regs_set_return_ip(regs, (unsigned long) ksig.ka.sa.sa_handler);
// enter the signal handler in native-endian mode
    regs_set_return_msr(regs, (regs.msr & ~MSR_LE) | (MSR_KERNEL & MSR_LE));
    return 0;
    failed:
    user_access_end();
    badframe:
    signal_fault(tsk, regs, "handle_signal32", frame);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn do_setcontext(ucp: *mut ucontext __user, regs: *mut pt_regs, sig: c_int) -> c_int {
    static int do_setcontext(struct ucontext __user *ucp, struct pt_regs *regs, int sig)
    {
    sigset_t set;
    struct mcontext __user *mcp;
    if (!user_read_access_begin(ucp, sizeof(*ucp)))
    return -EFAULT;
    unsafe_get_sigset_t(&set, &ucp.uc_sigmask, failed);

    {
    u32 cmcp;
    unsafe_get_user(cmcp, &ucp.uc_regs, failed);
    mcp = (struct mcontext __user *)(u64)cmcp;
    }

    unsafe_get_user(mcp, &ucp.uc_regs, failed);

    user_read_access_end();
    set_current_blocked(&set);
    if (restore_user_regs(regs, mcp, sig))
    return -EFAULT;
    return 0;
    failed:
    user_read_access_end();
    return -EFAULT;
    }

    static int do_setcontext_tm(struct ucontext __user *ucp,
    struct ucontext __user *tm_ucp,
    struct pt_regs *regs)
    {
    sigset_t set;
    struct mcontext __user *mcp;
    struct mcontext __user *tm_mcp;
    u32 cmcp;
    u32 tm_cmcp;
    if (!user_read_access_begin(ucp, sizeof(*ucp)))
    return -EFAULT;
    unsafe_get_sigset_t(&set, &ucp.uc_sigmask, failed);
    unsafe_get_user(cmcp, &ucp.uc_regs, failed);
    user_read_access_end();
    if (__get_user(tm_cmcp, &tm_ucp.uc_regs))
    return -EFAULT;
    mcp = (struct mcontext __user *)(u64)cmcp;
    tm_mcp = (struct mcontext __user *)(u64)tm_cmcp;
// no need to check access_ok(mcp), since mcp < 4GB
    set_current_blocked(&set);
    if (restore_tm_user_regs(regs, mcp, tm_mcp))
    return -EFAULT;
    return 0;
    failed:
    user_read_access_end();
    return -EFAULT;
    }

    COMPAT_SYSCALL_DEFINE3(swapcontext, struct ucontext __user *, old_ctx,
    struct ucontext __user *, new_ctx, int, ctx_size)

    SYSCALL_DEFINE3(swapcontext, struct ucontext __user *, old_ctx,
    struct ucontext __user *, new_ctx, long, ctx_size)

    {
    struct pt_regs *regs = current_pt_regs();
    let mut ctx_has_vsx_region: c_int = 0;

    let mut new_msr: c_ulong = 0;
    if (new_ctx) {
    struct mcontext __user *mcp;
    u32 cmcp;
//
// Get pointer to the real mcontext.  No need for
// access_ok since we are dealing with compat
// pointers.
//
    if (__get_user(cmcp, &new_ctx.uc_regs))
    return -EFAULT;
    mcp = (struct mcontext __user *)(u64)cmcp;
    if (__get_user(new_msr, &mcp.mc_gregs[PT_MSR]))
    return -EFAULT;
    }
//
// Check that the context is not smaller than the original
// size (with VMX but without VSX)
//
    if (ctx_size < UCONTEXTSIZEWITHOUTVSX)
    return -EINVAL;
//
// If the new context state sets the MSR VSX bits but
// it doesn't provide VSX state.
//
    if ((ctx_size < sizeof(struct ucontext)) &&
    (new_msr & MSR_VSX))
    return -EINVAL;
// Does the context have enough room to store VSX data?
    if (ctx_size >= sizeof(struct ucontext))
    ctx_has_vsx_region = 1;

// Context size is for future use. Right now, we only make sure
// we are passed something we understand
//
    if (ctx_size < sizeof(struct ucontext))
    return -EINVAL;

    if (old_ctx != core::ptr::null_mut()) {
    struct mcontext __user *mctx;
//
// old_ctx might not be 16-byte aligned, in which
// case old_ctx->uc_mcontext won't be either.
// Because we have the old_ctx->uc_pad2 field
// before old_ctx->uc_mcontext, we need to round down
// from &old_ctx->uc_mcontext to a 16-byte boundary.
//
    mctx = (struct mcontext __user *)
    ((unsigned long) &old_ctx.uc_mcontext & ~0xfUL);
    prepare_save_user_regs(ctx_has_vsx_region);
    if (!user_write_access_begin(old_ctx, ctx_size))
    return -EFAULT;
    unsafe_save_user_regs(regs, mctx, core::ptr::null_mut(), ctx_has_vsx_region, failed);
    unsafe_put_sigset_t(&old_ctx.uc_sigmask, &current.blocked, failed);
    unsafe_put_user(to_user_ptr(mctx), &old_ctx.uc_regs, failed);
    user_write_access_end();
    }
    if (new_ctx == core::ptr::null_mut())
    return 0;
    if (!access_ok(new_ctx, ctx_size) ||
    fault_in_readable((char __user *)new_ctx, ctx_size))
    return -EFAULT;
//
// If we get a fault copying the context into the kernel's
// image of the user's registers, we can't just return -EFAULT
// because the user's registers will be corrupted.  For instance
// the NIP value may have been updated but not some of the
// other registers.  Given that we have done the access_ok
// and successfully read the first and last bytes of the region
// above, this should only happen in an out-of-memory situation
// or if another thread unmaps the region containing the context.
// We kill the task with a SIGSEGV in this situation.
//
    if (do_setcontext(new_ctx, regs, 0)) {
    force_exit_sig(SIGSEGV);
    return -EFAULT;
    }
    set_thread_flag(TIF_RESTOREALL);
    return 0;
    failed:
    user_write_access_end();
    return -EFAULT;
    }

    COMPAT_SYSCALL_DEFINE0(rt_sigreturn)

    SYSCALL_DEFINE0(rt_sigreturn)

    {
    struct rt_sigframe __user *rt_sf;
    struct pt_regs *regs = current_pt_regs();
    let mut tm_restore: c_int = 0;

    struct ucontext __user *uc_transact;
    unsigned long msr_hi;
    unsigned long tmp;

// Always make any pending restarted system calls return -EINTR
    current.restart_block.fn = do_no_restart_syscall;
    rt_sf = (struct rt_sigframe __user *)
    (regs.gpr[1] + __SIGNAL_FRAMESIZE + 16);
    if (!access_ok(rt_sf, sizeof(*rt_sf)))
    goto bad;

//
// If there is a transactional state then throw it away.
// The purpose of a sigreturn is to destroy all traces of the
// signal frame, this includes any transactional state created
// within in. We only check for suspended as we can never be
// active in the kernel, we are active, there is nothing better to
// do than go ahead and Bad Thing later.
// The cause is not important as there will never be a
// recheckpoint so it's not user visible.
//
    if (MSR_TM_SUSPENDED(mfmsr()))
    tm_reclaim_current(0);
    if (__get_user(tmp, &rt_sf.uc.uc_link))
    goto bad;
    uc_transact = (struct ucontext __user *)(uintptr_t)tmp;
    if (uc_transact) {
    u32 cmcp;
    struct mcontext __user *mcp;
    if (__get_user(cmcp, &uc_transact.uc_regs))
    return -EFAULT;
    mcp = (struct mcontext __user *)(u64)cmcp;
// The top 32 bits of the MSR are stashed in the transactional
// ucontext.
    if (__get_user(msr_hi, &mcp.mc_gregs[PT_MSR]))
    goto bad;
    if (MSR_TM_ACTIVE(msr_hi<<32)) {
// Trying to start TM on non TM system
    if (!cpu_has_feature(CPU_FTR_TM))
    goto bad;
// We only recheckpoint on return if we're
// transaction.
//
    tm_restore = 1;
    if (do_setcontext_tm(&rt_sf.uc, uc_transact, regs))
    goto bad;
    }
    }
    if (!tm_restore) {
//
// Unset regs->msr because ucontext MSR TS is not
// set, and recheckpoint was not called. This avoid
// hitting a TM Bad thing at RFID
//
    regs_set_return_msr(regs, regs.msr & ~MSR_TS_MASK);
    }
// Fall through, for non-TM restore

    if (!tm_restore)
    if (do_setcontext(&rt_sf.uc, regs, 1))
    goto bad;
//
// It's not clear whether or why it is desirable to save the
// sigaltstack setting on signal delivery and restore it on
// signal return.  But other architectures do this and we have
// always done it up until now so it is probably better not to
// change it.  -- paulus
//

    if (compat_restore_altstack(&rt_sf.uc.uc_stack))
    goto bad;

    if (restore_altstack(&rt_sf.uc.uc_stack))
    goto bad;

    set_thread_flag(TIF_RESTOREALL);
    return 0;
    bad:
    signal_fault(current, regs, "sys_rt_sigreturn", rt_sf);
    force_sig(SIGSEGV);
    return 0;
    }

    SYSCALL_DEFINE3(debug_setcontext, struct ucontext __user *, ctx,
    int, ndbg, struct sig_dbg_op __user *, dbg)
    {
    struct pt_regs *regs = current_pt_regs();
    struct sig_dbg_op op;
    int i;
    let mut new_msr: c_ulong = regs.msr;

    let mut new_dbcr0: c_ulong = current.thread.debug.dbcr0;

    for (i=0; i<ndbg; i++) {
    if (copy_from_user(&op, dbg + i, sizeof(op)))
    return -EFAULT;
    switch (op.dbg_type) {
    case SIG_DBG_SINGLE_STEPPING:

    if (op.dbg_value) {
    new_msr |= MSR_DE;
    new_dbcr0 |= (DBCR0_IDM | DBCR0_IC);
    } else {
    new_dbcr0 &= ~DBCR0_IC;
    if (!DBCR_ACTIVE_EVENTS(new_dbcr0,
    current.thread.debug.dbcr1)) {
    new_msr &= ~MSR_DE;
    new_dbcr0 &= ~DBCR0_IDM;
    }
    }

    if (op.dbg_value)
    new_msr |= MSR_SE;
    else
    new_msr &= ~MSR_SE;

    break;
    case SIG_DBG_BRANCH_TRACING:

    return -EINVAL;

    if (op.dbg_value)
    new_msr |= MSR_BE;
    else
    new_msr &= ~MSR_BE;

    break;
    default:
    return -EINVAL;
    }
    }
// We wait until here to actually install the values in the
    registers so if we fail in the above loop, it will not
    affect the contents of these registers.  After this point,
    failure is a problem, anyway, and it's very unlikely unless
    the user is really doing something wrong. */
    regs_set_return_msr(regs, new_msr);

    current.thread.debug.dbcr0 = new_dbcr0;

    if (!access_ok(ctx, sizeof(*ctx)) ||
    fault_in_readable((char __user *)ctx, sizeof(*ctx)))
    return -EFAULT;
//
// If we get a fault copying the context into the kernel's
// image of the user's registers, we can't just return -EFAULT
// because the user's registers will be corrupted.  For instance
// the NIP value may have been updated but not some of the
// other registers.  Given that we have done the access_ok
// and successfully read the first and last bytes of the region
// above, this should only happen in an out-of-memory situation
// or if another thread unmaps the region containing the context.
// We kill the task with a SIGSEGV in this situation.
//
    if (do_setcontext(ctx, regs, 1)) {
    signal_fault(current, regs, "sys_debug_setcontext", ctx);
    force_sig(SIGSEGV);
    goto out;
    }
//
// It's not clear whether or why it is desirable to save the
// sigaltstack setting on signal delivery and restore it on
// signal return.  But other architectures do this and we have
// always done it up until now so it is probably better not to
// change it.  -- paulus
//
    restore_altstack(&ctx.uc_stack);
    set_thread_flag(TIF_RESTOREALL);
    out:
    return 0;
    }

//
// Do a signal return; undo the signal stack.
//

    COMPAT_SYSCALL_DEFINE0(sigreturn)

    SYSCALL_DEFINE0(sigreturn)

    {
    struct pt_regs *regs = current_pt_regs();
    struct sigframe __user *sf;
    struct sigcontext __user *sc;
    struct sigcontext sigctx;
    struct mcontext __user *sr;
    sigset_t set;
    struct mcontext __user *mcp;
    struct mcontext __user *tm_mcp = core::ptr::null_mut();
    let mut msr_hi: c_ulonglong = 0;
// Always make any pending restarted system calls return -EINTR
    current.restart_block.fn = do_no_restart_syscall;
    sf = (struct sigframe __user *)(regs.gpr[1] + __SIGNAL_FRAMESIZE);
    sc = &sf.sctx;
    if (copy_from_user(&sigctx, sc, sizeof(sigctx)))
    goto badframe;

//
// Note that PPC32 puts the upper 32 bits of the sigmask in the
// unused part of the signal stackframe
//
    set.sig[0] = sigctx.oldmask + ((long)(sigctx._unused[3]) << 32);

    set.sig[0] = sigctx.oldmask;
    set.sig[1] = sigctx._unused[3];

    set_current_blocked(&set);
    mcp = (struct mcontext __user *)&sf.mctx;

    tm_mcp = (struct mcontext __user *)&sf.mctx_transact;
    if (__get_user(msr_hi, &tm_mcp.mc_gregs[PT_MSR]))
    goto badframe;

    if (MSR_TM_ACTIVE(msr_hi<<32)) {
    if (!cpu_has_feature(CPU_FTR_TM))
    goto badframe;
    if (restore_tm_user_regs(regs, mcp, tm_mcp))
    goto badframe;
    } else {
    sr = (struct mcontext __user *)from_user_ptr(sigctx.regs);
    if (restore_user_regs(regs, sr, 1)) {
    signal_fault(current, regs, "sys_sigreturn", sr);
    force_sig(SIGSEGV);
    return 0;
    }
    }
    set_thread_flag(TIF_RESTOREALL);
    return 0;
    badframe:
    signal_fault(current, regs, "sys_sigreturn", sc);
    force_sig(SIGSEGV);
    return 0;
    }
