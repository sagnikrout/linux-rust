//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kvm_asm.h
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

pub const ARM_EXIT_WITH_SERROR_BIT: c_int = 31;

pub const ARM_EXCEPTION_IRQ: c_int = 0;
pub const ARM_EXCEPTION_EL1_SERROR: c_int = 1;
pub const ARM_EXCEPTION_TRAP: c_int = 2;
pub const ARM_EXCEPTION_IL: c_int = 3;
// The hyp-stub will return this for any kvm_call_hyp() call

//
// Size of the HYP vectors preamble. kvm_patch_vector_branch() generates code
// that jumps over this.
//

pub const __KVM_HOST_SMCCC_FUNC___kvm_hyp_init: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum __kvm_host_smccc_func {
// Hypercalls that are unavailable once pKVM has finalised.
// __KVM_HOST_SMCCC_FUNC___kvm_hyp_init
    __KVM_HOST_SMCCC_FUNC___pkvm_init = __KVM_HOST_SMCCC_FUNC___kvm_hyp_init + 1,
    __KVM_HOST_SMCCC_FUNC___pkvm_create_private_mapping,
    __KVM_HOST_SMCCC_FUNC___pkvm_cpu_set_vector,
    __KVM_HOST_SMCCC_FUNC___kvm_enable_ssbs,
    __KVM_HOST_SMCCC_FUNC___vgic_v3_init_lrs,
    __KVM_HOST_SMCCC_FUNC___vgic_v3_get_gic_config,

    MARKER(__KVM_HOST_SMCCC_FUNC_MIN_PKVM),

    __KVM_HOST_SMCCC_FUNC___pkvm_prot_finalize,

// Hypercalls that are always available and common to [nh]VHE/pKVM.
    __KVM_HOST_SMCCC_FUNC___kvm_adjust_pc,
    __KVM_HOST_SMCCC_FUNC___kvm_vcpu_run,
    __KVM_HOST_SMCCC_FUNC___kvm_flush_vm_context,
    __KVM_HOST_SMCCC_FUNC___kvm_tlb_flush_vmid_ipa,
    __KVM_HOST_SMCCC_FUNC___kvm_tlb_flush_vmid_ipa_nsh,
    __KVM_HOST_SMCCC_FUNC___kvm_tlb_flush_vmid,
    __KVM_HOST_SMCCC_FUNC___kvm_tlb_flush_vmid_range,
    __KVM_HOST_SMCCC_FUNC___kvm_flush_cpu_context,
    __KVM_HOST_SMCCC_FUNC___kvm_timer_set_cntvoff,
    __KVM_HOST_SMCCC_FUNC___tracing_load,
    __KVM_HOST_SMCCC_FUNC___tracing_unload,
    __KVM_HOST_SMCCC_FUNC___tracing_enable,
    __KVM_HOST_SMCCC_FUNC___tracing_swap_reader,
    __KVM_HOST_SMCCC_FUNC___tracing_update_clock,
    __KVM_HOST_SMCCC_FUNC___tracing_reset,
    __KVM_HOST_SMCCC_FUNC___tracing_enable_event,
    __KVM_HOST_SMCCC_FUNC___tracing_write_event,
    __KVM_HOST_SMCCC_FUNC___vgic_v3_save_aprs,
    __KVM_HOST_SMCCC_FUNC___vgic_v3_restore_vmcr_aprs,
    __KVM_HOST_SMCCC_FUNC___vgic_v5_save_apr,
    __KVM_HOST_SMCCC_FUNC___vgic_v5_restore_vmcr_apr,

    MARKER(__KVM_HOST_SMCCC_FUNC_PKVM_ONLY),

// Hypercalls that are available only when pKVM has finalised.
    __KVM_HOST_SMCCC_FUNC___pkvm_host_share_hyp,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_unshare_hyp,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_donate_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_share_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_unshare_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_relax_perms_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_wrprotect_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_test_clear_young_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_host_mkyoung_guest,
    __KVM_HOST_SMCCC_FUNC___pkvm_reserve_vm,
    __KVM_HOST_SMCCC_FUNC___pkvm_unreserve_vm,
    __KVM_HOST_SMCCC_FUNC___pkvm_init_vm,
    __KVM_HOST_SMCCC_FUNC___pkvm_init_vcpu,
    __KVM_HOST_SMCCC_FUNC___pkvm_vcpu_in_poison_fault,
    __KVM_HOST_SMCCC_FUNC___pkvm_force_reclaim_guest_page,
    __KVM_HOST_SMCCC_FUNC___pkvm_reclaim_dying_guest_page,
    __KVM_HOST_SMCCC_FUNC___pkvm_start_teardown_vm,
    __KVM_HOST_SMCCC_FUNC___pkvm_finalize_teardown_vm,
    __KVM_HOST_SMCCC_FUNC___pkvm_vcpu_load,
    __KVM_HOST_SMCCC_FUNC___pkvm_vcpu_put,
    __KVM_HOST_SMCCC_FUNC___pkvm_vcpu_sync_state,
    __KVM_HOST_SMCCC_FUNC___pkvm_tlb_flush_vmid,

    MARKER(__KVM_HOST_SMCCC_FUNC_MAX)
}

//
// Define a pair of symbols sharing the same name but one defined in
// VHE and the other in nVHE hyp implementations.
//

//
// Compute pointer to a symbol defined in nVHE percpu region.
// Returns NULL if percpu memory has not been allocated yet.
//

// The nVHE hypervisor shouldn't even try to access VHE symbols

// The VHE hypervisor shouldn't even try to access nVHE symbols

//
// BIG FAT WARNINGS:
//
// - Don't be tempted to change the following is_kernel_in_hyp_mode()
// to has_vhe(). has_vhe() is implemented as a *final* capability,
// while this is used early at boot time, when the capabilities are
// not final yet....
//
// - Don't let the nVHE hypervisor have access to this, as it will
// pick the *wrong* symbol (yes, it runs at EL2...).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_nvhe_init_params {
    pub mair_el2: c_ulong,
    pub tcr_el2: c_ulong,
    pub tpidr_el2: c_ulong,
    pub stack_hyp_va: c_ulong,
    pub stack_pa: c_ulong,
    pub pgd_pa: phys_addr_t,
    pub hcr_el2: c_ulong,
    pub vttbr: c_ulong,
    pub vtcr: c_ulong,
}

//
// Used by the host in EL1 to dump the nVHE hypervisor backtrace on
// hyp_panic() in non-protected mode.
//
// @stack_base:                 hyp VA of the hyp_stack base.
// @overflow_stack_base:        hyp VA of the hyp_overflow_stack base.
// @fp:                         hyp FP where the backtrace begins.
// @pc:                         hyp PC where the backtrace begins.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_nvhe_stacktrace_info {
    pub stack_base: c_ulong,
    pub overflow_stack_base: c_ulong,
    pub fp: c_ulong,
    pub pc: c_ulong,
}

// Translate a kernel address @ptr into its equivalent linear mapping

extern "C" {
    pub fn __kvm_flush_vm_context();
}
extern "C" {
    pub fn __kvm_flush_cpu_context(mmu: *mut kvm_s2_mmu);
}
extern "C" {
    pub fn __kvm_tlb_flush_vmid(mmu: *mut kvm_s2_mmu);
}
extern "C" {
    pub fn __kvm_tlbi_s1e2(mmu: *mut kvm_s2_mmu, va: u64, sys_encoding: u64) -> c_int;
}
extern "C" {
    pub fn __kvm_timer_set_cntvoff(cntvoff: u64);
}
extern "C" {
    pub fn __kvm_at_s1e01(vcpu: *mut kvm_vcpu, op: u32, vaddr: u64) -> c_int;
}
extern "C" {
    pub fn __kvm_at_s1e2(vcpu: *mut kvm_vcpu, op: u32, vaddr: u64) -> c_int;
}
extern "C" {
    pub fn __kvm_at_s12(vcpu: *mut kvm_vcpu, op: u32, vaddr: u64) -> c_int;
}
extern "C" {
    pub fn __kvm_vcpu_run(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn __kvm_adjust_pc(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn __vgic_v3_get_gic_config() -> bool;
}
extern "C" {
    pub fn __vgic_v3_init_lrs();
}

extern "C" {
    pub fn hyp_panic() -> void __noreturn;
}
extern "C" {
    pub fn kvm_unexpected_el2_exception() -> asmlinkage void;
}
extern "C" {
    pub fn hyp_panic() -> asmlinkage void __noreturn;
}
extern "C" {
    pub fn hyp_panic_bad_stack() -> asmlinkage void __noreturn;
}
extern "C" {
    pub fn kvm_unexpected_el2_exception() -> asmlinkage void;
}
extern "C" {
    pub fn handle_trap(host_ctxt: *mut kvm_cpu_context);
}
extern "C" {
    pub fn __kvm_host_psci_cpu_on_entry() -> asmlinkage void __noreturn;
}
extern "C" {
    pub fn __kvm_host_psci_cpu_resume_entry() -> asmlinkage void __noreturn;
}
extern "C" {
    pub fn __pkvm_init_finalise() -> void __noreturn;
}
extern "C" {
    pub fn kvm_nvhe_prepare_backtrace(fp: c_ulong, pc: c_ulong);
}

//
// KVM extable for unexpected exceptions.
// Create a struct kvm_exception_table_entry output to a section that can be
// mapped by EL2. The table is not sorted.
//
// The caller must ensure:
// x18 has the hypervisor value to allow any Shadow-Call-Stack instrumented
// code to write to it, and that SPSR_EL2 and ELR_EL2 are restored by the fixup.
//

//
// We treat x18 as callee-saved as the host may use it as a platform
// register (e.g. for shadow call stack).
//
// We require \ctxt is not x18-x28

