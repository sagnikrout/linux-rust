//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/processor.h
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
// Copyright (C) 2001 PPC 64 Team, IBM Corp
//

pub const TS_FPRWIDTH: c_int = 2;

pub const TS_FPROFFSET: c_int = 0;
pub const TS_VSRLOWOFFSET: c_int = 1;

pub const TS_FPROFFSET: c_int = 1;
pub const TS_VSRLOWOFFSET: c_int = 0;

pub const TS_FPRWIDTH: c_int = 1;
pub const TS_FPROFFSET: c_int = 0;

// Default SMT priority is set to 3. Use 11- 13bits to save priority.
pub const PPR_PRIORITY: c_int = 3;

// We do _not_ want to define new machine types at all, those must die
// in favor of using the device-tree
// -- BenH.
//
// PREP sub-platform types. Unused
pub const _PREP_Motorola: c_uint = 0x01	/* motorola prep */;
pub const _PREP_Firm: c_uint = 0x02	/* firmworks prep */;
pub const _PREP_IBM: c_uint = 0x00	/* ibm prep */;
pub const _PREP_Bull: c_uint = 0x03	/* bull prep */;
// CHRP sub-platform types. These are arbitrary
pub const _CHRP_Motorola: c_uint = 0x04	/* motorola chrp, the cobra */;
pub const _CHRP_IBM: c_uint = 0x05	/* IBM chrp, the longtrail and longtrail 2 */;
pub const _CHRP_Pegasos: c_uint = 0x06	/* Genesi/bplan's Pegasos and Pegasos2 */;
pub const _CHRP_briq: c_uint = 0x07	/* TotalImpact's briQ */;

extern "C" {
    pub fn start_thread(regs: *mut pt_regs, fdptr: c_ulong, sp: c_ulong);
}

// FP and VSX 0-31 register set
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_fp_state {
    pub __attribute__((aligned(16))): u64 fpr[32][TS_FPRWIDTH],
    pub /: *mut *mut u64 fpscr; / Floating point status,
}

// Complete AltiVec register set including VSCR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_vr_state {
    pub __attribute__((aligned(16))): vector128 vr[32],
    pub __attribute__((aligned(16))): vector128 vscr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_reg {

//
// The following help to manage the use of Debug Control Registers
// om the BookE platforms.
//
    pub dbcr0: u32,
    pub dbcr1: u32,

    pub dbcr2: u32,

//
// The stored value of the DBSR register will be the value at the
// last debug interrupt. This register can only be read from the
// user (will never be written to) and has value while helping to
// describe the reason for the last debug trap.  Torez
//
    pub dbsr: u32,
//
// The following will contain addresses used by debug applications
// to help trace and trap on particular address locations.
// The bits in the Debug Control Registers above help define which
// of the following registers will contain valid data and/or addresses.
//
    pub iac1: c_ulong,
    pub iac2: c_ulong,

    pub iac3: c_ulong,
    pub iac4: c_ulong,

    pub dac1: c_ulong,
    pub dac2: c_ulong,

    pub dvc1: c_ulong,
    pub dvc2: c_ulong,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_struct {
    pub /: *mut *mut unsigned long ksp; / Kernel stack pointer,

    pub ksp_vsid: c_ulong,

    pub /: *mut *mut *mut pt_regs regs; / Pointer to saved register state,

// BookE base exception scratch space; align on cacheline
    pub ____cacheline_aligned: unsigned long normsave[8],

    pub /: *mut *mut *mut void pgdir; / root of page-table tree,

    pub /: *mut *mut unsigned long rtas_sp; / stack pointer for when in RTAS,

    pub /: *mut *mut unsigned long kuap; / opened segments for user access,

    pub srr0: c_ulong,
    pub srr1: c_ulong,
    pub dar: c_ulong,
    pub dsisr: c_ulong,

    pub r11: unsigned long r0, r3, r4, r5, r6, r8, r9,,
    pub ctr: unsigned long lr,,
    pub sr0: c_ulong,

    pub /: *mut *mut unsigned long pid; / value written in PID reg. at interrupt exit,

// Debug Registers
    pub debug: debug_reg,

    pub fp_state: thread_fp_state,
    pub fp_save_area: *mut thread_fp_state,

    pub /: *mut *mut int fpexc_mode; / floating-point exception mode,
    pub /: *mut *mut unsigned int align_ctl; / alignment handling control,
    pub ptrace_bps: [*mut perf_event; HBP_NUM_MAX],
    pub /: *mut *mut arch_hw_breakpoint hw_brk[HBP_NUM_MAX]; / hardware breakpoint info,
    pub /: *mut *mut unsigned long trap_nr; / last trap # on this thread,
    pub /: *mut *mut u8 load_slb; / Ages out SLB preload cache entries,
    pub load_fp: u8,

    pub load_vec: u8,
    pub vr_state: thread_vr_state,
    pub vr_save_area: *mut thread_vr_state,
    pub vrsave: c_ulong,
    pub /: *mut *mut int used_vr; / set if process has used altivec,

// VSR status
    pub /: *mut *mut int used_vsr; / set if process has used VSX,

    pub /: *mut *mut unsigned long evr[32]; / upper 32-bits of SPE regs,
    pub /: *mut *mut u64 acc; / Accumulator,
    pub /: *mut *mut unsigned long spefscr; / SPE & eFP status,
    pub prctl: *mut *mut unsigned long spefscr_last; / SPEFSCR value on last,
    pub /: *mut *mut int used_spe; / set if process has used spe,

    pub load_tm: u8,
    pub /: *mut *mut u64 tm_tfhar; / Transaction fail handler addr,
    pub /: *mut *mut u64 tm_texasr; / Transaction exception & summary,
    pub /: *mut *mut u64 tm_tfiar; / Transaction fail instr address reg,
    pub /: *mut *mut pt_regs ckpt_regs; / Checkpointed registers,
    pub tm_tar: c_ulong,
    pub tm_ppr: c_ulong,
    pub tm_dscr: c_ulong,
    pub tm_amr: c_ulong,
//
// Checkpointed FP and VSX 0-31 register set.
//
// When a transaction is active/signalled/scheduled etc., *regs is the
// most recent set of/speculated GPRs with ckpt_regs being the older
// checkpointed regs to which we roll back if transaction aborts.
//
// These are analogous to how ckpt_regs and pt_regs work
//
    pub /: *mut *mut thread_fp_state ckfp_state; / Checkpointed FP state,
    pub /: *mut *mut thread_vr_state ckvr_state; / Checkpointed VR state,
    pub /: *mut *mut unsigned long ckvrsave; / Checkpointed VRSAVE,

    pub /: *mut *mut *mut void kvm_shadow_vcpu; / KVM internal data,

    pub kvm_vcpu: *mut kvm_vcpu,

    pub dscr: c_ulong,
    pub fscr: c_ulong,
//
// This member element dscr_inherit indicates that the process
// has explicitly attempted and changed the DSCR register value
// for itself. Hence kernel wont use the default CPU DSCR value
// contained in the PACA structure anymore during process context
// switch. Once this variable is set, this behaviour will also be
// inherited to all the children of this process from that point
// onwards.
//
    pub dscr_inherit: c_int,
    pub tidr: c_ulong,

    pub tar: c_ulong,
    pub ebbrr: c_ulong,
    pub ebbhr: c_ulong,
    pub bescr: c_ulong,
    pub siar: c_ulong,
    pub sdar: c_ulong,
    pub sier: c_ulong,
    pub mmcr2: c_ulong,
    pub mmcr0: unsigned,
    pub used_ebb: unsigned,
    pub mmcr3: c_ulong,
    pub sier2: c_ulong,
    pub sier3: c_ulong,
    pub hashkeyr: c_ulong,
    pub dexcr: c_ulong,
    pub /: *mut *mut unsigned long dexcr_onexec; / Reset value to load on exec,

}

pub const ARCH_MIN_TASKALIGN: c_int = 16;

// Macro flag: #define SPEFSCR_INIT

// Macro flag: #define SR0_INIT

extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> c_ulong;
}

// Get/set floating-point exception mode

extern "C" {
    pub fn get_fpexc_mode(tsk: *mut task_struct, adr: c_ulong) -> c_int;
}
extern "C" {
    pub fn set_fpexc_mode(tsk: *mut task_struct, val: c_uint) -> c_int;
}

extern "C" {
    pub fn get_endian(tsk: *mut task_struct, adr: c_ulong) -> c_int;
}
extern "C" {
    pub fn set_endian(tsk: *mut task_struct, val: c_uint) -> c_int;
}

extern "C" {
    pub fn get_unalign_ctl(tsk: *mut task_struct, adr: c_ulong) -> c_int;
}
extern "C" {
    pub fn set_unalign_ctl(tsk: *mut task_struct, val: c_uint) -> c_int;
}

extern "C" {
    pub fn get_dexcr_prctl(tsk: *mut task_struct, asp: c_ulong) -> c_int;
}
extern "C" {
    pub fn set_dexcr_prctl(tsk: *mut task_struct, asp: c_ulong, val: c_ulong) -> c_int;
}

extern "C" {
    pub fn load_fp_state(fp: *mut thread_fp_state);
}
extern "C" {
    pub fn store_fp_state(fp: *mut thread_fp_state);
}
extern "C" {
    pub fn load_vr_state(vr: *mut thread_vr_state);
}
extern "C" {
    pub fn store_vr_state(vr: *mut thread_vr_state);
}

//
// Check that a certain kernel stack pointer is a valid (minimum sized)
// stack frame in task_struct p.
//
extern "C" {
    pub fn validate_sp(sp: c_ulong, p: *mut task_struct) -> c_int;
}
//
// validate the stack frame of a particular minimum size, used for when we are
// looking at a certain object in the stack beyond the minimum.
//
// Prefetch macros.
//
// Macro flag: #define ARCH_HAS_PREFETCH
// Macro flag: #define ARCH_HAS_PREFETCHW
extern "C" {
    pub fn __volatile__(0: "dcbt, (x): %0" : : "r") -> __asm__;
}
extern "C" {
    pub fn __volatile__(0: "dcbtst, (x): %0" : : "r") -> __asm__;
}
// asm stubs
extern "C" {
    pub fn isa300_idle_stop_noloss(psscr_val: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn isa300_idle_stop_mayloss(psscr_val: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn isa206_idle_insn_mayloss(type: c_ulong) -> c_ulong;
}

extern "C" {
    pub fn power4_idle_nap();
}
extern "C" {
    pub fn power4_idle_nap_return();
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idle_boot_override {

    extern int powersave_nap;	/* set if nap mode can be used in idle loop */

    extern void power7_idle_type(unsigned long type);
    extern void arch300_idle_type(unsigned long stop_psscr_val,
    unsigned long stop_psscr_mask);
    void pnv_power9_force_smt4_catch(void);
    void pnv_power9_force_smt4_release(void);

    extern int fix_alignment(struct pt_regs *);

//
// We handle most unaligned accesses in hardware. On the other hand
// unaligned DMA can be very expensive on some ppc64 IO chips (it does
// powers of 2 writes until it reaches sufficient alignment).
//
// Based on this we disable the IP header alignment in network drivers.
//
pub const NET_IP_ALIGN: c_int = 0;

    int do_mathemu(struct pt_regs *regs);
    int do_spe_mathemu(struct pt_regs *regs);
    int speround_handler(struct pt_regs *regs);

// VMX copying
    int enter_vmx_usercopy(void);
    int exit_vmx_usercopy(void);
    int enter_vmx_ops(void);
    void *exit_vmx_ops(void *dest);

