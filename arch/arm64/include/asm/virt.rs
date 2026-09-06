//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/virt.h
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
// Author: Marc Zyngier <marc.zyngier@arm.com>
//
// The arm64 hcall implementation uses x0 to specify the hcall
// number. A value less than HVC_STUB_HCALL_NR indicates a special
// hcall, such as set vector. Any other value is handled in a
// hypervisor specific way.
//
// The hypercall is allowed to clobber any of the caller-saved
// registers (x0-x18), so it is advisable to use it through the
// indirection of a function call (as implemented in hyp-stub.S).
//
// HVC_SET_VECTORS - Set the value of the vbar_el2 register.
//
// @x1: Physical address of the new vector table.
//
pub const HVC_SET_VECTORS: c_int = 0;
//
// HVC_SOFT_RESTART - CPU soft reset, used by the cpu_soft_restart routine.
//
pub const HVC_SOFT_RESTART: c_int = 1;
//
// HVC_RESET_VECTORS - Restore the vectors to the original HYP stubs
//
pub const HVC_RESET_VECTORS: c_int = 2;
//
// HVC_FINALISE_EL2 - Upgrade the CPU from EL1 to EL2, if possible
//
pub const HVC_FINALISE_EL2: c_int = 3;
//
// HVC_GET_ICH_VTR_EL2 - Retrieve the ICH_VTR_EL2 value
//
pub const HVC_GET_ICH_VTR_EL2: c_int = 4;
// Max number of HYP stub hypercalls
pub const HVC_STUB_HCALL_NR: c_int = 5;
// Error returned when an invalid stub number is passed into x0
pub const HVC_STUB_ERR: c_uint = 0xbadca11;

//
// Flags returned together with the boot mode, but not preserved in
// __boot_cpu_mode. Used by the idreg override code to work out the
// boot state.
//

//
// __boot_cpu_mode records what mode CPUs were booted in.
// A correctly-implemented bootloader must start all CPUs in the same mode:
// In this case, both 32bit halves of __boot_cpu_mode will contain the
// same value (either BOOT_CPU_MODE_EL1 if booted in EL1, BOOT_CPU_MODE_EL2 if
// booted in EL2).
//
// Should the bootloader fail to do this, the two values will be different.
// This allows the kernel to flag an error when the secondaries have come up.
//

extern "C" {
    pub fn __hyp_set_vectors(phys_vector_base: phys_addr_t);
}
extern "C" {
    pub fn __hyp_reset_vectors();
}
extern "C" {
    pub fn is_kvm_arm_initialised() -> bool;
}

extern "C" {
    pub fn pkvm_force_reclaim_guest_page(phys: phys_addr_t) -> bool;
}

// Reports the availability of HYP mode
//
// If KVM protected mode is initialized, all CPUs must have been booted
// in EL2. Avoid checking __boot_cpu_mode as CPUs now come up in EL1.
//
// Check if the bootloader has booted CPUs in different modes
//
// If KVM protected mode is initialized, all CPUs must have been booted
// in EL2. Avoid checking __boot_cpu_mode as CPUs now come up in EL1.
//
// Code only run in VHE/NVHE hyp context can assume VHE is present or
// absent. Otherwise fall back to caps.
// This allows the compiler to discard VHE-specific code from the
// nVHE object, reducing the number of external symbol references
// needed to link.
//
extern "C" {
    pub fn cpus_have_final_cap(_arg: ARM64_HAS_VIRT_HOST_EXTN) -> return;
}
extern "C" {
    pub fn cpus_have_final_cap(_arg: ARM64_KVM_PROTECTED_MODE) -> return;
}
extern "C" {
    pub fn cpus_have_final_cap(_arg: ARM64_KVM_HVHE) -> return;
}
extern "C" {
    pub fn is_hyp_mode_available(!is_kernel_in_hyp_mode(: ) &&) -> return;
}

