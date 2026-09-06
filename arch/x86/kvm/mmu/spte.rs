//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/mmu/spte.h
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
// A MMU present SPTE is backed by actual memory and may or may not be present
// in hardware.  E.g. MMIO SPTEs are not considered present.  Use bit 11, as it
// is ignored by all flavors of SPTEs and checking a low bit often generates
// better code than for a high bit, e.g. 56+.  MMU present checks are pervasive
// enough that the improved code generation is noticeable in KVM's footprint.
//

//
// The ignored high bits are allocated as follows:
// - bits 52, 54: saved X-R bits for access tracking when EPT does not have A/D
// - bits 53 (EPT only): host writable
// - bits 55 (EPT only): MMU-writable
// - bits 56-59: unused
// - bits 60-61: type of A/D tracking
// - bits 62 (EPT only): saved XU bit for disabled AD
//
// TDP SPTES (more specifically, EPT SPTEs) may not have A/D bits, and may also
// be restricted to using write-protection (for L2 when CPU dirty logging, i.e.
// PML, is enabled).  Use bits 60 and 61 to hold the type of A/D tracking that
// is must be employed for a given TDP SPTE.
//
// Note, the "enabled" mask must be '0', as bits 62:52 are _reserved_ for PAE
// paging, including NPT PAE.  This scheme works because legacy shadow paging
// is guaranteed to have A/D bits and write-protection is forced only for
// TDP with CPU dirty logging (PML).  If NPT ever gains PML-like support, it
// must be restricted to 64-bit KVM.
//
pub const SPTE_TDP_AD_SHIFT: c_int = 60;

pub const SPTE_LEVEL_BITS: c_int = 9;

//
// The mask/shift to use for saving the original R/X bits when marking the PTE
// as not-present for access tracking purposes. We do not save the W bit as the
// PTEs being access tracked also need to be dirty tracked, so the W bit will be
// restored only when a write is attempted to the page.  This mask obviously
// must not overlap the A/D type mask.
//

pub const SHADOW_ACC_TRACK_SAVED_BITS_SHIFT: c_int = 52;

//
// {DEFAULT,EPT}_SPTE_{HOST,MMU}_WRITABLE are used to keep track of why a given
// SPTE is write-protected. See is_writable_pte() for details.
//
// Bits 9 and 10 are ignored by all non-EPT PTEs.

//
// Low ignored bits are at a premium for EPT, use high ignored bits, taking care
// to not overlap the A/D type mask or the saved access bits of access-tracked
// SPTEs when A/D bits are disabled.
//

// Defined only to keep the above static asserts readable.

//
// Due to limited space in PTEs, the MMIO generation is an 18 bit subset of
// the memslots generation and is derived as follows:
//
// Bits 0-6 of the MMIO generation are propagated to spte bits 3-9
// Bits 7-17 of the MMIO generation are propagated to spte bits 52-62
//
// The KVM_MEMSLOT_GEN_UPDATE_IN_PROGRESS flag is intentionally not included in
// the MMIO generation number, as doing so would require stealing a bit from
// the "real" generation number and thus effectively halve the maximum number
// of MMIO generations that can be handled before encountering a wrap (which
// requires a full MMU zap).  The flag is instead explicitly queried when
// checking for MMIO spte cache hits.
//
pub const MMIO_SPTE_GEN_LOW_START: c_int = 3;
pub const MMIO_SPTE_GEN_LOW_END: c_int = 9;
pub const MMIO_SPTE_GEN_HIGH_START: c_int = 52;
pub const MMIO_SPTE_GEN_HIGH_END: c_int = 62;

//
// The SPTE MMIO mask must NOT overlap the MMIO generation bits or the
// MMU-present bit.  The generation obviously co-exists with the magic MMIO
// mask/value, and MMIO SPTEs are considered !MMU-present.
//
// The SPTE MMIO mask is allowed to use hardware "present" bits (i.e. all EPT
// RWX bits), all physical address bits (legal PA bits are used for "fast" MMIO
// and so they're off-limits for generation; additional checks ensure the mask
// doesn't overlap legal PA bits), and bit 63 (carved out for future usage).
//

// remember to adjust the comment above as well if you change these

//
// Non-present SPTE value needs to set bit 63 for TDX, in order to suppress
// #VE and get EPT violations on non-present PTEs.  We can use the
// same value also without TDX for both VMX and SVM:
//
// For SVM NPT, for non-present spte (bit 0 = 0), other bits are ignored.
// For VMX EPT, bit 63 is ignored if #VE is disabled. (EPT_VIOLATION_VE=0)
// bit 63 is #VE suppress if #VE is enabled. (EPT_VIOLATION_VE=1)
//

//
// True if A/D bits are supported in hardware and are enabled by KVM.  When
// enabled, KVM uses A/D bits for all non-nested MMUs.  Because L1 can disable
// A/D bits in EPTP12, SP and SPTE variants are needed to handle the scenario
// where KVM is using A/D bits for L1, but not L2.
//
// SPTEs in MMUs without A/D bits are marked with SPTE_TDP_AD_DISABLED;
// shadow_acc_track_mask is the set of bits to be cleared in non-accessed
// pages.
//
// This mask must be set on all non-zero Non-Present or Reserved SPTEs in order
// to guard against L1TF attacks.
//
// The number of high-order 1 bits to use in the mask above.
//
pub const SHADOW_NONPRESENT_OR_RSVD_MASK_LEN: c_int = 5;
//
// If a thread running without exclusive control of the MMU lock must perform a
// multi-part operation on an SPTE, it can set the SPTE to FROZEN_SPTE as a
// non-present intermediate value. Other threads which encounter this value
// should not modify the SPTE.
//
// Use a semi-arbitrary value that doesn't set RWX bits, i.e. is not-present on
// both AMD and Intel CPUs, and doesn't set PFN bits, i.e. doesn't create a L1TF
// vulnerability.
//
// Only used by the TDP MMU.
//

// Frozen SPTEs must not be misconstrued as shadow or MMU present PTEs.
// Get an SPTE's index into its parent's page table (and the spt array).
//
// In some cases, we need to preserve the GFN of a non-present or reserved
// SPTE when we usurp the upper five bits of the physical address space to
// defend against L1TF, e.g. for MMIO SPTEs.  To preserve the GFN, we'll
// shift bits of the GFN that overlap with shadow_nonpresent_or_rsvd_mask
// left into the reserved bits, i.e. the GFN in the SPTE will be split into
// high and low parts.  This mask covers the lower bits of the GFN.
//
extern "C" {
    pub fn is_zero_pfn(PAGE_SHIFT: shadow_page >>) -> return;
}
extern "C" {
    pub fn to_shadow_page(SPTE_BASE_ADDR_MASK: spte &) -> return;
}
extern "C" {
    pub fn to_shadow_page(_arg: __pa(sptep)) -> return;
}
//
// The "root" may be a special root, e.g. a PAE entry, treat it as a
// SPTE to ensure any non-PA bits are dropped.
//
extern "C" {
    pub fn spte_to_child_sp(_arg: root) -> return;
}
extern "C" {
    pub fn is_mirror_sp(_arg: sptep_to_sp(rcu_dereference(sptep))) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: root->has_mapped_host_mmio) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: vcpu->kvm->arch.has_mapped_host_mmio) -> return;
}
//
// This is benign for non-TDP SPTEs as SPTE_TDP_AD_ENABLED is '0',
// and non-TDP SPTEs will never set these bits.  Optimize for 64-bit
// TDP and do the A/D type check unconditionally.
//
// A shadow-present leaf SPTE may be non-writable for 4 possible reasons:
//
// 1. To intercept writes for dirty logging. KVM write-protects huge pages
// so that they can be split down into the dirty logging
// granularity (4KiB) whenever the guest writes to them. KVM also
// write-protects 4KiB pages so that writes can be recorded in the dirty log
// (e.g. if not using PML). SPTEs are write-protected for dirty logging
// during the VM-iotcls that enable dirty logging.
//
// 2. To intercept writes to guest page tables that KVM is shadowing. When a
// guest writes to its page table the corresponding shadow page table will
// be marked "unsync". That way KVM knows which shadow page tables need to
// be updated on the next TLB flush, INVLPG, etc. and which do not.
//
// 3. To prevent guest writes to read-only memory, such as for memory in a
// read-only memslot or guest memory backed by a read-only VMA. Writes to
// such pages are disallowed entirely.
//
// 4. To emulate the Accessed bit for SPTEs without A/D bits.  Note, in this
// case, the SPTE is access-protected, not just write-protected!
//
// For cases #1 and #4, KVM can safely make such SPTEs writable without taking
// mmu_lock as capturing the Accessed/Dirty state doesn't require taking it.
// To differentiate #1 and #4 from #2 and #3, KVM uses two software-only bits
// in the SPTE:
//
// shadow_mmu_writable_mask, aka MMU-writable -
// Cleared on SPTEs that KVM is currently write-protecting for shadow paging
// purposes (case 2 above).
//
// shadow_host_writable_mask, aka Host-writable -
// Cleared on SPTEs that are not host-writable (case 3 above)
//
// Note, not all possible combinations of PT_WRITABLE_MASK,
// shadow_mmu_writable_mask, and shadow_host_writable_mask are valid. A given
// SPTE can be in only one of the following states, which map to the
// aforementioned 3 cases:
//
// shadow_host_writable_mask | shadow_mmu_writable_mask | PT_WRITABLE_MASK
// ------------------------- | ------------------------ | ----------------
// 1                         | 1                        | 1       (writable)
// 1                         | 1                        | 0       (case 1)
// 1                         | 0                        | 0       (case 2)
// 0                         | 0                        | 0       (case 3)
//
// The valid combinations of these bits are checked by
// check_spte_writable_invariants() whenever an SPTE is modified.
//
// Clearing the MMU-writable bit is always done under the MMU lock and always
// accompanied by a TLB flush before dropping the lock to avoid corrupting the
// shadow page tables between vCPUs. Write-protecting an SPTE for dirty logging
// (which does not clear the MMU-writable bit), does not flush TLBs before
// dropping the lock, as it only needs to synchronize guest writes with the
// dirty bitmap. Similarly, making the SPTE inaccessible (and non-writable) for
// access-tracking via the clear_young() MMU notifier also does not flush TLBs.
//
// So, there is the problem: clearing the MMU-writable bit can encounter a
// write-protected SPTE while CPUs still have writable mappings for that SPTE
// cached in their TLB. To address this, KVM always flushes TLBs when
// write-protecting SPTEs if the MMU-writable bit is set on the old SPTE.
//
// The Host-writable bit is not modified on present SPTEs, it is only set or
// cleared when an SPTE is first faulted in from non-present and then remains
// immutable.
//
// Note: spte must be a shadow-present leaf SPTE.
//
// Returns true if the access indicated by @fault is forbidden by the existing
// SPTE protections.
//
// strip nested paging fault error codes
//
// RSVD is handled elsewhere, and is used for SMAP in the context
// of accessing fmt.permissions[].  SPTEs never use PK or SS, as
// they are not supported for shadow paging and irrelevant for TDP.
//
// If the MMU-writable flag is cleared, i.e. the SPTE is write-protected for
// write-tracking, remote TLBs must be flushed, even if the SPTE was read-only,
// as KVM allows stale Writable TLB entries to exist.  When dirty logging, KVM
// flushes TLBs based on whether or not dirty bitmap/ring entries were reaped,
// not whether or not SPTEs were modified, i.e. only the write-tracking case
// needs to flush at the time the SPTEs is modified, before dropping mmu_lock.
//
// Don't flush if the Accessed bit is cleared, as access tracking tolerates
// false negatives, e.g. KVM x86 omits TLB flushes even when aging SPTEs for a
// mmu_notifier.clear_flush_young() event.
//
// Lastly, don't flush if the Dirty bit is cleared, as KVM unconditionally
// flushes when enabling dirty logging (see kvm_mmu_slot_apply_flags()), and
// when clearing dirty logs, KVM flushes based on whether or not dirty entries
// were reaped from the bitmap/ring, not whether or not dirty SPTEs were found.
//
// Note, this logic only applies to shadow-present leaf SPTEs.  The caller is
// responsible for checking that the old SPTE is shadow-present, and is also
// responsible for determining whether or not a TLB flush is required when
// modifying a shadow-present non-leaf SPTE.
//
extern "C" {
    pub fn is_mmu_writable_spte(!is_mmu_writable_spte(new_spte: old_spte) &&) -> return;
}
extern "C" {
    pub fn spte_needs_atomic_update(spte: u64) -> bool;
}
extern "C" {
    pub fn make_huge_spte(kvm: *mut kvm, small_spte: u64, level: c_int) -> u64;
}
extern "C" {
    pub fn make_nonleaf_spte(child_pt: *mut u64, ad_disabled: bool) -> u64;
}
extern "C" {
    pub fn make_mmio_spte(vcpu: *mut kvm_vcpu, gfn: u64, access: c_uint) -> u64;
}
extern "C" {
    pub fn mark_spte_for_access_track(spte: u64) -> u64;
}
// Restore an acc-track PTE back to a regular PTE
extern "C" {
    pub fn kvm_mmu_spte_module_init() -> void __init;
}
extern "C" {
    pub fn kvm_mmu_reset_all_pte_masks();
}
