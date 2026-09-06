//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/mmu/mmu_internal.h
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

// Page table builder macros common to shadow (host) PTEs and guest PTEs.

//
// Unlike regular MMU roots, PAE "roots", a.k.a. PDPTEs/PDPTRs, have a PRESENT
// bit, and thus are guaranteed to be non-zero when valid.  And, when a guest
// PDPTR is !PRESENT, its corresponding PAE root cannot be set to INVALID_PAGE,
// as the CPU would treat that as PRESENT PDPTR with reserved bits set.  Use
// '0' instead of INVALID_PAGE to indicate an invalid PAE root.
//
pub const INVALID_PAE_ROOT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mmu_page {
//
// Note, "link" through "spt" fit in a single 64 byte cache line on
// 64-bit kernels, keep it that way unless there's a reason not to.
//
    pub link: list_head,
    pub hash_link: hlist_node,
    pub tdp_mmu_page: bool,
    pub unsync: bool,
    pub mmu_valid_gen: u8,
// Only accessed under slots_lock.
    pub tdp_mmu_scheduled_root_to_zap: bool,
}

//
// The shadow page can't be replaced by an equivalent huge page
// because it is being used to map an executable page in the guest
// and the NX huge page mitigation is enabled.
//
// The following two entries are used to key the shadow page in the
// hash table.
//
// Stores the result of the guest translation being shadowed by each
// SPTE.  KVM shadows two types of guest translations: nGPA -> GPA
// (shadow EPT/NPT) and GVA -> GPA (traditional shadow paging). In both
// cases the result of the translation is a GPA and a set of access
// constraints.
//
// The GFN is stored in the upper bits (PAGE_SHIFT) and the shadowed
// access permissions are stored in the lower bits. Note, for
// convenience and uniformity across guests, the access permissions are
// stored in KVM format (e.g.  ACC_EXEC_MASK) not the raw guest format.
//
// Currently serving as active root
// These two members aren't used for TDP MMU
//
// Number of writes since the last time traversal
// visited this page.
//
// Page table page of external PT.
// Passed to TDX module, not accessed by KVM.
//
// Tracks shadow pages that, if zapped, would allow KVM to create an NX
// huge page.  A shadow page will have nx_huge_page_disallowed set but
// not be on the list if a huge page is disallowed for other reasons,
// e.g. because KVM is shadowing a PTE at the same gfn, the memslot
// isn't properly aligned, etc...
//

//
// Used out of the mmu-lock to avoid reading spte values while an
// update is in progress; see the comments in __get_spte_lockless().
//

// Used for freeing the page asynchronously if it is a TDP MMU page.

extern "C" {
    pub fn kvm_mmu_role_as_id(_arg: sp->role) -> return;
}
//
// external_spt is allocated for TDX module to hold private EPT mappings,
// TDX module will initialize the page by itself.
// Therefore, KVM does not need to initialize or access external_spt.
// KVM only interacts with sp->spt for private EPT operations.
//
// Since mirror SPs are used only for TDX, which maps private memory
// at its "natural" GFN, no mask needs to be applied to them - and, dually,
// we expect that the bits is only used for the shared PT.
//
extern "C" {
    pub fn kvm_gfn_direct_bits(_arg: kvm) -> return;
}
//
// When using the EPT page-modification log, the GPAs in the CPU dirty
// log would come from L2 rather than L1.  Therefore, we need to rely
// on write protection to record dirty pages, which bypasses PML, since
// writes now result in a vmexit.  Note, the check on CPU dirty logging
// being enabled is mandatory as the bits used to denote WP-only SPTEs
// are reserved for PAE paging (32-bit KVM).
//
extern "C" {
    pub fn kvm_mmu_gfn_disallow_lpage(slot: *const kvm_memory_slot, gfn: gfn_t);
}
extern "C" {
    pub fn kvm_mmu_gfn_allow_lpage(slot: *const kvm_memory_slot, gfn: gfn_t);
}
// Flush the given page (huge or not) of guest memory.
extern "C" {
    pub fn pte_list_count(rmap_head: *mut kvm_rmap_head) -> c_uint;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_page_fault {
// arguments to kvm_mmu_do_page_fault.
    pub addr: gpa_t,
    pub error_code: u64,
    pub prefetch: bool,
// Derived from error_code.
    pub exec: bool,
    pub write: bool,
    pub present: bool,
    pub rsvd: bool,
    pub user: bool,
// Derived from mmu and global state.
    pub is_tdp: bool,
    pub is_private: bool,
    pub nx_huge_page_workaround_enabled: bool,
//
// Whether a >4KB mapping can be created or is forbidden due to NX
// hugepages.
//
    pub huge_page_disallowed: bool,
//
// Maximum page size that can be created for this fault; input to
// FNAME(fetch), direct_map() and kvm_tdp_mmu_map().
//
    pub max_level: u8,
//
// Page size that can be created based on the max_level and the
// page size used by the host mapping.
//
    pub req_level: u8,
//
// Page size that will be created based on the req_level and
// huge_page_disallowed.
//
    pub goal_level: u8,
//
// Shifted addr, or result of guest page table walk if addr is a gva. In
// the case of VM where memslot's can be mapped at multiple GPA aliases
// (i.e. TDX), the gfn field does not contain the bit that selects between
// the aliases (i.e. the shared bit for TDX).
//
    pub gfn: gfn_t,
// The memslot containing gfn. May be NULL.
    pub slot: *mut kvm_memory_slot,
// Outputs of kvm_mmu_faultin_pfn().
    pub mmu_seq: c_ulong,
    pub pfn: kvm_pfn_t,
    pub refcounted_page: *mut page,
    pub map_writable: bool,
//
// Indicates the guest is trying to write a gfn that contains one or
// more of the PTEs used to translate the write itself, i.e. the access
// is changing its own translation in the guest page tables.
//
    pub write_fault_to_shadow_pgtable: bool,
}

//
// Return values of handle_mmio_page_fault(), mmu.page_fault(), fast_page_fault(),
// and of course kvm_mmu_do_page_fault().
//
// RET_PF_CONTINUE: So far, so good, keep handling the page fault.
// RET_PF_RETRY: let CPU fault again on the address.
// RET_PF_EMULATE: mmio page fault, emulate the instruction directly.
// RET_PF_WRITE_PROTECTED: the gfn is write-protected, either unprotected the
// gfn and retry, or emulate the instruction directly.
// RET_PF_INVALID: the spte is invalid, let the real page fault path update it.
// RET_PF_FIXED: The faulting entry has been fixed.
// RET_PF_SPURIOUS: The faulting entry was already fixed, e.g. by another vCPU.
//
// Any names added to this enum should be exported to userspace for use in
// tracepoints via TRACE_DEFINE_ENUM() in mmutrace.h
//
// Note, all values must be greater than or equal to zero so as not to encroach
// on -errno return values.
//
// Define RET_PF_CONTINUE as 0 to allow for
// - efficient machine code when checking for CONTINUE, e.g.
// "TEST %rax, %rax, JNZ", as all "stop!" values are non-zero,
// - kvm_mmu_do_page_fault() to return other RET_PF_* as a positive value.
//
extern "C" {
    pub fn kvm_mmu_hugepage_adjust(vcpu: *mut kvm_vcpu, fault: *mut kvm_page_fault);
}
extern "C" {
    pub fn disallowed_hugepage_adjust(fault: *mut kvm_page_fault, spte: u64, cur_level: c_int);
}
