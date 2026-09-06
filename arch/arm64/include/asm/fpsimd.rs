//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/fpsimd.h
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
// Copyright (C) 2012 ARM Ltd.
//

// Masks for extracting the FPSR and FPCR from the FPSCR
pub const VFP_FPSCR_STAT_MASK: c_uint = 0xf800009f;
pub const VFP_FPSCR_CTRL_MASK: c_uint = 0x07f79f00;
//
// The VFP state has 32x64-bit registers and a single 32-bit
// control/status register.
//

//
// When we defined the maximum SVE vector length we defined the ABI so
// that the maximum vector length included all the reserved for future
// expansion bits in ZCR rather than those just currently defined by
// the architecture.  Using this length to allocate worst size buffers
// results in excessively large allocations, and this effect is even
// more pronounced for SME due to ZA.  Define more suitable VLs for
// these situations.
//

extern "C" {
    pub fn fpsimd_thread_switch(next: *mut task_struct);
}
extern "C" {
    pub fn fpsimd_flush_thread();
}
extern "C" {
    pub fn fpsimd_preserve_current_state();
}
extern "C" {
    pub fn fpsimd_restore_current_state();
}
extern "C" {
    pub fn fpsimd_update_current_state(state: *const user_fpsimd_state);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_fp_state {
    pub st: *mut user_fpsimd_state,
    pub sve_state: *mut arm64_sve_state,
    pub sme_state: *mut arm64_sme_state,
    pub svcr: *mut u64,
    pub fpmr: *mut u64,
    pub sve_vl: c_uint,
    pub sme_vl: c_uint,
    pub fp_type: *mut fp_type,
    pub to_save: fp_type,
}

extern "C" {
    pub fn fpsimd_bind_state_to_cpu(fp_state: *mut cpu_fp_state);
}
extern "C" {
    pub fn fpsimd_flush_task_state(target: *mut task_struct);
}
extern "C" {
    pub fn fpsimd_save_and_flush_current_state();
}
extern "C" {
    pub fn fpsimd_save_and_flush_cpu_state();
}
extern "C" {
    pub fn system_supports_sme(SVCR_SM_MASK: ) && (thread->svcr &) -> return;
}
extern "C" {
    pub fn system_supports_sme(SVCR_ZA_MASK: ) && (thread->svcr &) -> return;
}
extern "C" {
    pub fn task_smstop_sm(task: *mut task_struct);
}
// Maximum VL that SVE/SME VL-agnostic software can transparently support
pub const VL_ARCH_MAX: c_uint = 0x100;
// The ZT register state is stored immediately after the ZA state

//
// Zero all SVE registers except for the first 128 bits of each vector.
//
// The caller must ensure that the VL has been configured and the CPU must be
// in non-streaming mode.
//
extern "C" {
    pub fn cpu_enable_fpsimd(__unused: *const arm64_cpu_capabilities);
}
extern "C" {
    pub fn cpu_enable_sve(__unused: *const arm64_cpu_capabilities);
}
extern "C" {
    pub fn cpu_enable_sme(__unused: *const arm64_cpu_capabilities);
}
extern "C" {
    pub fn cpu_enable_sme2(__unused: *const arm64_cpu_capabilities);
}
extern "C" {
    pub fn cpu_enable_fa64(__unused: *const arm64_cpu_capabilities);
}
extern "C" {
    pub fn cpu_enable_fpmr(__unused: *const arm64_cpu_capabilities);
}
//
// Helpers to translate bit indices in sve_vq_map to VQ values (and
// vice versa).  This allows find_next_bit() to be used to find the
// _maximum_ VQ not exceeding a certain value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vl_info {
    pub type: vec_type,
    pub /: *const *const *const char name; / For display purposes,
// Minimum supported vector length across all CPUs
    pub min_vl: c_int,
// Maximum supported vector length across all CPUs
    pub max_vl: c_int,
    pub max_virtualisable_vl: c_int,
//
// Set of available vector lengths,
// where length vq encoded as bit __vq_to_bit(vq):
//
    pub SVE_VQ_MAX): DECLARE_BITMAP(vq_map,,
// Set of vector lengths present on at least one cpu:
    pub SVE_VQ_MAX): DECLARE_BITMAP(vq_partial_map,,
}

extern "C" {
    pub fn sve_alloc(task: *mut task_struct, flush: bool);
}
extern "C" {
    pub fn fpsimd_release_task(task: *mut task_struct);
}
extern "C" {
    pub fn fpsimd_sync_from_effective_state(task: *mut task_struct);
}
extern "C" {
    pub fn fpsimd_sync_to_effective_state_zeropad(task: *mut task_struct);
}
extern "C" {
    pub fn sve_set_current_vl(arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn sve_get_current_vl() -> c_int;
}

//
// Probing and setup functions.
// Calls to these functions must be serialised with one another.
//
extern "C" {
    pub fn vec_init_vq_map(type: vec_type) -> void __init;
}
extern "C" {
    pub fn vec_update_vq_map(type: vec_type);
}
extern "C" {
    pub fn vec_verify_vq_map(type: vec_type) -> c_int;
}
extern "C" {
    pub fn sve_setup() -> void __init;
}

extern "C" {
    pub fn vec_max_vl(_arg: ARM64_VEC_SVE) -> return;
}
extern "C" {
    pub fn vec_max_virtualisable_vl(_arg: ARM64_VEC_SVE) -> return;
}
// Ensure vq >= SVE_VQ_MIN && vq <= SVE_VQ_MAX before calling this function
extern "C" {
    pub fn test_bit(_arg: __vq_to_bit(vq), _arg: vl_info[type].vq_map) -> return;
}
extern "C" {
    pub fn vq_available(_arg: ARM64_VEC_SVE, _arg: vq) -> return;
}
extern "C" {
    pub fn SVE_SIG_REGS_SIZE(_arg: sve_vq_from_vl(vl)) -> return;
}
//
// Return how many bytes of memory are required to store the full SVE
// state for task, given task's currently configured vector length.
//
extern "C" {
    pub fn __sve_state_size(_arg: sve_vl, _arg: sme_vl) -> return;
}

extern "C" {
    pub fn volatile(_arg: __msr_s(SYS_SVCR_SMSTART_SM_EL0, _arg: "xzr")) -> asm;
}
extern "C" {
    pub fn volatile(_arg: __msr_s(SYS_SVCR_SMSTOP_SM_EL0, _arg: "xzr")) -> asm;
}
extern "C" {
    pub fn volatile(_arg: __msr_s(SYS_SVCR_SMSTOP_SMZA_EL0, _arg: "xzr")) -> asm;
}
extern "C" {
    pub fn sme_setup() -> void __init;
}
extern "C" {
    pub fn vec_max_vl(_arg: ARM64_VEC_SME) -> return;
}
extern "C" {
    pub fn vec_max_virtualisable_vl(_arg: ARM64_VEC_SME) -> return;
}
extern "C" {
    pub fn sme_alloc(task: *mut task_struct, flush: bool);
}
extern "C" {
    pub fn sme_set_current_vl(arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn sme_get_current_vl() -> c_int;
}
extern "C" {
    pub fn sme_suspend_exit();
}
//
// The <Wv> argument to LDR/STR (array vector) can only encode W12-W15.
// The "Ucj" constraint exists for this, but is only supported by GCC
// 14.1.0+ and LLVM 18.1.0+.
//
extern "C" {
    pub fn asm(_arg: "w12") -> register unsigned int v;
}
// See comment in __sme_save_za
extern "C" {
    pub fn asm(_arg: "w12") -> register unsigned int v;
}
//
// STR ZT0, [<Xn|SP>]
// Supported by binutils 2.41+.
// Supported by LLVM 16+
//
// LDR ZT0, [<Xn|SP>]
// Supported by binutils 2.41+.
// Supported by LLVM 16+
//
// Return how many bytes of memory are required to store the full SME
// specific state for task, given task's currently configured vector
// length.
//
extern "C" {
    pub fn __sme_state_size(_arg: task_get_sme_vl(task)) -> return;
}
extern "C" {
    pub fn sme_enable_dvmsync();
}
extern "C" {
    pub fn sme_set_active();
}
extern "C" {
    pub fn sme_clear_active();
}

// For use by EFI runtime services calls only
extern "C" {
    pub fn __efi_fpsimd_begin();
}
extern "C" {
    pub fn __efi_fpsimd_end();
}

