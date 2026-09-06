//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/hyp/include/hyp/sysreg-sr.h
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
// Copyright (C) 2012-2015 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//

extern "C" {
    pub fn ctxt_has_s1poe(ctxt: *mut kvm_cpu_context) -> bool;
}
extern "C" {
    pub fn read_cpuid_id() -> return;
}
extern "C" {
    pub fn kvm_read_vm_id_reg(_arg: kvm, _arg: SYS_MIDR_EL1) -> return;
}
// ctxt_mdscr_el1(ctxt)	= read_sysreg(mdscr_el1);
// POR_EL0 can affect uaccess, so must be saved/restored early.
extern "C" {
    pub fn kvm_has_mte(_arg: kern_hyp_va(vcpu->kvm)) -> return;
}
extern "C" {
    pub fn kvm_has_s1pie(_arg: kern_hyp_va(vcpu->kvm)) -> return;
}
extern "C" {
    pub fn kvm_has_tcr2(_arg: kern_hyp_va(vcpu->kvm)) -> return;
}
extern "C" {
    pub fn kvm_has_s1poe(_arg: kern_hyp_va(vcpu->kvm)) -> return;
}
extern "C" {
    pub fn kvm_has_ras(_arg: kern_hyp_va(vcpu->kvm)) -> return;
}
extern "C" {
    pub fn kvm_has_sctlr2(_arg: kern_hyp_va(vcpu->kvm)) -> return;
}
// Retrieve L2's HCR_EL2, and save it for future use
//
// Guest PSTATE gets saved at guest fixup time in all
// cases. We still need to handle the nVHE host side here.
//
// POR_EL0 can affect uaccess, so must be saved/restored early.
//
// Must only be done for guest registers, hence the context
// test. We're coming from the host, so SCTLR.M is already
// set. Pairs with nVHE's __activate_traps().
//
// Must only be done for host registers, hence the context
// test. Pairs with nVHE's __deactivate_traps().
//
// At this stage, and thanks to the above isb(), S2 is
// deconfigured and disabled. We can now restore the host's
// S1 configuration: SCTLR, and only then TCR.
//
// Publish the L2 view of HCR_EL2 to the HW if L1 is using NV3.
// Otherwise, the data is already in place in the L1's own VNCR.
//
// Read the VCPU state's PSTATE, but translate (v)EL2 to EL1.
//
// Safety check to ensure we're setting the CPU up to enter the guest
// in a less privileged mode.
//
// If we are attempting a return to EL2 or higher in AArch64 state,
// program SPSR_EL2 with M=EL2h and the IL bit set which ensures that
// we'll take an illegal exception state exception immediately after
// the ERET to the guest.  Attempts to return to AArch32 Hyp will
// result in an illegal exception return because EL2's execution state
// is determined by SCR_EL3.RW.
//
