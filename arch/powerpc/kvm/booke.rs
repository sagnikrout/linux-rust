//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/kvm/booke.h
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
// Copyright IBM Corp. 2008
//
// Authors: Hollis Blanchard <hollisb@us.ibm.com>
//

// interrupt priortity ordering
pub const BOOKE_IRQPRIO_DATA_STORAGE: c_int = 0;
pub const BOOKE_IRQPRIO_INST_STORAGE: c_int = 1;
pub const BOOKE_IRQPRIO_ALIGNMENT: c_int = 2;
pub const BOOKE_IRQPRIO_PROGRAM: c_int = 3;
pub const BOOKE_IRQPRIO_FP_UNAVAIL: c_int = 4;

pub const BOOKE_IRQPRIO_SPE_UNAVAIL: c_int = 5;
pub const BOOKE_IRQPRIO_SPE_FP_DATA: c_int = 6;
pub const BOOKE_IRQPRIO_SPE_FP_ROUND: c_int = 7;

pub const BOOKE_IRQPRIO_ALTIVEC_UNAVAIL: c_int = 5;
pub const BOOKE_IRQPRIO_ALTIVEC_ASSIST: c_int = 6;

pub const BOOKE_IRQPRIO_SYSCALL: c_int = 8;
pub const BOOKE_IRQPRIO_AP_UNAVAIL: c_int = 9;
pub const BOOKE_IRQPRIO_DTLB_MISS: c_int = 10;
pub const BOOKE_IRQPRIO_ITLB_MISS: c_int = 11;
pub const BOOKE_IRQPRIO_MACHINE_CHECK: c_int = 12;
pub const BOOKE_IRQPRIO_DEBUG: c_int = 13;
pub const BOOKE_IRQPRIO_CRITICAL: c_int = 14;
pub const BOOKE_IRQPRIO_WATCHDOG: c_int = 15;
pub const BOOKE_IRQPRIO_EXTERNAL: c_int = 16;
pub const BOOKE_IRQPRIO_FIT: c_int = 17;
pub const BOOKE_IRQPRIO_DECREMENTER: c_int = 18;
pub const BOOKE_IRQPRIO_PERFORMANCE_MONITOR: c_int = 19;
// Internal pseudo-irqprio for level triggered externals
pub const BOOKE_IRQPRIO_EXTERNAL_LEVEL: c_int = 20;
pub const BOOKE_IRQPRIO_DBELL: c_int = 21;
pub const BOOKE_IRQPRIO_DBELL_CRIT: c_int = 22;
pub const BOOKE_IRQPRIO_MAX: c_int = 23;

extern "C" {
    pub fn kvmppc_set_msr(vcpu: *mut kvm_vcpu, new_msr: u32);
}
extern "C" {
    pub fn kvmppc_mmu_msr_notify(vcpu: *mut kvm_vcpu, old_msr: u32);
}
extern "C" {
    pub fn kvmppc_set_epcr(vcpu: *mut kvm_vcpu, new_epcr: u32);
}
extern "C" {
    pub fn kvmppc_set_tcr(vcpu: *mut kvm_vcpu, new_tcr: u32);
}
extern "C" {
    pub fn kvmppc_set_tsr_bits(vcpu: *mut kvm_vcpu, tsr_bits: u32);
}
extern "C" {
    pub fn kvmppc_clr_tsr_bits(vcpu: *mut kvm_vcpu, tsr_bits: u32);
}
extern "C" {
    pub fn kvmppc_booke_emulate_mfspr(vcpu: *mut kvm_vcpu, sprn: c_int, spr_val: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn kvmppc_booke_emulate_mtspr(vcpu: *mut kvm_vcpu, sprn: c_int, spr_val: c_ulong) -> c_int;
}
// low-level asm code to transfer guest state
extern "C" {
    pub fn kvmppc_load_guest_spe(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_save_guest_spe(vcpu: *mut kvm_vcpu);
}
// high-level function, manages flags, host state
extern "C" {
    pub fn kvmppc_vcpu_disable_spe(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_booke_vcpu_load(vcpu: *mut kvm_vcpu, cpu: c_int);
}
extern "C" {
    pub fn kvmppc_booke_vcpu_put(vcpu: *mut kvm_vcpu);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum int_class {
    INT_CLASS_NONCRIT,
    INT_CLASS_CRIT,
    INT_CLASS_MC,
    INT_CLASS_DBG,
}

extern "C" {
    pub fn kvmppc_set_pending_interrupt(vcpu: *mut kvm_vcpu, type: int_class);
}
extern "C" {
    pub fn kvmppc_handle_exit(vcpu: *mut kvm_vcpu, exit_nr: c_uint) -> c_int;
}
