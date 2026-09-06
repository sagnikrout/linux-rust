//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/mmu.h
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


// SPDX-License-Identifier: GPL-2.0

pub const KVM_MEMSLOT_PAGES_TO_MMU_PAGES_RATIO: c_int = 50;

pub const KVM_MMU_HASH_SHIFT: c_int = 12;

pub const KVM_MIN_FREE_MMU_PAGES: c_int = 5;
pub const KVM_REFILL_PAGES: c_int = 25;
pub const PT_WRITABLE_SHIFT: c_int = 1;
pub const PT_USER_SHIFT: c_int = 2;

pub const PT_ACCESSED_SHIFT: c_int = 5;

pub const PT_DIRTY_SHIFT: c_int = 6;

pub const PT_PAGE_SIZE_SHIFT: c_int = 7;

pub const PT64_NX_SHIFT: c_int = 63;

pub const PT_PAT_SHIFT: c_int = 7;
pub const PT_DIR_PAT_SHIFT: c_int = 12;

pub const PT64_ROOT_5LEVEL: c_int = 5;
pub const PT64_ROOT_4LEVEL: c_int = 4;
pub const PT32_ROOT_LEVEL: c_int = 2;
pub const PT32E_ROOT_LEVEL: c_int = 3;

pub const ACC_EXEC_MASK: c_int = 8;

//
// Note that this uses the host MAXPHYADDR, not the guest's.
// EPT/NPT cannot support GPAs that would exceed host.MAXPHYADDR;
// assuming KVM is running on bare metal, guest accesses beyond
// host.MAXPHYADDR will hit a #PF(RSVD) and never cause a vmexit
// (either EPT Violation/Misconfig or #NPF), and so KVM will never
// install a SPTE for such addresses.  If KVM is running as a VM
// itself, on the other hand, it might see a MAXPHYADDR that is less
// than hardware's real MAXPHYADDR.  Using the host MAXPHYADDR
// disallows such SPTEs entirely and simplifies the TDP MMU.
//
extern "C" {
    pub fn kvm_mmu_get_max_tdp_level() -> u8;
}
extern "C" {
    pub fn kvm_mmu_x86_module_init() -> void __init;
}
extern "C" {
    pub fn kvm_mmu_vendor_module_init() -> c_int;
}
extern "C" {
    pub fn kvm_mmu_vendor_module_exit();
}
extern "C" {
    pub fn kvm_mmu_destroy(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_mmu_create(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_mmu_init_vm(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_mmu_uninit_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_mmu_after_set_cpuid(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_mmu_reset_context(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_mmu_invalidate_mmio_sptes(kvm: *mut kvm, gen: u64);
}
extern "C" {
    pub fn kvm_mmu_change_mmu_pages(kvm: *mut kvm, kvm_nr_mmu_pages: c_ulong);
}
extern "C" {
    pub fn kvm_zap_gfn_range(kvm: *mut kvm, gfn_start: gfn_t, gfn_end: gfn_t);
}
extern "C" {
    pub fn kvm_mmu_set_mmio_spte_mask(mmio_value: u64, mmio_mask: u64, access_mask: u64);
}
extern "C" {
    pub fn kvm_mmu_set_mmio_spte_value(kvm: *mut kvm, mmio_value: u64);
}
extern "C" {
    pub fn kvm_mmu_set_me_spte_mask(me_value: u64, me_mask: u64);
}
extern "C" {
    pub fn kvm_mmu_set_ept_masks(has_ad_bits: bool);
}
extern "C" {
    pub fn kvm_init_mmu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_mmu_print_sptes(vcpu: *mut kvm_vcpu, gpa: gpa_t, msg: *const c_char);
}
extern "C" {
    pub fn kvm_mmu_invlpg(vcpu: *mut kvm_vcpu, gva: gva_t);
}
extern "C" {
    pub fn kvm_mmu_invpcid_gva(vcpu: *mut kvm_vcpu, gva: gva_t, pcid: c_ulong);
}
extern "C" {
    pub fn kvm_mmu_new_pgd(vcpu: *mut kvm_vcpu, new_pgd: gpa_t);
}
extern "C" {
    pub fn kvm_can_do_async_pf(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_mmu_load(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_mmu_unload(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_mmu_free_obsolete_roots(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_mmu_sync_roots(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_mmu_sync_prev_roots(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn __kvm_mmu_unprotect_gfn_and_retry(_arg: vcpu, _arg: cr2_or_gpa, _arg: false) -> return;
}
extern "C" {
    pub fn kvm_mmu_free_guest_mode_roots(kvm: *mut kvm, mmu: *mut kvm_mmu);
}
//
// Checking root.hpa is sufficient even when KVM has mirror root.
// We can have either:
// (1) mirror_root_hpa = INVALID_PAGE, root.hpa = INVALID_PAGE
// (2) mirror_root_hpa = root,         root.hpa = INVALID_PAGE
// (3) mirror_root_hpa = root1,        root.hpa = root2
// We don't ever have:
// mirror_root_hpa = INVALID_PAGE, root.hpa = root
//
extern "C" {
    pub fn kvm_mmu_load(_arg: vcpu) -> return;
}
extern "C" {
    pub fn kvm_get_pcid(_arg: vcpu, _arg: kvm_read_cr3(vcpu)) -> return;
}
extern "C" {
    pub fn kvm_read_cr3(X86_CR3_LAM_U57: vcpu) & (X86_CR3_LAM_U48 |) -> return;
}
//
// When EPT is enabled, KVM may passthrough CR0.WP to the guest, i.e.
// @w's snapshot of CR0.WP and thus all related paging metadata may
// be stale.  Refresh CR0.WP and the metadata on-demand when checking
// for permission faults.  Exempt nested MMUs, i.e. MMUs for shadowing
// nEPT and nNPT, as CR0.WP is ignored in both cases.  Note, KVM will
// still refresh gva_walk, so as to honor L2's CR0.WP when translating
// L2 GVAs to GPAs.
//
// Check if a given access (described through the I/D, W/R and U/S bits of a
// page fault error code pfec) causes a permission fault with the given PTE
// access rights (in ACC_* format).
//
// Return zero if the access does not fault; return the page fault error code
// if the access faults.
//
// strip nested paging fault error codes
//
// For explicit supervisor accesses, SMAP is disabled if EFLAGS.AC = 1.
// For implicit supervisor accesses, SMAP cannot be overridden.
//
// SMAP works on supervisor accesses only, and not_smap can
// be set or not set when user access with neither has any bearing
// on the result.
//
// We put the SMAP checking bit in place of the PFERR_RSVD_MASK bit;
// this bit will always be zero in pfec, but it will be one in index
// if SMAP checks are being disabled.
//
// PKRU defines 32 bits, there are 16 domains and 2
// attribute bits per domain in pkru.  pte_pkey is the
// index of the protection domain, so pte_pkey * 2 is
// is the index of the first bit for the domain.
//
// clear present bit, replace PFEC.RSVD with ACC_USER_MASK.
extern "C" {
    pub fn kvm_mmu_post_init_vm(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_mmu_pre_destroy_vm(kvm: *mut kvm);
}
//
// Read shadow_root_allocated before related pointers. Hence, threads
// reading shadow_root_allocated in any lock context are guaranteed to
// see the pointers. Pairs with smp_store_release in
// mmu_first_shadow_root_alloc.
//
extern "C" {
    pub fn smp_load_acquire(_arg: &kvm->arch.shadow_root_allocated) -> return;
}
extern "C" {
    pub fn kvm_tdp_mmu_map_private_pfn(vcpu: *mut kvm_vcpu, gfn: gfn_t, pfn: kvm_pfn_t) -> c_int;
}
// KVM_HPAGE_GFN_SHIFT(PG_LEVEL_4K) must be 0.
extern "C" {
    pub fn __kvm_mmu_slot_lpages(_arg: slot, _arg: slot->npages, _arg: level) -> return;
}
