//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kvm_mmu.h
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

//
// As ARMv8.0 only has the TTBR0_EL2 register, we cannot express
// "negative" addresses. This makes it impossible to directly share
// mappings with the kernel.
//
// Instead, give the HYP mode its own VA region at a fixed offset from
// the kernel by just masking the top bits (which are all ones for a
// kernel address). We need to find out how many bits to mask.
//
// We want to build a set of page tables that cover both parts of the
// idmap (the trampoline page used to initialize EL2), and our normal
// runtime VA space, at the same time.
//
// Given that the kernel uses VA_BITS for its entire address space,
// and that half of that space (VA_BITS - 1) is used for the linear
// mapping, we can also limit the EL2 space to (VA_BITS - 1).
//
// The main question is "Within the VA_BITS space, does EL2 use the
// top or the bottom half of that space to shadow the kernel's linear
// mapping?". As we need to idmap the trampoline page, this is
// determined by the range in which this page lives.
//
// If the page is in the bottom half, we have to use the top half. If
// the page is in the top half, we have to use the bottom half:
//
// T = __pa_symbol(__hyp_idmap_text_start)
// if (T & BIT(VA_BITS - 1))
// HYP_VA_MIN = 0  //idmap in upper half
// else
// HYP_VA_MIN = 1 << (VA_BITS - 1)
// HYP_VA_MAX = HYP_VA_MIN + (1 << (VA_BITS - 1)) - 1
//
// When using VHE, there are no separate hyp mappings and all KVM
// functionality is already mapped as part of the main kernel
// mappings, and none of this applies in that case.
//

//
// Convert a hypervisor VA to a PA
// reg: hypervisor address to be converted in place
// tmp: temporary register
//
// Convert a hypervisor VA to a kernel image address
// reg: hypervisor address to be converted in place
// tmp: temporary register
//
// The actual code generation takes place in kvm_get_kimage_voffset, and
// the instructions below are only there to reserve the space and
// perform the register allocation (kvm_get_kimage_voffset uses the
// specific registers encoded in the instructions).
//
// Convert hyp VA -> PA.
// Load kimage_voffset.
// Convert PA -> kimg VA.

extern "C" {
    pub fn kvm_compute_layout();
}
extern "C" {
    pub fn kvm_hyp_va_bits() -> u32;
}
extern "C" {
    pub fn kvm_apply_hyp_relocations();
}

//
// Convert a kernel VA into a HYP VA.
//
// Can be called from hyp or non-hyp context.
//
// The actual code generation takes place in kvm_update_va_mask(), and
// the instructions below are only there to reserve the space and
// perform the register allocation (kvm_update_va_mask() uses the
// specific registers encoded in the instructions).
//
// This #ifndef is an optimisation for when this is called from VHE hyp
// context.  When called from a VHE non-hyp context, kvm_update_va_mask() will
// replace the instructions with `nop`s.
//

//
// We currently support using a VM-specified IPA size. For backward
// compatibility, the default IPA size is fixed to 40bits.
//

extern "C" {
    pub fn kvm_share_hyp(from: *mut c_void, to: *mut c_void) -> c_int;
}
extern "C" {
    pub fn kvm_unshare_hyp(from: *mut c_void, to: *mut c_void);
}
extern "C" {
    pub fn create_hyp_mappings(from: *mut c_void, to: *mut c_void, prot: kvm_pgtable_prot) -> c_int;
}
extern "C" {
    pub fn hyp_alloc_private_va_range(size: usize, haddr: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn create_hyp_stack(phys_addr: phys_addr_t, haddr: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn free_hyp_pgds() -> void __init;
}
extern "C" {
    pub fn kvm_stage2_flush_range(mmu: *mut kvm_s2_mmu, addr: phys_addr_t, end: phys_addr_t);
}
extern "C" {
    pub fn kvm_stage2_wp_range(mmu: *mut kvm_s2_mmu, addr: phys_addr_t, end: phys_addr_t);
}
extern "C" {
    pub fn stage2_unmap_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_init_stage2_mmu(kvm: *mut kvm, mmu: *mut kvm_s2_mmu, type: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_uninit_stage2_mmu(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_free_stage2_pgd(mmu: *mut kvm_s2_mmu);
}
extern "C" {
    pub fn kvm_handle_guest_sea(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_guest_abort(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_mmu_get_httbr() -> phys_addr_t;
}
extern "C" {
    pub fn kvm_get_idmap_vector() -> phys_addr_t;
}
extern "C" {
    pub fn kvm_mmu_init(hyp_va_bits: u32) -> int __init;
}

//
// With FWB, we ensure that the guest always accesses memory using
// cacheable attributes, and we don't have to clean to PoC when
// faulting in pages. Furthermore, FWB implies IDC, so cleaning to
// PoU is not required either in this case.
//
// Blow the whole I-cache if it is aliasing (i.e. VIPT) or the
// invalidation range exceeds our arbitrary limit on invadations by
// cache line.
//
extern "C" {
    pub fn kvm_set_way_flush(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_toggle_cache(vcpu: *mut kvm_vcpu, was_enabled: bool);
}
extern "C" {
    pub fn get_vmid_bits(_arg: reg) -> return;
}
//
// We are not in the kvm->srcu critical section most of the time, so we take
// the SRCU read lock here. Since we copy the data from the user page, we
// can immediately drop the lock again.
//

//
// When this is (directly or indirectly) used on the TLB invalidation
// path, we rely on a previously issued DSB so that page table updates
// and VMID reads are correctly ordered.
//
// Must be called from hyp code running at EL2 with an updated VTTBR
// and interrupts disabled.
//
// ARM errata 1165522 and 1530923 require the actual execution of the
// above before we can switch to the EL1/EL0 translation regime used by
// the guest.
//
extern "C" {
    pub fn container_of(_arg: mmu->arch, kvm: struct, _arg: arch) -> return;
}
//
// Be careful, mmu may not be fully initialised so do look at
// *any* of its fields.
//
// ARM64 KVM relies on a simple conversion from physaddr to a kernel
// virtual address (KVA) when it does cache maintenance as the CMO
// instructions work on virtual addresses. This is incompatible with
// VM_PFNMAP VMAs which may not have a kernel direct mapping to a
// virtual address.
//
// With S2FWB and CACHE DIC features, KVM need not do cache flushing
// and CMOs are NOP'd. This has the effect of no longer requiring a
// KVA for addresses mapped into the S2. The presence of these features
// are thus necessary to support cacheable S2 mapping of VM_PFNMAP.
//

extern "C" {
    pub fn kvm_s2_ptdump_create_debugfs(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_nested_s2_ptdump_create_debugfs(mmu: *mut kvm_s2_mmu);
}
extern "C" {
    pub fn kvm_nested_s2_ptdump_remove_debugfs(mmu: *mut kvm_s2_mmu);
}

