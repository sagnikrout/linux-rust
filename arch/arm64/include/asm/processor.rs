//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/processor.h
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
// Based on arch/arm/include/asm/processor.h
//
// Copyright (C) 1995-1999 Russell King
// Copyright (C) 2012 ARM Ltd.
//
// On arm64 systems, unaligned accesses by the CPU are cheap, and so there is
// no point in shifting all network buffers by 2 bytes just to make some IP
// header fields appear aligned in memory, potentially sacrificing some DMA
// performance on some platforms.
//
pub const NET_IP_ALIGN: c_int = 0;
pub const MTE_CTRL_GCR_USER_EXCL_SHIFT: c_int = 0;
pub const MTE_CTRL_GCR_USER_EXCL_MASK: c_uint = 0xffff;

//
// TASK_SIZE - the maximum size of a user space task.
// TASK_UNMAPPED_BASE - the lower boundary of the mmap VM area.
//

//
// With CONFIG_ARM64_64K_PAGES enabled, the last page is occupied
// by the compat vectors page.
//

pub const AARCH32_VECTORS_BASE: c_uint = 0xffff0000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_info {

// Have we suspended stepping by a debugger?
    pub suspended_step: c_int,
// Allow breakpoints and watchpoints to be disabled for this thread.
    pub bps_disabled: c_int,
    pub wps_disabled: c_int,
// Hardware breakpoints pinned to this task.
    pub hbp_break: [*mut perf_event; ARM_MAX_BRP],
    pub hbp_watch: [*mut perf_event; ARM_MAX_WRP],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vec_type {
    ARM64_VEC_SVE = 0,
    ARM64_VEC_SME,
    ARM64_VEC_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fp_type {
    FP_STATE_CURRENT,	/* Save based on current task state. */
    FP_STATE_FPSIMD,
    FP_STATE_SVE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_context {
    pub x19: c_ulong,
    pub x20: c_ulong,
    pub x21: c_ulong,
    pub x22: c_ulong,
    pub x23: c_ulong,
    pub x24: c_ulong,
    pub x25: c_ulong,
    pub x26: c_ulong,
    pub x27: c_ulong,
    pub x28: c_ulong,
    pub fp: c_ulong,
    pub sp: c_ulong,
    pub pc: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_struct {
    pub /: *mut *mut cpu_context cpu_context; / cpu context,
//
// Whitelisted fields for hardened usercopy:
// Maintainers must ensure manually that this contains no
// implicit padding.
//
    pub /: *mut *mut unsigned long tp_value; / TLS register,
    pub tp2_value: c_ulong,
    pub fpmr: u64,
    pub pad: c_ulong,
    pub fpsimd_state: user_fpsimd_state,
    pub uw: },
    pub /: *mut *mut fp_type fp_type; / registers FPSIMD or SVE?,
    pub fpsimd_cpu: c_uint,
    pub /: *mut *mut *mut arm64_sve_state sve_state; / SVE registers, if any,
    pub /: *mut *mut *mut arm64_sme_state sme_state; / ZA and ZT state, if any,
    pub /: *mut *mut unsigned int vl[ARM64_VEC_MAX]; / vector length,
    pub /: *mut *mut unsigned int vl_onexec[ARM64_VEC_MAX]; / vl after next exec,
    pub /: *mut *mut unsigned long fault_address; / fault info,
    pub /: *mut *mut unsigned long fault_code; / ESR_EL1 value,
    pub /: *mut *mut debug_info debug; / debugging,
//
// Set [cleared] by kernel_neon_begin() [kernel_neon_end()] to the
// address of a caller provided buffer that will be used to preserve a
// task's kernel mode FPSIMD state while it is scheduled out.
//
    pub kernel_fpsimd_state: *mut user_fpsimd_state,
    pub kernel_fpsimd_cpu: c_uint,

    pub keys_user: ptrauth_keys_user,

    pub keys_kernel: ptrauth_keys_kernel,

    pub mte_ctrl: u64,

    pub sctlr_user: u64,
    pub svcr: u64,
    pub tpidr2_el0: u64,
    pub por_el0: u64,

    pub gcs_el0_mode: c_uint,
    pub gcs_el0_locked: c_uint,
    pub gcspr_el0: u64,
    pub gcs_base: u64,
    pub gcs_size: u64,

}

extern "C" {
    pub fn thread_get_vl(_arg: thread, _arg: ARM64_VEC_SVE) -> return;
}
extern "C" {
    pub fn thread_get_vl(_arg: thread, _arg: ARM64_VEC_SME) -> return;
}
extern "C" {
    pub fn thread_get_sme_vl(_arg: thread) -> return;
}
extern "C" {
    pub fn thread_get_sve_vl(_arg: thread) -> return;
}
extern "C" {
    pub fn task_get_vl(task: *const task_struct, type: vec_type) -> c_uint;
}
extern "C" {
    pub fn task_get_vl(_arg: task, _arg: ARM64_VEC_SVE) -> return;
}
extern "C" {
    pub fn task_get_vl(_arg: task, _arg: ARM64_VEC_SME) -> return;
}
extern "C" {
    pub fn task_get_vl_onexec(_arg: task, _arg: ARM64_VEC_SVE) -> return;
}

// Verify that there is no padding among the whitelisted fields:
// offset = offsetof(struct thread_struct, uw);
// size = sizeof_field(struct thread_struct, uw);

// Sync TPIDR_EL0 back to thread_struct for current
extern "C" {
    pub fn tls_preserve_current_state();
}

//
// Ensure all GPRs are zeroed, and initialize PC + PSTATE.
// The SP (or compat SP) will be initialized later.
//
// To allow the syscalls:sys_exit_execve tracepoint we need to preserve
// syscallno, but do not need orig_x0 or the original GPRs.
//
// An exec from a kernel thread won't have an existing PMR value.
//
// The pt_regs::stackframe field must remain valid throughout this
// function as a stacktrace can be taken at any time. Any user or
// kernel task should have a valid final frame.
//

// entry assembly clears tags for TTBR0 addrs
// TTBR1 addresses may have a tag if KASAN_SW_TAGS is in use
// Forward declaration, a strange C thing
extern "C" {
    pub fn __get_wchan(p: *mut task_struct) -> c_ulong;
}
extern "C" {
    pub fn update_sctlr_el1(sctlr: u64);
}
// Thread switching

//
// Prefetching support
//
// Macro flag: #define ARCH_HAS_PREFETCH
extern "C" {
    pub fn volatile(pldl1keep: "prfm, (ptr): %a0\n" : : "p") -> asm;
}
// Macro flag: #define ARCH_HAS_PREFETCHW
extern "C" {
    pub fn volatile(pstl1keep: "prfm, (ptr): %a0\n" : : "p") -> asm;
}
extern "C" {
    pub fn minsigstksz_setup() -> void __init;
}
//
// Not at the top of the file due to a direct #include cycle between
// <asm/fpsimd.h> and <asm/processor.h>.  Deferring this #include
// ensures that contents of processor.h are visible to fpsimd.h even if
// processor.h is included first.
//
// These prctl helpers are the only things in this file that require
// fpsimd.h.  The core code expects them to be in this header.
//

// Userspace interface for PR_S[MV]E_{SET,GET}_VL prctl()s:

// PR_PAC_RESET_KEYS prctl

// PR_PAC_{SET,GET}_ENABLED_KEYS prctl

// PR_{SET,GET}_TAGGED_ADDR_CTRL prctl
extern "C" {
    pub fn set_tagged_addr_ctrl(task: *mut task_struct, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn get_tagged_addr_ctrl(task: *mut task_struct) -> c_long;
}

extern "C" {
    pub fn get_tsc_mode(adr: c_ulong) -> c_int;
}
extern "C" {
    pub fn set_tsc_mode(val: c_uint) -> c_int;
}

