//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kvm_emulate.h
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
// Copyright (C) 2012,2013 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//
// Derived from arch/arm/include/kvm_emulate.h
// Copyright (C) 2012 - Virtual Open Systems and Columbia University
// Author: Christoffer Dall <c.dall@virtualopensystems.com>
//

pub const CURRENT_EL_SP_EL0_VECTOR: c_uint = 0x0;
pub const CURRENT_EL_SP_ELx_VECTOR: c_uint = 0x200;
pub const LOWER_EL_AArch64_VECTOR: c_uint = 0x400;
pub const LOWER_EL_AArch32_VECTOR: c_uint = 0x600;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum exception_type {
    except_type_sync	= 0,
    except_type_irq		= 0x80,
    except_type_fiq		= 0x100,
    except_type_serror	= 0x180,
}

extern "C" {
    pub fn kvm_condition_valid32(vcpu: *const kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_skip_instr32(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_inject_undefined(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_inject_sync(vcpu: *mut kvm_vcpu, esr: u64);
}
extern "C" {
    pub fn kvm_inject_serror_esr(vcpu: *mut kvm_vcpu, esr: u64) -> c_int;
}
extern "C" {
    pub fn kvm_inject_sea(vcpu: *mut kvm_vcpu, iabt: bool, addr: u64) -> c_int;
}
extern "C" {
    pub fn kvm_inject_dabt_excl_atomic(vcpu: *mut kvm_vcpu, addr: u64) -> c_int;
}
extern "C" {
    pub fn kvm_inject_size_fault(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_inject_sea(_arg: vcpu, _arg: false, _arg: addr) -> return;
}
extern "C" {
    pub fn kvm_inject_sea(_arg: vcpu, _arg: true, _arg: addr) -> return;
}
//
// ESR_ELx.ISV (later renamed to IDS) indicates whether or not
// ESR_ELx.ISS contains IMPLEMENTATION DEFINED syndrome information.
//
// Set the bit when injecting an SError w/o an ESR to indicate ISS
// does not follow the architected format.
//
extern "C" {
    pub fn kvm_inject_serror_esr(_arg: vcpu, _arg: ESR_ELx_ISV) -> return;
}
extern "C" {
    pub fn kvm_vcpu_wfi(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_emulate_nested_eret(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_inject_nested_sync(vcpu: *mut kvm_vcpu, esr_el2: u64) -> c_int;
}
extern "C" {
    pub fn kvm_inject_nested_irq(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_inject_nested_sea(vcpu: *mut kvm_vcpu, iabt: bool, addr: u64) -> c_int;
}
extern "C" {
    pub fn kvm_inject_nested_serror(vcpu: *mut kvm_vcpu, esr: u64) -> c_int;
}

extern "C" {
    pub fn vcpu_has_feature(_arg: vcpu, _arg: KVM_ARM_VCPU_EL1_32BIT) -> return;
}

//
// For non-FWB CPUs, we trap VM ops (HCR_EL2.TVM) until M+C
// get set in SCTLR_EL1 such that we can detect when the guest
// MMU gets turned on and do the necessary cache maintenance
// then.
//
extern "C" {
    pub fn kvm_condition_valid32(_arg: vcpu) -> return;
}
// vcpu_cpsr(vcpu) |= PSR_AA32_T_BIT;
//
// vcpu_get_reg and vcpu_set_reg should always be passed a register number
// coming from a read of ESR_EL2. Otherwise, it may give the wrong result on
// AArch32 with banked registers.
//
extern "C" {
    pub fn vcpu_is_el2_ctxt(_arg: &vcpu->arch.ctxt) -> return;
}
//
// DDI0487L.b Known Issue D22105
//
// When executing at EL2 and HCR_EL2.{E2H,TGE} = {1, 0} it is
// IMPLEMENTATION DEFINED whether the effective value of HCR_EL2.AMO
// is the value programmed or 1.
//
// Make the implementation choice of treating the effective value as 1 as
// we cannot subsequently catch changes to TGE or AMO that would
// otherwise lead to the SError becoming deliverable.
//
// We are in a hypervisor context if the vcpu mode is EL2 or
// E2H and TGE bits are set. The latter means we are in the user space
// of the VHE kernel. ARMv8.1 ARM describes this as 'InHost'
//
// Note that the HCR_EL2.{E2H,TGE}={0,1} isn't really handled in the
// rest of the KVM code, and will result in a misbehaving guest.
//
extern "C" {
    pub fn is_hyp_ctxt(!vcpu_is_el2(vcpu: vcpu) &&) -> return;
}
extern "C" {
    pub fn vcpu_has_nv(!is_hyp_ctxt(vcpu: vcpu) &&) -> return;
}
//
// The layout of SPSR for an AArch32 state is different when observed from an
// AArch64 SPSR_ELx or an AArch32 SPSR_*. This function generates the AArch32
// view given an AArch64 view.
//
// In ARM DDI 0487E.a see:
//
// - The AArch64 view (SPSR_EL2) in section C5.2.18, page C5-426
// - The AArch32 view (SPSR_abt) in section G8.2.126, page G8-6256
// - The AArch32 view (SPSR_und) in section G8.2.132, page G8-6280
//
// Which show the following differences:
//
// | Bit | AA64 | AA32 | Notes                       |
// +-----+------+------+-----------------------------|
// | 24  | DIT  | J    | J is RES0 in ARMv8          |
// | 21  | SS   | DIT  | SS doesn't exist in AArch32 |
//
// ... and all other bits are (currently) common.
//
extern "C" {
    pub fn kvm_vcpu_get_esr(ESR_ELx_FSC: vcpu) & (ESR_ELx_CM | ESR_ELx_WNR |) -> return;
}
// Always check for S1PTW *before* using this.
// This one is not specific to Data Abort
extern "C" {
    pub fn ESR_ELx_EC(_arg: kvm_vcpu_get_esr(vcpu)) -> return;
}
extern "C" {
    pub fn kvm_vcpu_trap_is_iabt(!kvm_vcpu_abt_iss1tw(vcpu: vcpu) &&) -> return;
}
extern "C" {
    pub fn esr_fsc_is_permission_fault(_arg: kvm_vcpu_get_esr(vcpu)) -> return;
}
extern "C" {
    pub fn esr_fsc_is_translation_fault(_arg: kvm_vcpu_get_esr(vcpu)) -> return;
}
extern "C" {
    pub fn BIT(ESR_ELx_FSC_LEVEL): ARM64_HW_PGTABLE_LEVEL_SHIFT(esr &) -> return;
}
extern "C" {
    pub fn ESR_ELx_SYS64_ISS_RT(_arg: esr) -> return;
}
//
// Only a permission fault on a S1PTW should be
// considered as a write. Otherwise, page tables baked
// in a read-only memslot will result in an exception
// being delivered in the guest.
//
// The drawback is that we end-up faulting twice if the
// guest is using any of HW AF/DB: a translation fault
// to map the page containing the PT (read only at
// first), then a permission fault to allow the flags
// to be set.
//
extern "C" {
    pub fn kvm_vcpu_trap_is_permission_fault(_arg: vcpu) -> return;
}
extern "C" {
    pub fn kvm_vcpu_dabt_iswrite(_arg: vcpu) -> return;
}
// In nVHE hyp code, registers are always in memory: use the raw accessors.

// vcpu_cpsr(vcpu) |= PSR_AA32_E_BIT;
extern "C" {
    pub fn be16_to_cpu(0xffff: data &) -> return;
}
extern "C" {
    pub fn be32_to_cpu(0xffffffff: data &) -> return;
}
extern "C" {
    pub fn be64_to_cpu(_arg: data) -> return;
}
extern "C" {
    pub fn le16_to_cpu(0xffff: data &) -> return;
}
extern "C" {
    pub fn le32_to_cpu(0xffffffff: data &) -> return;
}
extern "C" {
    pub fn le64_to_cpu(_arg: data) -> return;
}
extern "C" {
    pub fn cpu_to_be16(0xffff: data &) -> return;
}
extern "C" {
    pub fn cpu_to_be32(0xffffffff: data &) -> return;
}
extern "C" {
    pub fn cpu_to_be64(_arg: data) -> return;
}
extern "C" {
    pub fn cpu_to_le16(0xffff: data &) -> return;
}
extern "C" {
    pub fn cpu_to_le32(0xffffffff: data &) -> return;
}
extern "C" {
    pub fn cpu_to_le64(_arg: data) -> return;
}

//
// Returns a 'sanitised' view of CPTR_EL2, translating from nVHE to the VHE
// format if E2H isn't set.
//
extern "C" {
    pub fn vcpu_el2_tge_is_set(!vcpu_is_el2(vcpu: vcpu) &&) -> return;
}

extern "C" {
    pub fn __guest_hyp_cptr_xen_trap_enabled(_arg: vcpu, _arg: FPEN) -> return;
}
extern "C" {
    pub fn __guest_hyp_cptr_xen_trap_enabled(_arg: vcpu, _arg: ZEN) -> return;
}
//
// In general, all HCRX_EL2 bits are gated by a feature.
// The only reason we can set SMPME without checking any
// feature is that its effects are not directly observable
// from the guest.
//
// NV3 is a host-specific extension, and we always use
// it when present and that the guest uses NV. It may
// be hidden from the guest though.
//
// If the guest is NV2-capable, then we need to see
// all the TLBIs, as configured in HCR_EL2.
// Otherwise, relax the TLBI traps to only TGE=0.
//
// Reset a vcpu's core registers.
// Reset core registers
// PSCI reset handling for a vcpu.
// Gracefully handle Thumb2 entry point
// Propagate caller endianness
// vcpu_pc(vcpu) = target_pc;
//
// We may come from a state where either a PC update was
// pending (SMC call resulting in PC being increpented to
// skip the SMC) or a pending exception. Make sure we get
// rid of all that, as this cannot be valid out of reset.
//
// Note that clearing the exception mask also clears PC
// updates, but that's an implementation detail, and we
// really want to make it explicit.
//
