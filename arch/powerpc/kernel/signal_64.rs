//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/signal_64.c
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
// PowerPC version
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Derived from "arch/i386/kernel/signal.c"
// Copyright (C) 1991, 1992 Linus Torvalds
// 1997-11-28  Modified for POSIX.1b signals by Richard Henderson
//

pub const TRAMP_TRACEBACK: c_int = 4;
pub const TRAMP_SIZE: c_int = 7;
//
// When we have signals to deliver, we set up on the user stack,
// going down from the original stack pointer:
// 1) a rt_sigframe struct which contains the ucontext
// 2) a gap of __SIGNAL_FRAMESIZE bytes which acts as a dummy caller
// frame for the signal handler.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_sigframe {
// sys_rt_sigreturn requires the ucontext be the first field
    pub uc: ucontext,

    pub uc_transact: ucontext,
    pub _unused: [c_ulong; 2],
    pub tramp: [c_uint; TRAMP_SIZE],
    pub pinfo: *mut siginfo __user,
    pub puc: *mut void __user,
    pub info: siginfo,
// New 64 bit little-endian ABI allows redzone of 512 bytes below sp
    pub abigap: [c_char; USER_REDZONE_SIZE],
// C attribute field omitted
#[no_mangle]
pub unsafe extern "C" fn get_min_sigframe_size_64() -> c_ulong {
    unsigned long get_min_sigframe_size_64(void)
    {
    pub __SIGNAL_FRAMESIZE: return sizeof(struct rt_sigframe) +,
    }
//
// This computes a quad word aligned pointer inside the vmx_reserve array
// element. For historical reasons sigcontext might not be quad word aligned,
// but the location we write the VMX regs to must be. See the comment in
// sigcontext for more detail.
//

    static elf_vrreg_t __user *sigcontext_vmx_regs(struct sigcontext __user *sc)
    {
    pub ~0xful): *mut *mut return (elf_vrreg_t __user ) (((unsigned long)sc->vmx_reserve + 15) &,
    }

#[no_mangle]
unsafe extern "C" fn prepare_setup_sigcontext(tsk: *mut task_struct) {
    static void prepare_setup_sigcontext(struct task_struct *tsk)
    {

// save altivec registers
    if (tsk.thread.used_vr)
    if (cpu_has_feature(CPU_FTR_ALTIVEC))
    pub mfspr(SPRN_VRSAVE): tsk->thread.vrsave =,

    if (tsk.thread.used_vsr)

    }
//
// Set up the sigcontext for the signal frame.
//

    do {											\
    if (__unsafe_setup_sigcontext(sc, tsk, signr, set, handler, ctx_has_vsx_region))\
    pub \: goto label;,
    } while (0)
    static long notrace __unsafe_setup_sigcontext(struct sigcontext __user *sc,
    struct task_struct *tsk, int signr, sigset_t *set,
    unsigned long handler, int ctx_has_vsx_region)
    {
// When CONFIG_ALTIVEC is set, we _always_ setup v_regs even if the
// process never used altivec yet (MSR_VEC is zero in pt_regs of
// the context). This is very important because we must ensure we
// don't lose the VRSAVE content that may have been set prior to
// the process doing its first vector operation
// Userland shall check AT_HWCAP to know whether it can rely on the
// v_regs pointer or not
//

    pub sigcontext_vmx_regs(sc): *mut *mut elf_vrreg_t __user v_regs =,

    pub tsk->thread.regs: *mut *mut pt_regs regs =,
    pub regs->msr: unsigned long msr =,
// Force usr to always see softe as 1 (interrupts enabled)
    pub 0x1: unsigned long softe =,
    pub current): BUG_ON(tsk !=,

    pub efault_out): unsafe_put_user(v_regs, &sc->v_regs,,
// save altivec registers
    if (tsk.thread.used_vr) {
// Copy 33 vec registers (vr0..31 and vscr) to the stack
    unsafe_copy_to_user(v_regs, &tsk.thread.vr_state,
    pub efault_out): *mut *mut 33  sizeof(vector128),,
// set MSR_VEC in the MSR value in the frame to indicate that sc->v_reg)
// contains valid data.
//
    pub MSR_VEC: msr |=,
    }
// We always copy to/from vrsave, it's 0 if we don't have or don't
// use altivec.
//
    pub efault_out): *mut *mut unsafe_put_user(tsk->thread.vrsave, (u32 __user )&v_regs[33],,

    pub efault_out): unsafe_put_user(0, &sc->v_regs,,

// copy fpr regs and fpscr
    pub efault_out): unsafe_copy_fpr_to_user(&sc->fp_regs, tsk,,
//
// Clear the MSR VSX bit to indicate there is no valid state attached
// to this context, except in the specific case below where we set it.
//
    pub ~MSR_VSX: msr &=,

//
// Copy VSX low doubleword to local buffer for formatting,
// then out to userspace.  Update v_regs to point after the
// VMX data.
//
    if (tsk.thread.used_vsr && ctx_has_vsx_region) {
    pub ELF_NVRREG: v_regs +=,
    pub efault_out): unsafe_copy_vsx_to_user(v_regs, tsk,,
// set MSR_VSX in the MSR value in the frame to
// indicate that sc->vs_reg) contains valid data.
//
    pub MSR_VSX: msr |=,
    }

    pub efault_out): unsafe_put_user(&sc->gp_regs, &sc->regs,,
    pub efault_out): unsafe_copy_to_user(&sc->gp_regs, regs, GP_REGS_SIZE,,
    pub efault_out): unsafe_put_user(msr, &sc->gp_regs[PT_MSR],,
    pub efault_out): unsafe_put_user(softe, &sc->gp_regs[PT_SOFTE],,
    pub efault_out): unsafe_put_user(signr, &sc->signal,,
    pub efault_out): unsafe_put_user(handler, &sc->handler,,
    if (set != core::ptr::null_mut())
    pub efault_out): unsafe_put_user(set->sig[0], &sc->oldmask,,
    pub 0: return,
    efault_out:
    pub -EFAULT: return,
    }

//
// As above, but Transactional Memory is in use, so deliver sigcontexts
// containing checkpointed and transactional register states.
//
// To do this, we treclaim (done before entering here) to gather both sets of
// registers and set up the 'normal' sigcontext registers with rolled-back
// register values such that a simple signal handler sees a correct
// checkpointed register state.  If interested, a TM-aware sighandler can
// examine the transactional registers in the 2nd sigcontext to determine the
// real origin of the signal.
//
    static long setup_tm_sigcontexts(struct sigcontext __user *sc,
    struct sigcontext __user *tm_sc,
    struct task_struct *tsk,
    int signr, sigset_t *set, unsigned long handler,
    unsigned long msr)
    {
// When CONFIG_ALTIVEC is set, we _always_ setup v_regs even if the
// process never used altivec yet (MSR_VEC is zero in pt_regs of
// the context). This is very important because we must ensure we
// don't lose the VRSAVE content that may have been set prior to
// the process doing its first vector operation
// Userland shall check AT_HWCAP to know wether it can rely on the
// v_regs pointer or not.
//

    pub sigcontext_vmx_regs(sc): *mut *mut elf_vrreg_t __user v_regs =,
    pub sigcontext_vmx_regs(tm_sc): *mut *mut elf_vrreg_t __user tm_v_regs =,

    pub tsk->thread.regs: *mut *mut pt_regs regs =,
    pub 0: long err =,
    pub current): BUG_ON(tsk !=,
// Restore checkpointed FP, VEC, and VSX bits from ckpt_regs as
// it contains the correct FP, VEC, VSX state after we treclaimed
// the transaction and giveup_all() was called on reclaiming.
//
    pub MSR_VSX): msr |= tsk->thread.ckpt_regs.msr & (MSR_FP | MSR_VEC |,

    pub &sc->v_regs): err |= __put_user(v_regs,,
    pub &tm_sc->v_regs): err |= __put_user(tm_v_regs,,
// save altivec registers
    if (tsk.thread.used_vr) {
// Copy 33 vec registers (vr0..31 and vscr) to the stack
    err |= __copy_to_user(v_regs, &tsk.thread.ckvr_state,
    pub sizeof(vector128)): *mut *mut 33,
// If VEC was enabled there are transactional VRs valid too,
// else they're a copy of the checkpointed VRs.
//
    if (msr & MSR_VEC)
    err |= __copy_to_user(tm_v_regs,
    &tsk.thread.vr_state,
    pub sizeof(vector128)): *mut *mut 33,
    else
    err |= __copy_to_user(tm_v_regs,
    &tsk.thread.ckvr_state,
    pub sizeof(vector128)): *mut *mut 33,
// set MSR_VEC in the MSR value in the frame to indicate
// that sc->v_reg contains valid data.
//
    pub MSR_VEC: msr |=,
    }
// We always copy to/from vrsave, it's 0 if we don't have or don't
// use altivec.
//
    if (cpu_has_feature(CPU_FTR_ALTIVEC))
    pub mfspr(SPRN_VRSAVE): tsk->thread.ckvrsave =,
    pub )&v_regs[33]): *mut err |= __put_user(tsk->thread.ckvrsave, (u32 __user,
    if (msr & MSR_VEC)
    err |= __put_user(tsk.thread.vrsave,
    pub )&tm_v_regs[33]): *mut (u32 __user,
    else
    err |= __put_user(tsk.thread.ckvrsave,
    pub )&tm_v_regs[33]): *mut (u32 __user,

    pub &sc->v_regs): err |= __put_user(0,,
    pub &tm_sc->v_regs): err |= __put_user(0,,

// copy fpr regs and fpscr
    pub tsk): err |= copy_ckfpr_to_user(&sc->fp_regs,,
    if (msr & MSR_FP)
    pub tsk): err |= copy_fpr_to_user(&tm_sc->fp_regs,,
    else
    pub tsk): err |= copy_ckfpr_to_user(&tm_sc->fp_regs,,

//
// Copy VSX low doubleword to local buffer for formatting,
// then out to userspace.  Update v_regs to point after the
// VMX data.
//
    if (tsk.thread.used_vsr) {
    pub ELF_NVRREG: v_regs +=,
    pub ELF_NVRREG: tm_v_regs +=,
    pub tsk): err |= copy_ckvsx_to_user(v_regs,,
    if (msr & MSR_VSX)
    pub tsk): err |= copy_vsx_to_user(tm_v_regs,,
    else
    pub tsk): err |= copy_ckvsx_to_user(tm_v_regs,,
// set MSR_VSX in the MSR value in the frame to
// indicate that sc->vs_reg) contains valid data.
//
    pub MSR_VSX: msr |=,
    }

    pub &sc->regs): err |= __put_user(&sc->gp_regs,,
    pub &tm_sc->regs): err |= __put_user(&tm_sc->gp_regs,,
    pub GP_REGS_SIZE): err |= __copy_to_user(&tm_sc->gp_regs, regs,,
    err |= __copy_to_user(&sc.gp_regs,
    pub GP_REGS_SIZE): &tsk->thread.ckpt_regs,,
    pub &tm_sc->gp_regs[PT_MSR]): err |= __put_user(msr,,
    pub &sc->gp_regs[PT_MSR]): err |= __put_user(msr,,
    pub &sc->signal): err |= __put_user(signr,,
    pub &sc->handler): err |= __put_user(handler,,
    if (set != core::ptr::null_mut())
    pub &sc->oldmask): err |= __put_user(set->sig[0],,
    pub err: return,
    }

//
// Restore the sigcontext from the signal frame.
//

    if (__unsafe_restore_sigcontext(tsk, set, sig, sc))		\
    pub \: goto label;,
    } while (0)
    static long notrace __unsafe_restore_sigcontext(struct task_struct *tsk, sigset_t *set,
    int sig, struct sigcontext __user *sc)
    {

    pub v_regs: *mut elf_vrreg_t __user,

    pub 0: unsigned long save_r13 =,
    pub msr: c_ulong,
    pub tsk->thread.regs: *mut *mut pt_regs regs =,

    pub i: c_int,

    pub current): BUG_ON(tsk !=,
// If this is not a signal return, we preserve the TLS in r13
    if (!sig)
    pub regs->gpr[13]: save_r13 =,
// copy the GPRs
    pub efault_out): unsafe_copy_from_user(regs->gpr, sc->gp_regs, sizeof(regs->gpr),,
    pub efault_out): unsafe_get_user(regs->nip, &sc->gp_regs[PT_NIP],,
// get MSR separately, transfer the LE bit if doing signal return
    pub efault_out): unsafe_get_user(msr, &sc->gp_regs[PT_MSR],,
    if (sig)
    pub MSR_LE)): regs_set_return_msr(regs, (regs->msr & ~MSR_LE) | (msr &,
    pub efault_out): unsafe_get_user(regs->orig_gpr3, &sc->gp_regs[PT_ORIG_R3],,
    pub efault_out): unsafe_get_user(regs->ctr, &sc->gp_regs[PT_CTR],,
    pub efault_out): unsafe_get_user(regs->link, &sc->gp_regs[PT_LNK],,
    pub efault_out): unsafe_get_user(regs->xer, &sc->gp_regs[PT_XER],,
    pub efault_out): unsafe_get_user(regs->ccr, &sc->gp_regs[PT_CCR],,
// Don't allow userspace to set SOFTE
    pub efault_out): unsafe_get_user(regs->dar, &sc->gp_regs[PT_DAR],,
    pub efault_out): unsafe_get_user(regs->dsisr, &sc->gp_regs[PT_DSISR],,
    pub efault_out): unsafe_get_user(regs->result, &sc->gp_regs[PT_RESULT],,
    if (!sig)
    pub save_r13: regs->gpr[13] =,
    if (set != core::ptr::null_mut())
    pub efault_out): unsafe_get_user(set->sig[0], &sc->oldmask,,
//
// Force reload of FP/VEC/VSX so userspace sees any changes.
// Clear these bits from the user process' MSR before copying into the
// thread struct. If we are rescheduled or preempted and another task
// uses FP/VEC/VSX, and this process has the MSR bits set, then the
// context switch code will save the current CPU state into the
// thread_struct - possibly overwriting the data we are updating here.
//
    pub MSR_VSX)): regs_set_return_msr(regs, regs->msr & ~(MSR_FP | MSR_FE0 | MSR_FE1 | MSR_VEC |,

    pub efault_out): unsafe_get_user(v_regs, &sc->v_regs,,
    if (v_regs && !access_ok(v_regs, 34 * sizeof(vector128)))
    pub -EFAULT: return,
// Copy 33 vec registers (vr0..31 and vscr) from the stack
    if (v_regs != core::ptr::null_mut() && (msr & MSR_VEC) != 0) {
    unsafe_copy_from_user(&tsk.thread.vr_state, v_regs,
    pub efault_out): *mut *mut 33  sizeof(vector128),,
    pub true: tsk->thread.used_vr =,
    } else if (tsk.thread.used_vr) {
    pub sizeof(vector128)): *mut *mut memset(&tsk->thread.vr_state, 0, 33,
    }
// Always get VRSAVE back
    if (v_regs != core::ptr::null_mut())
    pub efault_out): *mut *mut unsafe_get_user(tsk->thread.vrsave, (u32 __user )&v_regs[33],,
    else
    pub 0: tsk->thread.vrsave =,
    if (cpu_has_feature(CPU_FTR_ALTIVEC))
    pub tsk->thread.vrsave): mtspr(SPRN_VRSAVE,,

// restore floating point
    pub efault_out): unsafe_copy_fpr_from_user(tsk, &sc->fp_regs,,

//
// Get additional VSX data. Update v_regs to point after the
// VMX data.  Copy VSX low doubleword from userspace to local
// buffer for formatting, then into the taskstruct.
//
    pub ELF_NVRREG: v_regs +=,
    if ((msr & MSR_VSX) != 0) {
    pub efault_out): unsafe_copy_vsx_from_user(tsk, v_regs,,
    pub true: tsk->thread.used_vsr =,
    } else {
    pub i++): for (i = 0; i < 32 ;,
    pub 0: tsk->thread.fp_state.fpr[i][TS_VSRLOWOFFSET] =,
    }

    pub 0: return,
    efault_out:
    pub -EFAULT: return,
    }

//
// Restore the two sigcontexts from the frame of a transactional processes.
//
    static long restore_tm_sigcontexts(struct task_struct *tsk,
    struct sigcontext __user *sc,
    struct sigcontext __user *tm_sc)
    {

    pub tm_v_regs: *mut *mut elf_vrreg_t __user v_regs,,

    pub 0: unsigned long err =,
    pub msr: c_ulong,
    pub tsk->thread.regs: *mut *mut pt_regs regs =,

    pub i: c_int,

    pub current): BUG_ON(tsk !=,
    if (tm_suspend_disabled)
    pub -EINVAL: return,
// copy the GPRs
    pub sizeof(regs->gpr)): err |= __copy_from_user(regs->gpr, tm_sc->gp_regs,,
    err |= __copy_from_user(&tsk.thread.ckpt_regs, sc.gp_regs,
//
// TFHAR is restored from the checkpointed 'wound-back' ucontext's NIP.
// TEXASR was set by the signal delivery reclaim, as was TFIAR.
// Users doing anything abhorrent like thread-switching w/ signals for
// TM-Suspended code will have to back TEXASR/TFIAR up themselves.
// For the case of getting a signal and simply returning from it,
// we don't need to re-copy them here.
//
    pub &tm_sc->gp_regs[PT_NIP]): err |= __get_user(regs->nip,,
    pub &sc->gp_regs[PT_NIP]): err |= __get_user(tsk->thread.tm_tfhar,,
// get MSR separately, transfer the LE bit if doing signal return
    pub &sc->gp_regs[PT_MSR]): err |= __get_user(msr,,
// Don't allow reserved mode.
    if (MSR_TM_RESV(msr))
    pub -EINVAL: return,
// pull in MSR LE from user context
    pub MSR_LE)): regs_set_return_msr(regs, (regs->msr & ~MSR_LE) | (msr &,
// The following non-GPR non-FPR non-VR state is also checkpointed:
    pub &tm_sc->gp_regs[PT_CTR]): err |= __get_user(regs->ctr,,
    pub &tm_sc->gp_regs[PT_LNK]): err |= __get_user(regs->link,,
    pub &tm_sc->gp_regs[PT_XER]): err |= __get_user(regs->xer,,
    pub &tm_sc->gp_regs[PT_CCR]): err |= __get_user(regs->ccr,,
    err |= __get_user(tsk.thread.ckpt_regs.ctr,
    err |= __get_user(tsk.thread.ckpt_regs.link,
    err |= __get_user(tsk.thread.ckpt_regs.xer,
    err |= __get_user(tsk.thread.ckpt_regs.ccr,
// Don't allow userspace to set SOFTE
// These regs are not checkpointed; they can go in 'regs'.
    pub &sc->gp_regs[PT_DAR]): err |= __get_user(regs->dar,,
    pub &sc->gp_regs[PT_DSISR]): err |= __get_user(regs->dsisr,,
    pub &sc->gp_regs[PT_RESULT]): err |= __get_user(regs->result,,
//
// Force reload of FP/VEC.
// This has to be done before copying stuff into tsk->thread.fpr/vr
// for the reasons explained in the previous comment.
//
    pub MSR_VSX)): regs_set_return_msr(regs, regs->msr & ~(MSR_FP | MSR_FE0 | MSR_FE1 | MSR_VEC |,

    pub &sc->v_regs): err |= __get_user(v_regs,,
    pub &tm_sc->v_regs): err |= __get_user(tm_v_regs,,
    if (err)
    pub err: return,
    if (v_regs && !access_ok(v_regs, 34 * sizeof(vector128)))
    pub -EFAULT: return,
    if (tm_v_regs && !access_ok(tm_v_regs, 34 * sizeof(vector128)))
    pub -EFAULT: return,
// Copy 33 vec registers (vr0..31 and vscr) from the stack
    if (v_regs != core::ptr::null_mut() && tm_v_regs != core::ptr::null_mut() && (msr & MSR_VEC) != 0) {
    err |= __copy_from_user(&tsk.thread.ckvr_state, v_regs,
    pub sizeof(vector128)): *mut *mut 33,
    err |= __copy_from_user(&tsk.thread.vr_state, tm_v_regs,
    pub sizeof(vector128)): *mut *mut 33,
    pub true: current->thread.used_vr =,
    }
#[no_mangle]
pub unsafe extern "C" fn if(_arg: tsk->thread.used_vr) -> else {
    pub sizeof(vector128)): *mut *mut memset(&tsk->thread.vr_state, 0, 33,
    pub sizeof(vector128)): *mut *mut memset(&tsk->thread.ckvr_state, 0, 33,
    }
// Always get VRSAVE back
    if (v_regs != core::ptr::null_mut() && tm_v_regs != core::ptr::null_mut()) {
    err |= __get_user(tsk.thread.ckvrsave,
    pub )&v_regs[33]): *mut (u32 __user,
    err |= __get_user(tsk.thread.vrsave,
    pub )&tm_v_regs[33]): *mut (u32 __user,
    }
    else {
    pub 0: tsk->thread.vrsave =,
    pub 0: tsk->thread.ckvrsave =,
    }
    if (cpu_has_feature(CPU_FTR_ALTIVEC))
    pub tsk->thread.vrsave): mtspr(SPRN_VRSAVE,,

// restore floating point
    pub &tm_sc->fp_regs): err |= copy_fpr_from_user(tsk,,
    pub &sc->fp_regs): err |= copy_ckfpr_from_user(tsk,,

//
// Get additional VSX data. Update v_regs to point after the
// VMX data.  Copy VSX low doubleword from userspace to local
// buffer for formatting, then into the taskstruct.
//
    if (v_regs && ((msr & MSR_VSX) != 0)) {
    pub ELF_NVRREG: v_regs +=,
    pub ELF_NVRREG: tm_v_regs +=,
    pub tm_v_regs): err |= copy_vsx_from_user(tsk,,
    pub v_regs): err |= copy_ckvsx_from_user(tsk,,
    pub true: tsk->thread.used_vsr =,
    } else {
    pub {: for (i = 0; i < 32 ; i++),
    pub 0: tsk->thread.fp_state.fpr[i][TS_VSRLOWOFFSET] =,
    pub 0: tsk->thread.ckfp_state.fpr[i][TS_VSRLOWOFFSET] =,
    }
    }

// Make sure the transaction is marked as failed
    pub TEXASR_FS: tsk->thread.tm_texasr |=,
//
// Disabling preemption, since it is unsafe to be preempted
// with MSR[TS] set without recheckpointing.
//
// pull in MSR TS bits from user context
    pub MSR_TS_MASK)): regs_set_return_msr(regs, regs->msr | (msr &,
//
// Ensure that TM is enabled in regs->msr before we leave the signal
// handler. It could be the case that (a) user disabled the TM bit
// through the manipulation of the MSR bits in uc_mcontext or (b) the
// TM bit was disabled because a sufficient number of context switches
// happened whilst in the signal handler and load_tm overflowed,
// disabling the TM bit. In either case we can end up with an illegal
// TM state leading to a TM Bad Thing when we return to userspace.
//
// CAUTION:
// After regs->MSR[TS] being updated, make sure that get_user(),
// put_user() or similar functions are *not* called. These
// functions can generate page faults which will cause the process
// to be de-scheduled with MSR[TS] set but without calling
// tm_recheckpoint(). This can cause a bug.
//
    pub MSR_TM): regs_set_return_msr(regs, regs->msr |,
// This loads the checkpointed FP/VEC state, if used
    pub MSR_VEC)): msr_check_and_set(msr & (MSR_FP |,
    if (msr & MSR_FP) {
    pub tsk->thread.fpexc_mode)): regs_set_return_msr(regs, regs->msr | (MSR_FP |,
    }
    if (msr & MSR_VEC) {
    pub MSR_VEC): regs_set_return_msr(regs, regs->msr |,
    }
    pub err: return,
    }

    static long restore_tm_sigcontexts(struct task_struct *tsk, struct sigcontext __user *sc,
    struct sigcontext __user *tm_sc)
    {
    pub -EINVAL: return,
    }

//
// Setup the trampoline code on the stack
//
#[no_mangle]
unsafe extern "C" fn setup_trampoline(syscall: c_uint, tramp: *mut unsigned int __user) -> c_long {
    static long setup_trampoline(unsigned int syscall, unsigned int __user *tramp)
    {
    pub i: c_int,
    pub 0: long err =,
// Call the handler and pop the dummy stackframe
    pub &tramp[0]): err |= __put_user(PPC_RAW_BCTRL(),,
    pub &tramp[1]): err |= __put_user(PPC_RAW_ADDI(_R1, _R1, __SIGNAL_FRAMESIZE),,
    pub &tramp[2]): err |= __put_user(PPC_RAW_LI(_R0, syscall),,
    pub &tramp[3]): err |= __put_user(PPC_RAW_SC(),,
// Minimal traceback info
    pub ;i++): for (i=TRAMP_TRACEBACK; i < TRAMP_SIZE,
    pub &tramp[i]): err |= __put_user(0,,
    if (!err)
    flush_icache_range((unsigned long) &tramp[0],
    pub &tramp[TRAMP_SIZE]): (unsigned long),
    pub err: return,
    }
//
// Userspace code may pass a ucontext which doesn't include VSX added
// at the end.  We need to check for this case.
//

    (sizeof(struct ucontext) - 32*sizeof(long))
//
// Handle {get,set,swap}_context operations
//
    SYSCALL_DEFINE3(swapcontext, struct ucontext __user *, old_ctx,
    struct ucontext __user *, new_ctx, long, ctx_size)
    {
    pub set: sigset_t,
    pub 0: unsigned long new_msr =,
    pub 0: int ctx_has_vsx_region =,
    if (new_ctx &&
    get_user(new_msr, &new_ctx.uc_mcontext.gp_regs[PT_MSR]))
    pub -EFAULT: return,
//
// Check that the context is not smaller than the original
// size (with VMX but without VSX)
//
    if (ctx_size < UCONTEXTSIZEWITHOUTVSX)
    pub -EINVAL: return,
//
// If the new context state sets the MSR VSX bits but
// it doesn't provide VSX state.
//
    if ((ctx_size < sizeof(struct ucontext)) &&
    (new_msr & MSR_VSX))
    pub -EINVAL: return,
// Does the context have enough room to store VSX data?
    if (ctx_size >= sizeof(struct ucontext))
    pub 1: ctx_has_vsx_region =,
    if (old_ctx != core::ptr::null_mut()) {
    if (!user_write_access_begin(old_ctx, ctx_size))
    pub -EFAULT: return,
    unsafe_setup_sigcontext(&old_ctx.uc_mcontext, current, 0, core::ptr::null_mut(),
    pub efault_out): 0, ctx_has_vsx_region,,
    unsafe_copy_to_user(&old_ctx.uc_sigmask, &current.blocked,
    pub efault_out): sizeof(sigset_t),,
    }
    if (new_ctx == core::ptr::null_mut())
    pub 0: return,
    if (!access_ok(new_ctx, ctx_size) ||
    fault_in_readable((char __user *)new_ctx, ctx_size))
    pub -EFAULT: return,
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
    if (__get_user_sigset(&set, &new_ctx.uc_sigmask)) {
    pub -EFAULT: return,
    }
    if (!user_read_access_begin(new_ctx, ctx_size))
    pub -EFAULT: return,
    if (__unsafe_restore_sigcontext(current, core::ptr::null_mut(), 0, &new_ctx.uc_mcontext)) {
    pub -EFAULT: return,
    }
// This returns like rt_sigreturn
    pub 0: return,
    efault_out:
    pub -EFAULT: return,
    }
//
// Do a signal return; undo the signal stack.
//
    SYSCALL_DEFINE0(rt_sigreturn)
    {
    pub current_pt_regs(): *mut *mut pt_regs regs =,
    pub )regs->gpr[1]: *mut *mut ucontext __user uc = (ucontext __user,
    pub set: sigset_t,
    pub msr: c_ulong,
// Always make any pending restarted system calls return -EINTR
    pub do_no_restart_syscall: current->restart_block.fn =,
    if (!access_ok(uc, sizeof(*uc)))
    pub badframe: goto,
    if (__get_user_sigset(&set, &uc.uc_sigmask))
    pub badframe: goto,
    if (IS_ENABLED(CONFIG_PPC_TRANSACTIONAL_MEM)) {
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
//
// Disable MSR[TS] bit also, so, if there is an exception in the
// code below (as a page fault in copy_ckvsx_to_user()), it does
// not recheckpoint this task if there was a context switch inside
// the exception.
//
// A major page fault can indirectly call schedule(). A reschedule
// process in the middle of an exception can have a side effect
// (Changing the CPU MSR[TS] state), since schedule() is called
// with the CPU MSR[TS] disable and returns with MSR[TS]=Suspended
// (switch_to() calls tm_recheckpoint() for the 'new' process). In
// this case, the process continues to be the same in the CPU, but
// the CPU state just changed.
//
// This can cause a TM Bad Thing, since the MSR in the stack will
// have the MSR[TS]=0, and this is what will be used to RFID.
//
// Clearing MSR[TS] state here will avoid a recheckpoint if there
// is any process reschedule in kernel space. The MSR[TS] state
// does not need to be saved also, since it will be replaced with
// the MSR[TS] that came from user context later, at
// restore_tm_sigcontexts.
//
    pub ~MSR_TS_MASK): regs_set_return_msr(regs, regs->msr &,
    if (__get_user(msr, &uc.uc_mcontext.gp_regs[PT_MSR]))
    pub badframe: goto,
    }
    if (IS_ENABLED(CONFIG_PPC_TRANSACTIONAL_MEM) && MSR_TM_ACTIVE(msr)) {
// We recheckpoint on return.
    pub uc_transact: *mut ucontext __user,
// Trying to start TM on non TM system
    if (!cpu_has_feature(CPU_FTR_TM))
    pub badframe: goto,
    if (__get_user(uc_transact, &uc.uc_link))
    pub badframe: goto,
    if (restore_tm_sigcontexts(current, &uc.uc_mcontext,
    &uc_transact.uc_mcontext))
    pub badframe: goto,
    } else {
//
// Fall through, for non-TM restore
//
// Unset MSR[TS] on the thread regs since MSR from user
// context does not have MSR active, and recheckpoint was
// not called since restore_tm_sigcontexts() was not called
// also.
//
// If not unsetting it, the code can RFID to userspace with
// MSR[TS] set, but without CPU in the proper state,
// causing a TM bad thing.
//
    regs_set_return_msr(current.thread.regs,
    pub ~MSR_TS_MASK): current->thread.regs->msr &,
    if (!user_read_access_begin(&uc.uc_mcontext, sizeof(uc.uc_mcontext)))
    pub badframe: goto,
    unsafe_restore_sigcontext(current, core::ptr::null_mut(), 1, &uc.uc_mcontext,
    }
    if (restore_altstack(&uc.uc_stack))
    pub badframe: goto,
    pub 0: return,
    badframe_block:
    badframe:
    pub uc): signal_fault(current, regs, "rt_sigreturn",,
    pub 0: return,
    }
    int handle_rt_signal64(struct ksignal *ksig, sigset_t *set,
    struct task_struct *tsk)
    {
    pub frame: *mut rt_sigframe __user,
    pub 0: unsigned long newsp =,
    pub 0: long err =,
    pub tsk->thread.regs: *mut *mut pt_regs regs =,
// Save the thread's msr before get_tm_stackpointer() changes it
    pub regs->msr: unsigned long msr =,
    pub 0): *mut *mut frame = get_sigframe(ksig, tsk, sizeof(frame),,
//
// This only applies when calling unsafe_setup_sigcontext() and must be
// called before opening the uaccess window.
//
    if (!MSR_TM_ACTIVE(msr))
    if (!user_write_access_begin(frame, sizeof(*frame)))
    pub badframe: goto,
    pub badframe_block): unsafe_put_user(&frame->info, &frame->pinfo,,
    pub badframe_block): unsafe_put_user(&frame->uc, &frame->puc,,
// Create the ucontext.
    pub badframe_block): unsafe_put_user(0, &frame->uc.uc_flags,,
    pub badframe_block): unsafe_save_altstack(&frame->uc.uc_stack, regs->gpr[1],,
    if (MSR_TM_ACTIVE(msr)) {

// The ucontext_t passed to userland points to the second
// ucontext_t (for transactional state) with its uc_link ptr.
//
    pub badframe_block): unsafe_put_user(&frame->uc_transact, &frame->uc.uc_link,,
    err |= setup_tm_sigcontexts(&frame.uc.uc_mcontext,
    &frame.uc_transact.uc_mcontext,
    tsk, ksig.sig, core::ptr::null_mut(),
    (unsigned long)ksig.ka.sa.sa_handler,
    if (!user_write_access_begin(&frame.uc.uc_sigmask,
    sizeof(frame.uc.uc_sigmask)))
    pub badframe: goto,

    } else {
    pub badframe_block): unsafe_put_user(0, &frame->uc.uc_link,,
    unsafe_setup_sigcontext(&frame.uc.uc_mcontext, tsk, ksig.sig,
    core::ptr::null_mut(), (unsigned long)ksig.ka.sa.sa_handler,
    pub badframe_block): 1,,
    }
    pub badframe_block): *mut *mut unsafe_copy_to_user(&frame->uc.uc_sigmask, set, sizeof(set),,
// Save the siginfo outside of the unsafe block.
    if (copy_siginfo_to_user(&frame.info, &ksig.info))
    pub badframe: goto,
// Make sure signal handler doesn't get spurious FP exceptions
    pub 0: tsk->thread.fp_state.fpscr =,
// Set up to return from userspace.
    if (tsk.mm.context.vdso) {
    pub sigtramp_rt64)): regs_set_return_ip(regs, VDSO64_SYMBOL(tsk->mm->context.vdso,,
    } else {
    pub &frame->tramp[0]): err |= setup_trampoline(__NR_rt_sigreturn,,
    if (err)
    pub badframe: goto,
    pub &frame->tramp[0]): regs_set_return_ip(regs, (unsigned long),
    }
// Allocate a dummy caller frame for the signal handler.
    pub __SIGNAL_FRAMESIZE: newsp = ((unsigned long)frame) -,
    pub )newsp): *mut err |= put_user(regs->gpr[1], (unsigned long __user,
// Set up "regs" so we "return" to the signal handler.
    if (is_elf2_task()) {
    pub ksig->ka.sa.sa_handler: regs->ctr = (unsigned long),
    pub regs->ctr: regs->gpr[12] =,
    } else {
// Handler is *really* a pointer to the function descriptor for
// the signal routine.  The first entry in the function
// descriptor is the entry address of signal and the second
// entry is the TOC value we need to use.
//
    struct func_desc __user *ptr =
    pub )ksig->ka.sa.sa_handler: *mut (struct func_desc __user,
    pub &ptr->addr): err |= get_user(regs->ctr,,
    pub &ptr->toc): err |= get_user(regs->gpr[2],,
    }
// enter the signal handler in native-endian mode
    pub MSR_LE)): regs_set_return_msr(regs, (regs->msr & ~MSR_LE) | (MSR_KERNEL &,
    pub newsp: regs->gpr[1] =,
    pub ksig->sig: regs->gpr[3] =,
    pub 0: regs->result =,
    if (ksig.ka.sa.sa_flags & SA_SIGINFO) {
    pub long)&frame->info: regs->gpr[4] = (unsigned,
    pub long)&frame->uc: regs->gpr[5] = (unsigned,
    pub frame: regs->gpr[6] = (unsigned long),
    } else {
    pub long)&frame->uc.uc_mcontext: regs->gpr[4] = (unsigned,
    }
    if (err)
    pub badframe: goto,
    pub 0: return,
    badframe_block:
    badframe:
    pub frame): signal_fault(current, regs, "handle_rt_signal64",,
    pub 1: return,
    }
